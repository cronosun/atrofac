use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use dirs::{config_dir, data_local_dir};
use indexmap::map::IndexMap;
use log::info;
use log::warn;

use atrofac_library::{AfErr, AtkAcpi, FanCurveDevice, FanCurveTable, FanCurveTableBuilder};

use crate::engine::configuration::{Configuration, Plan, PlanName};
use flexi_logger::{Age, Cleanup, Criterion, Logger, Naming, detailed_format};
use std::fs;

const DEFAULT_LOG_LEVEL: &str = "info";
const CONFIG_FILE_NAME: &str = "atrofac_gui_config.yaml";
const ATROFAC_LOG_DIR: &str = "atrofac_logs";

pub struct Engine {
    configuration: Configuration,
    plans: IndexMap<PlanName, Plan>,
    config_file: PathBuf,
}

impl Engine {
    pub fn new() -> Result<Self, AfErr> {
        let config_file = Self::require_config_file_path_buf()?;
        let engine = Self {
            configuration: Default::default(),
            plans: Default::default(),
            config_file,
        };
        engine.start_logging()?;
        Ok(engine)
    }

    pub fn load_configuration(&mut self) -> Result<(), AfErr> {
        let path_buf = self.config_file();
        if !path_buf.exists() {
            let configuration = serde_yaml::from_str(include_str!("default_config.yaml"))
                .map_err(|err| AfErr::from(format!("Configuration file is invalid: {}.", err)))?;
            self.set_configuration(configuration);
            // save the configuration right now (so the user has a template to edit).
            self.save_configuration()?;
            info!("Created a new configuration file (since there was none).");
            Ok(())
        } else {
            let file = File::open(path_buf.as_path()).map_err(|err| {
                AfErr::from(format!(
                    "Unable to open config file {}: {}.",
                    path_buf.as_path().display(),
                    err
                ))
            })?;
            let buffered_reader = BufReader::new(file);
            let configuration = serde_yaml::from_reader(buffered_reader)
                .map_err(|err| AfErr::from(format!("Configuration file is invalid: {}.", err)))?;
            self.set_configuration(configuration);
            info!("Configuration file successfully (re-)loaded.");
            Ok(())
        }
    }

    pub fn save_configuration(&self) -> Result<(), AfErr> {
        let path_buf = self.config_file();
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path_buf.as_path()).map_err(|err| {
            AfErr::from(format!("Unable to open config file {} for writing; are you currently editing the file?: {}.", path_buf.as_path().display(), err))
        })?;
        {
            let mut buffered_writer = BufWriter::new(&mut file);
            serde_yaml::to_writer(&mut buffered_writer, &self.configuration).map_err(|err| {
                AfErr::from(format!(
                    "Unable to serialize configuration (save configuration): {}.",
                    err
                ))
            })?;
            buffered_writer
                .flush()
                .map_err(|err| AfErr::from(format!("Unable to flush buffer: {}", err)))?;
        }
        file.flush()
            .map_err(|err| AfErr::from(format!("Unable to flush file: {}", err)))?;
        info!("Configuration file successfully saved (modified configuration file).");
        Ok(())
    }

    pub fn active_plan(&self) -> Option<&Plan> {
        if let Some(active_plan) = &self.configuration.active_plan {
            self.plans.get(active_plan)
        } else {
            None
        }
    }

    pub fn set_active_plan(&mut self, plan: PlanName) {
        self.configuration.active_plan = Some(plan);
    }

    pub fn available_plans<'a>(&'a self) -> impl Iterator<Item = (usize, PlanName)> + 'a {
        self.configuration
            .plans
            .iter()
            .enumerate()
            .map(|(index, plan)| (index, plan.name.clone()))
    }

    pub fn plan_by_index(&self, index: usize) -> Option<&PlanName> {
        self.plans.get_index(index).map(|(key, _)| key)
    }

    pub fn number_of_plans(&self) -> usize {
        self.configuration.plans.len()
    }

    pub fn apply(&mut self) -> Result<ApplyInfo, AfErr> {
        if let Some(active_plan) = self.active_plan() {
            let mut atk = AtkAcpi::new()?;
            if let (Some(cpu), Some(gpu)) = (&active_plan.cpu_curve, &active_plan.gpu_curve) {
                // plan and fan curve
                let cpu = convert_to_curve(FanCurveDevice::Cpu, cpu)?;
                let gpu = convert_to_curve(FanCurveDevice::Gpu, gpu)?;
                atk.set_power_plan(active_plan.plan.into())?;
                atk.set_fan_curve(&cpu)?;
                atk.set_fan_curve(&gpu)?;
                info!(
                    "Power plan updated with custom fan curve: {:?}; CPU {}; GPU {}.",
                    active_plan.plan,
                    cpu.to_string(),
                    gpu.to_string(),
                );
            } else {
                // plan only
                atk.set_power_plan(active_plan.plan.into())?;
                info!(
                    "Power plan updated (no custom fan curve): {:?}.",
                    active_plan.plan
                );
            }
            Ok(ApplyInfo::Ok)
        } else {
            warn!("No active plan found (cannot apply plan).");
            Ok(ApplyInfo::NoPlan)
        }
    }

    pub fn config_file(&self) -> &PathBuf {
        &self.config_file
    }

    fn require_config_file_path_buf() -> Result<PathBuf, AfErr> {
        if let Some(config_file) = Self::config_file_path_buf() {
            Ok(config_file)
        } else {
            Err(AfErr::from(
                "Unable to determine configuration directory for current user.",
            ))
        }
    }

    fn config_file_path_buf() -> Option<PathBuf> {
        if let Some(mut config_dir) = config_dir() {
            config_dir.push(CONFIG_FILE_NAME);
            Some(config_dir)
        } else {
            None
        }
    }

    fn set_configuration(&mut self, configuration: Configuration) {
        self.configuration = configuration;
        self.update_plan_map();
    }

    fn update_plan_map(&mut self) {
        self.plans.clear();
        for plan in &self.configuration.plans {
            self.plans.insert(plan.name.clone(), plan.clone());
        }
    }

    fn start_logging(&self) -> Result<(), AfErr> {
        let disable_logging = self.configuration.disable_logging.unwrap_or(false);
        if !disable_logging {
            let log_spec: &str = if let Some(log_spec) = &self.configuration.log_spec {
                &log_spec
            } else {
                DEFAULT_LOG_LEVEL
            };
            Logger::with_env_or_str(log_spec)
                .log_to_file()
                .directory(self.log_directory()?)
                .rotate(
                    Criterion::Age(Age::Day),
                    Naming::Timestamps,
                    Cleanup::KeepLogFiles(7),
                )
                // no background thread. This would just waste resources; for atrofac it's no problem if cleanup blocks.
                .cleanup_in_background_thread(false)
                .format(detailed_format)
                .start()
                .map_err(|log_err| AfErr::from(format!("Unable to start logger: {}", log_err)))?;
            info!("atrofac started.");
            Ok(())
        } else {
            Ok(())
        }
    }

    fn log_directory(&self) -> Result<PathBuf, AfErr> {
        let mut log_dir = data_local_dir().ok_or_else(|| {
            AfErr::from("Unable to determine data local dir (used to store log files).")
        })?;
        log_dir.push(ATROFAC_LOG_DIR);
        fs::create_dir_all(&log_dir).map_err(|error| {
            AfErr::from(format!(
                "Unable to create log directory '{:?}'; \
            error: {}.",
                &log_dir, error
            ))
        })?;
        Ok(log_dir)
    }
}

pub enum ApplyInfo {
    Ok,
    NoPlan,
}

fn convert_to_curve(device: FanCurveDevice, string: &str) -> Result<FanCurveTable, AfErr> {
    let builder_from_string = FanCurveTableBuilder::from_string(device, string)?;
    let is_valid = builder_from_string.is_valid();
    let table = builder_from_string.auto_fix_build();
    if !is_valid {
        warn!(
            "Fan curve for {:?} might damage your device and has been auto-adjusted to \
        the minimum safe values: {}.",
            device,
            table.to_string()
        );
    }
    Ok(table)
}
