use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Configuration {
    pub active_plan: Option<PlanName>,
    pub plans: Vec<Plan>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            active_plan: None,
            plans: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlanName(Rc<String>);

impl PlanName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Serialize for PlanName {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for PlanName {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(Self(Rc::new(string)))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    pub name: PlanName,
    pub plan: PowerPlan,
    pub refresh_interval_sec: Option<u32>,
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
