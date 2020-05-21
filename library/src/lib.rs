mod atkacpi;
mod device_control;
mod err;

pub use {atkacpi::*, device_control::*, err::*};

/*
#[test]
fn set_to_silent() {
    let mut atk = AtkAcpi::new().unwrap();
    atk.set_power_plan(PowerPlan::TurboManual).unwrap();
    let cpu_table: FanCurveTable = [
        0x1e, 0x2c, 0x32, 0x45, 0x4e, 0x59, 0x63, 0x64, 0x00, 0x00, 0x00, 0x00, 0x20, 0x32, 0x39,
        0x39,
    ]
    .into();
    let gpu_table: FanCurveTable = [
        0x1e, 0x2c, 0x32, 0x3d, 0x46, 0x50, 0x5a, 0x64, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1c, 0x22,
        0x28,
    ]
    .into();
    atk.set_fan_curve(FanCurveDevice::Cpu, &cpu_table).unwrap();
    atk.set_fan_curve(FanCurveDevice::Gpu, &gpu_table).unwrap();
}

#[test]
fn set_to_real_silent() {
    let mut atk = AtkAcpi::new().unwrap();
    atk.set_power_plan(PowerPlan::Silent).unwrap();
}

#[test]
fn set_to_real_silent_no_fan() {
    let mut atk = AtkAcpi::new().unwrap();
    atk.set_power_plan(PowerPlan::Silent).unwrap();
    let cpu_table: FanCurveTable = [
        0x1e, 0x2c, 0x32, 0x45, 0x4e, 0x59, 0x63, 0x64, 0x00, 0x00, 0x00, 0x00, 0x20, 0x32, 0x39,
        0x39,
    ]
        .into();
    let gpu_table: FanCurveTable = [
        0x1e, 0x2c, 0x32, 0x3d, 0x46, 0x50, 0x5a, 0x64, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1c, 0x22,
        0x28,
    ]
        .into();
    atk.set_fan_curve(FanCurveDevice::Cpu, &cpu_table).unwrap();
    atk.set_fan_curve(FanCurveDevice::Gpu, &gpu_table).unwrap();
}

#[test]
fn set_fan_only() {
    let mut atk = AtkAcpi::new().unwrap();
    let cpu_table: FanCurveTable = [
        0x1e, 0x2c, 0x32, 0x45, 0x4e, 0x59, 0x63, 0x64, 0x00, 0x00, 0x00, 0x00, 0x20, 0x32, 0x39,
        0x39,
    ]
        .into();
    let gpu_table: FanCurveTable = [
        0x1e, 0x2c, 0x32, 0x3d, 0x46, 0x50, 0x5a, 0x64, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1c, 0x22,
        0x28,
    ]
        .into();
    atk.set_fan_curve(FanCurveDevice::Cpu, &cpu_table).unwrap();
    atk.set_fan_curve(FanCurveDevice::Gpu, &gpu_table).unwrap();
}

fn main() {
    println!("Hello, world!");
}
*/
