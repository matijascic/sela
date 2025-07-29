use std::io::{self, Write};
mod audio;

use cpal::{self, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig, SupportedStreamConfig};

fn main() {
    let stream: cpal::Stream;
    {
        let host = cpal::default_host();
        let devices: Vec<_> = host.output_devices().expect("Failed to get output devices").collect();
        for (i, device) in devices.iter().enumerate() {
            println!("{}: {}", i + 1, device.name().unwrap_or("Unknown".to_string()));
        }

        print!("Select device (1-{}): ", devices.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice: usize = input.trim().parse().unwrap_or(0);

        if choice == 0 || choice > devices.len() {
            eprintln!("Invalid choice");
            return;
        }

        let selected_device = &devices[choice - 1];
        println!("You selected: {}", selected_device.name().unwrap_or("Unknown".to_string()));
        
        let configs: Vec<_> = match selected_device.supported_output_configs() {
            Ok(cfgs) => cfgs.collect(),
            Err(_) => {
                eprintln!("Could not query configs");
                return;
            }
        };

        for (i, config) in configs.iter().enumerate() {
            println!("{}: {:?}", i + 1, config);
        }

        print!("Select config (1-{}): ", configs.len());
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let config_choice: usize = input.trim().parse().unwrap_or(0);

        if config_choice == 0 || config_choice > configs.len() {
            eprintln!("Invalid config choice");
            return;
        }

        let config_range = &configs[config_choice - 1];
        let chosen_config = config_range.with_sample_rate(config_range.min_sample_rate());

        println!(
            "Using device '{}' with config {:?}",
            selected_device.name().unwrap_or("Unknown".to_string()),
            chosen_config
        );

        let mut reader = hound::WavReader::open("resources/corvette.wav").unwrap();
        let spec = reader.spec();
        println!("Sample format: {:?}", spec.sample_format);
        println!("Bits per sample: {:?}", spec.bits_per_sample);
        println!("Sample rate: {}", spec.sample_rate);
        println!("Channels: {}", spec.channels);
        let samples: Vec<f32> = reader
            .samples::<i16>() 
            .filter_map(Result::ok)
            .map(|s| s as f32 / i16::MAX as f32) 
            .collect();

        println!("Loaded {} samples", samples.len());

        let mut index = 0;
        let stream_config: StreamConfig = chosen_config.into();
        stream = selected_device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = samples[index % samples.len()];
                    index += 1;
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        ).expect("Failed to build output stream");
    }
    stream.play().expect("Failed to play output stream");
    println!("Stream is playing. Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    stream.pause().expect("Failed to pause stream");
    println!("Stream paused. Exiting...");
}