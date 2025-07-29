mod audio;


use cpal::{self, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig, SupportedStreamConfig};

fn main() {
    
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available");
    let config: SupportedStreamConfig = device.default_output_config()
        .expect("Failed to get default output format").into();

    println!("Using device: {}", device.name().unwrap());
    println!("Output format: {:?}", config);

    let sine_buffer: Vec<f32> = (0..44100*10).map(|x| {
        let freq = 440.0; // A4 note
        (x as f32 * freq * 2.0 * std::f32::consts::PI / 44100.0).sin()
    }).collect();
    let mut index = 0;
    let stream_config: StreamConfig = config.into();
    let stream = device.build_output_stream(
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

    stream.play().expect("Failed to play output stream");
    println!("Stream is playing. Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    stream.pause().expect("Failed to pause stream");
    println!("Stream paused. Exiting...");
    // The stream will be dropped here, which stops the audio playback.

}
