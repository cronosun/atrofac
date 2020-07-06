use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Configuration {
    pub active_plan: Option<PlanName>,
    pub plans: Vec<Plan>,
    /// default value is "false".
    pub disable_logging: Option<bool>,
    /// log specification. Default value is "info" (other meaningful values are "debug").
    pub log_spec: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            active_plan: None,
            plans: Default::default(),
            disable_logging: None,
            log_spec: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PlanName(Rc<str>);

impl PlanName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    pub name: PlanName,
    pub plan: PowerPlan,
    pub refresh_interval_sec: Option<u32>,
    pub refresh_on_apm_resume_automatic: Option<bool>,
    pub cpu_curve: Option<String>,
    pub gpu_curve: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerPlan {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "silent")]
    Silent,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "turbo")]
    Turbo,
}

impl Into<atrofac_library::PowerPlan> for PowerPlan {
    fn into(self) -> atrofac_library::PowerPlan {
        match self {
            PowerPlan::Windows => atrofac_library::PowerPlan::PerformanceWindows,
            PowerPlan::Silent => atrofac_library::PowerPlan::Silent,
            PowerPlan::Performance => atrofac_library::PowerPlan::PerformanceWindows,
            PowerPlan::Turbo => atrofac_library::PowerPlan::TurboManual,
        }
    }
}
