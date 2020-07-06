use crate::{AfErr, DeviceControl};
use regex::Regex;
use std::convert::TryFrom;
use std::ops::Index;

const FILE_NAME: &'static str = "\\\\.\\ATKACPI";
const CONTROL_CODE: u32 = 2237452;
const POWER_PLAN_TEMPLATE: [u8; 16] = [
    0x44, 0x45, 0x56, 0x53, 0x08, 0x00, 0x00, 0x00, 0x75, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const POWER_PLAN_INDEX: usize = 12;
const SET_FAN_CURVE_TEMPLATE: [u8; 28] = [
    0x44, 0x45, 0x56, 0x53, 0x14, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x11, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];
const SET_FAN_CURVE_DEVICE_INDEX: usize = 8;

pub struct AtkAcpi {
    device_control: DeviceControl,
}

impl AtkAcpi {
    pub fn new() -> Result<Self, AfErr> {
        Ok(Self {
            device_control: DeviceControl::new(FILE_NAME)?,
        })
    }

    pub fn set_power_plan(&mut self, power_plan: PowerPlan) -> Result<(), AfErr> {
        let mut in_buffer: [u8; 16] = POWER_PLAN_TEMPLATE;
        in_buffer[POWER_PLAN_INDEX] = power_plan.to_byte();
        self.control(&mut in_buffer)
    }

    pub fn set_fan_curve(&mut self, table: &FanCurveTable) -> Result<(), AfErr> {
        let mut in_buffer: [u8; 28] = SET_FAN_CURVE_TEMPLATE;
        in_buffer[SET_FAN_CURVE_DEVICE_INDEX] = table.device.to_byte();
        in_buffer[12..].copy_from_slice(&table.table);
        self.control(&mut in_buffer)
    }

    fn control(&mut self, in_buffer: &mut [u8]) -> Result<(), AfErr> {
        let mut out_buffer: [u8; 1024] = [0; 1024];
        self.device_control
            .control(CONTROL_CODE, in_buffer, &mut out_buffer)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PowerPlan {
    PerformanceWindows,
    TurboManual,
    Silent,
}

impl PowerPlan {
    pub(crate) fn to_byte(&self) -> u8 {
        match self {
            PowerPlan::PerformanceWindows => 0x00,
            PowerPlan::TurboManual => 0x01,
            PowerPlan::Silent => 0x02,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum FanCurveDevice {
    Cpu,
    Gpu,
}

impl FanCurveDevice {
    pub(crate) fn to_byte(&self) -> u8 {
        match self {
            FanCurveDevice::Cpu => 0x24,
            FanCurveDevice::Gpu => 0x25,
        }
    }
}

pub struct FanCurveTable {
    device: FanCurveDevice,
    table: [u8; 16],
}

impl FanCurveTable {
    pub fn entry(&self, index: TableIndex) -> TableEntry {
        let degrees = self.table[index.0 as usize];
        let fan_percent = self.table[index.0 as usize + 8];
        TableEntry {
            degrees,
            fan_percent,
        }
    }

    fn set(&mut self, index: TableIndex, entry: TableEntry) {
        self.table[index.0 as usize] = entry.degrees;
        self.table[index.0 as usize + 8] = entry.fan_percent;
    }

    pub fn is_valid(&self) -> bool {
        let mut last_percentage: u8 = 0;
        for index in TableIndex::iterator() {
            let entry = self.entry(index);
            let degrees = entry.degrees();
            let degrees_are_ok = degrees >= index.min_degrees_inclusive()
                && degrees <= index.max_degrees_inclusive();
            if !degrees_are_ok {
                return false;
            }
            let percentage = entry.fan_percent();
            let fan_percentage_ok = percentage >= index.min_percentage_inclusive(self.device)
                && percentage >= last_percentage;
            last_percentage = percentage;
            if !fan_percentage_ok {
                return false;
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        let mut string = String::with_capacity(100);
        for (index, table_index) in TableIndex::iterator().enumerate() {
            let entry = self.entry(table_index);
            string.push_str(&format!("{}c:{}%", entry.degrees(), entry.fan_percent()));
            if index != TableIndex::max_ordinal() as usize {
                string.push_str(",");
            }
        }
        string
    }

    fn auto_fix(&mut self) {
        let mut last_percentage: u8 = 0;
        for index in TableIndex::iterator() {
            let entry = self.entry(index);

            // fix degrees
            let mut degrees = entry.degrees();
            if degrees > index.max_degrees_inclusive() {
                degrees = index.max_degrees_inclusive();
            }
            if degrees < index.min_degrees_inclusive() {
                degrees = index.min_degrees_inclusive();
            }

            // fix percentage
            let mut percentage = entry.fan_percent();
            if percentage < index.min_percentage_inclusive(self.device) {
                percentage = index.min_percentage_inclusive(self.device);
            }
            if percentage < last_percentage {
                percentage = last_percentage;
            }
            last_percentage = percentage;

            // write back
            let new_entry = TableEntry::new(degrees, percentage);
            self.set(index, new_entry);
        }
    }
}

pub struct FanCurveTableBuilder {
    table: FanCurveTable,
}

impl FanCurveTableBuilder {
    pub fn new(device: FanCurveDevice) -> Self {
        Self {
            table: FanCurveTable {
                device,
                table: [0; 16],
            },
        }
    }

    pub fn set(&mut self, index: TableIndex, entry: TableEntry) {
        self.table.set(index, entry)
    }

    pub fn is_valid(&self) -> bool {
        self.table.is_valid()
    }

    pub fn auto_fix(&mut self) {
        self.table.auto_fix()
    }

    pub fn auto_fix_build(self) -> FanCurveTable {
        let mut table = self.table;
        table.auto_fix();
        table
    }

    pub fn from_string(
        device: FanCurveDevice,
        string: &str,
    ) -> Result<FanCurveTableBuilder, AfErr> {
        let regex = Regex::new(r"\s*(\d{1,3})c:(\d{1,3})%\s*").unwrap();
        let mut builder = FanCurveTableBuilder::new(device);

        for (index, pattern) in string.split(",").enumerate() {
            if index > TableIndex::max_ordinal() as usize {
                return Err(format!(
                    "Too many entries for fan curve table, cannot have more \
                than {} entries.",
                    TableIndex::max_ordinal() + 1
                )
                .into());
            }
            let mut captures = regex.captures_iter(pattern);
            if let Some(captures) = captures.next() {
                let table_index = TableIndex::from_ordinal(u8::try_from(index)?);
                if let (Ok(degrees), Ok(percentage), Some(table_index)) = (
                    captures.index(1).parse::<u8>(),
                    captures.index(2).parse::<u8>(),
                    table_index,
                ) {
                    builder.set(table_index, TableEntry::new(degrees, percentage));
                    continue;
                }
            }

            return Err(format!(
                "Unable to parse '{}': It must look like this: \
            <DEGREES>c:<PERCENT>%, examples: 35c:45% or 55c:75% (while degrees must be \
            <=255 and percent within 0-100).",
                pattern
            )
            .into());
        }
        Ok(builder)
    }
}

pub struct TableEntry {
    degrees: u8,
    fan_percent: u8,
}

impl TableEntry {
    pub fn new(degrees: u8, fan_percent: u8) -> Self {
        Self {
            degrees,
            fan_percent,
        }
    }

    pub fn degrees(&self) -> u8 {
        self.degrees
    }

    pub fn fan_percent(&self) -> u8 {
        self.fan_percent
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableIndex(u8);

impl TableIndex {
    pub const fn max_ordinal() -> u8 {
        7
    }

    pub fn iterator() -> impl Iterator<Item = TableIndex> {
        (0..Self::max_ordinal() + 1).map(|ordinal| TableIndex(ordinal))
    }

    pub fn from_ordinal(ordinal: u8) -> Option<TableIndex> {
        if ordinal <= Self::max_ordinal() {
            Some(TableIndex(ordinal))
        } else {
            None
        }
    }

    pub fn min_degrees_inclusive(&self) -> u8 {
        (self.0 * 10) + 30
    }

    pub fn max_degrees_inclusive(&self) -> u8 {
        (self.0 * 10) + 39
    }

    pub fn min_percentage_inclusive(&self, device: FanCurveDevice) -> u8 {
        match self.0 {
            0 | 1 | 2 | 3 => 0,
            4 => match device {
                FanCurveDevice::Cpu => 31,
                FanCurveDevice::Gpu => 34,
            },
            5 => match device {
                FanCurveDevice::Cpu => 49,
                FanCurveDevice::Gpu => 51,
            },
            6 | 7 => match device {
                FanCurveDevice::Cpu => 56,
                FanCurveDevice::Gpu => 61,
            },
            _ => panic!("No such table index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FanCurveDevice, FanCurveTableBuilder};

    #[test]
    pub fn minimum_cpu_table_temp_given() {
        let minimum_table_string = "39c:0%,49c:0%,59c:0%,69c:0%,79c:31%,89c:49%,99c:56%,109c:56%";
        let table =
            FanCurveTableBuilder::from_string(FanCurveDevice::Cpu, minimum_table_string).unwrap();
        assert_eq!(true, table.is_valid());
        // auto-fix should do nothing
        let table = table.auto_fix_build();
        // should return the same
        assert_eq!(table.to_string(), minimum_table_string);
    }

    #[test]
    pub fn minimum_gpu_table_temp_given() {
        let minimum_table_string = "39c:0%,49c:0%,59c:0%,69c:0%,79c:34%,89c:51%,99c:61%,109c:61%";
        let table =
            FanCurveTableBuilder::from_string(FanCurveDevice::Gpu, minimum_table_string).unwrap();
        assert_eq!(true, table.is_valid());
        // auto-fix should do nothing
        let table = table.auto_fix_build();
        // should return the same
        assert_eq!(table.to_string(), minimum_table_string);
    }

    #[test]
    pub fn minimum_cpu_table() {
        let table_string = "150c:0%,150c:0%,150c:0%,150c:0%,150c:0%,150c:0%,150c:0%,150c:0%";
        let minimum_allowed = "39c:0%,49c:0%,59c:0%,69c:0%,79c:31%,89c:49%,99c:56%,109c:56%";
        let table = FanCurveTableBuilder::from_string(FanCurveDevice::Cpu, table_string).unwrap();
        assert_eq!(false, table.is_valid());
        // should fix the table to minimum values
        let table = table.auto_fix_build();
        // should return the same
        assert_eq!(table.to_string(), minimum_allowed);
    }

    #[test]
    pub fn minimum_gpu_table() {
        let table_string = "150c:0%,150c:0%,150c:0%,150c:0%,150c:0%,150c:0%,150c:0%,150c:0%";
        let minimum_allowed = "39c:0%,49c:0%,59c:0%,69c:0%,79c:34%,89c:51%,99c:61%,109c:61%";
        let table = FanCurveTableBuilder::from_string(FanCurveDevice::Gpu, table_string).unwrap();
        assert_eq!(false, table.is_valid());
        // should fix the table to minimum values
        let table = table.auto_fix_build();
        // should return the same
        assert_eq!(table.to_string(), minimum_allowed);
    }
}
