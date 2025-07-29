use anyhow::{anyhow, bail, Result};
use cpal::{self, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig, SupportedStreamConfig};
use std::io::{self, Write};

pub struct AudioEngine {
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    stream: cpal::Stream,
}

impl AudioEngine {
    pub fn select_device(host: &cpal::Host) -> Result<cpal::Device> {
        let devices: Vec<_> = host.output_devices().expect("Failed to get output devices").collect();
        for (i, device) in devices.iter().enumerate() {
            println!("{}: {}", i + 1, device.name()?);
        }
        print!("Select device (1-{}): ", devices.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: usize = input.trim().parse().unwrap_or(0);
        if choice == 0 || choice > devices.len() {
            bail!("Invalid device choice");
        }
        let selected_device = &devices[choice - 1];
        println!("You selected: {}", selected_device.name().unwrap_or("Unknown".to_string()));
        Ok(selected_device.clone())
    }

    pub fn select_config(device: &cpal::Device) -> Result<SupportedStreamConfig> {
        let configs: Vec<_> = match device.supported_output_configs() {
            Ok(cfgs) => cfgs.collect(),
            Err(_) => {
                bail!("Failed to get supported output configs for device: {}", device.name()?);
            }
        };

        for (i, config) in configs.iter().enumerate() {
            println!("{}: {:?}", i + 1, config);
        }

        print!("Select config (1-{}): ", configs.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let config_choice: usize = input.trim().parse().unwrap_or(0);

        if config_choice == 0 || config_choice > configs.len() {
            bail!("Invalid config choice");
        }

        let config_range = &configs[config_choice - 1];
        let chosen_config = config_range.with_sample_rate(config_range.min_sample_rate());
        Ok(chosen_config)
    }
}
