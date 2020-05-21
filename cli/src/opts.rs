use structopt::clap::arg_enum;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
#[structopt(rename_all = "kebab-case")]
pub enum Opts {
    /// Sets the power plan and uses the default fan curve.
    #[structopt(aliases = &["plan", "set-plan"])]
    Plan(PowerPlan),
    /// Sets a custom fan curve with a power plan.
    #[structopt(aliases = &["fan", "set-fan"])]
    Fan(Fan),
}

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum PowerPlan {
    /// Windows power plan.
    #[structopt(aliases = &["win", "windows"])]
    Windows,
    /// Silent power plan (with default fan curve).
    #[structopt(aliases = &["silent"])]
    Silent,
    /// Performance power plan.
    #[structopt(aliases = &["performance"])]
    Performance,
    /// Turbo power plan.
    #[structopt(aliases = &["turbo"])]
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

arg_enum! {
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum PowerPlan2 {
    windows,
    silent,
    performance,
    turbo
}
}

impl Into<atrofac_library::PowerPlan> for PowerPlan2 {
    fn into(self) -> atrofac_library::PowerPlan {
        match self {
            PowerPlan2::windows => atrofac_library::PowerPlan::PerformanceWindows,
            PowerPlan2::silent => atrofac_library::PowerPlan::Silent,
            PowerPlan2::performance => atrofac_library::PowerPlan::PerformanceWindows,
            PowerPlan2::turbo => atrofac_library::PowerPlan::TurboManual,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub struct Fan {
    /// The power plan to set with the custom fan curve.
    #[structopt(possible_values = &PowerPlan2::variants(), long = "plan", default_value = "silent")]
    pub plan: PowerPlan2,
    /// The CPU fan curve. If not given, will set the fan speed to minimum. Must be a list of
    /// comma separated entries (8 entries). Each entry looks like this:
    /// <DEGREES>c:<PERCENTAGE>%. Example: 0c:0%,40c:0%,50c:0%,60c:0%,70c:34%,80c:51%,90c:61%,100c:61%.
    #[structopt(long = "cpu", default_value = "")]
    pub cpu: String,
    /// The GPU fan curve. If not given, will set the fan speed to minimum. Must be a list of
    /// comma separated entries (8 entries). Each entry looks like this:
    /// <DEGREES>c:<PERCENTAGE>%. Example: 0c:0%,40c:0%,50c:0%,60c:0%,70c:34%,80c:51%,90c:61%,100c:61%.
    #[structopt(long = "gpu", default_value = "")]
    pub gpu: String,
}
