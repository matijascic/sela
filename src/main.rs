mod audio;


use cpal::{self, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig, SupportedStreamConfig};

fn main() {
    let stream: cpal::Stream;
    {
        let host = cpal::default_host();
        let devices = host.output_devices().expect("Failed to get output devices");
        for device in devices {
            println!("Device: {}", device.name().unwrap_or("Unknown".to_string()));
            if let Ok(supported_configs) = device.supported_output_configs() {
                for config in supported_configs {
                    println!("  {:?}", config);
                }
            } else {
                println!("  Could not query configs");
            }
        }

        let device = host.default_output_device().expect("No output device available");
        let config: SupportedStreamConfig = device.default_output_config()
            .expect("Failed to get default output format").into();

        println!("Using device: {}", device.name().unwrap());
        println!("Output format: {:?}", config);

        let seconds = 4;
        let sine_buffer: Vec<f32> = (0..44100*seconds).map(|x| {
            let freq = 440.0;
            (x as f32 * freq * 2.0 * std::f32::consts::PI / 44100.0).sin()
        }).collect();
        let mut index = 0;
        let stream_config: StreamConfig = config.into();
        stream = device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = sine_buffer[index % sine_buffer.len()];
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