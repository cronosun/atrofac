mod opts;

use crate::opts::Opts;
use atrofac_library::{AfErr, AtkAcpi, FanCurveDevice, FanCurveTable, FanCurveTableBuilder};
use env_logger::Env;
use log::info;
use log::warn;
use structopt::StructOpt;

fn convert_to_curve(device: FanCurveDevice, string: &str) -> Result<FanCurveTable, AfErr> {
    if string.trim().len() == 0 {
        // the minimum table
        Ok(FanCurveTableBuilder::from_string(
            device,
            "0c:0%,0c:0%,0c:0%,0c:0%,0c:0%,0c:0%,0c:0%,0c:0%",
        )
        .unwrap()
        .auto_fix_build())
    } else {
        let builder_from_string = FanCurveTableBuilder::from_string(device, string)?;
        let is_valid = builder_from_string.is_valid();
        let table = builder_from_string.auto_fix_build();
        if !is_valid {
            warn!("Fan curve for {:?} might damage your device and has been auto-adjusted to the minimum safe values: {}.", device, table.to_string());
        }
        Ok(table)
    }
}

fn perform(opt: Opts) -> Result<(), AfErr> {
    match opt {
        Opts::Plan(power_plan) => {
            let mut atk = AtkAcpi::new()?;
            atk.set_power_plan(power_plan.into())?;
            Ok(())
        }
        Opts::Fan(fan) => {
            let cpu = convert_to_curve(FanCurveDevice::Cpu, &fan.cpu)?;
            let gpu = convert_to_curve(FanCurveDevice::Gpu, &fan.gpu)?;
            let mut atk = AtkAcpi::new()?;
            atk.set_power_plan(fan.plan.into())?;
            atk.set_fan_curve(&cpu)?;
            atk.set_fan_curve(&gpu)?;
            Ok(())
        }
    }
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let opt = Opts::from_args();
    if let Err(error) = perform(opt) {
        warn!("Unable to perform operation: {:?}", error);
        std::process::exit(exitcode::CONFIG);
    } else {
        info!("Success.");
        std::process::exit(exitcode::OK);
    }
}
