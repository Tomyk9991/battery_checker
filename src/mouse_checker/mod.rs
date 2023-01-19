extern crate hidapi;

use hidapi::DeviceInfo;
use crate::mouse_checker::RivalError::HandleNotOpenable;

struct Rival650Config {
    vendor_id: u16,
    product_id: u16,
    endpoint: i32,
    report_type: u8,
}

#[derive(Debug)]
pub enum RivalError {
    DeviceNotFound,
    HandleNotOpenable,
    ReadNotAllowed,
    WriteNotAllowed,
    ParsingFailed,
    Unexpected,
}

pub struct Rival650 {
    battery_level: u8,
    wired: bool,
}

impl Rival650 {
    pub fn new() -> Self {
        Self {
            battery_level: 0,
            wired: false,
        }
    }

    /// Connects to the mouse via. the HID API and updates the internal battery and wired states
    /// You need to connect everytime you want to update
    pub fn connect(&mut self) -> Result<(), RivalError> {
        let rival_wired = Rival650Config {
            vendor_id: 0x1038,
            product_id: 0x172B,
            endpoint: 0,
            report_type: 0x02,
        };

        let rival_wireless = Rival650Config {
            vendor_id: 0x1038,
            product_id: 0x1726,
            endpoint: 0,
            report_type: 0x02,
        };

        let api = hidapi::HidApi::new().ok().ok_or(RivalError::Unexpected)?;

        for rival_config in &[rival_wireless, rival_wired] {
            let rival_hids = api.device_list()
                .filter(|device| {
                    device.vendor_id() == rival_config.vendor_id &&
                        device.product_id() == rival_config.product_id &&
                        device.interface_number() == rival_config.endpoint
                })
                .collect::<Vec<&DeviceInfo>>();
            

            let device_info = rival_hids
                .first()
                .ok_or(RivalError::DeviceNotFound)?;

            let mouse_handle = device_info
                .open_device(&api)
                .ok()
                .ok_or(HandleNotOpenable)?;


            let battery_level: [u8; 3] = [rival_config.report_type, 0xAA, 0x01];

            let _ = mouse_handle.write(&battery_level)
                .ok()
                .ok_or(RivalError::WriteNotAllowed)?;

            let mut buf = [0u8; 3];
            let _ = mouse_handle.read_timeout(&mut buf[..], 200)
                .ok()
                .ok_or(RivalError::ReadNotAllowed)?;

            if let [0, 0, 0] = buf {} else {
                self.battery_level = buf[0];
                self.wired = match buf[2] {
                    0 => false,
                    1 => true,
                    _ => return Err(RivalError::ParsingFailed)
                };

                return Ok(());
            }
        }

        return Ok(());
    }

    pub fn battery_level(&self) -> u8 {
        Rival650::map(self.battery_level, 0, 98, 0, 100).min(100).max(0)
    }

    fn map(value: u8, initial_start: u8, initial_stop: u8, mapped_start: u8, mapped_stop: u8) -> u8 {
        let value = mapped_start as f32 + (mapped_stop as f32 - mapped_start as f32) * ((value as f32 - initial_start as f32) / (initial_stop as f32 - initial_start as f32));
        return value as u8;
    }

    pub fn get_is_wired(&self) -> bool {
        self.wired
    }
}