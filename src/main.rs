mod audio;
use audio::engine::AudioEngine;
use audio::io::load_file_as_f32;

use anyhow::Result;
use cpal::{self, traits::{DeviceTrait, StreamTrait}, StreamConfig};

fn main() -> Result<()> {
    let host = cpal::default_host();
    let device = AudioEngine::select_device(&host)?;
    let config = AudioEngine::select_config(&device)?;
    println!("Using device '{}' with config {:?}", device.name()?, config);

    let file = "resources/corvette.wav";
    let samples = load_file_as_f32(file)?;
    println!("Loaded {} samples", samples.len());

    let mut index = 0;
    let stream_config: StreamConfig = config.into();
    let stream = device.build_output_stream(
        &stream_config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = samples[index % samples.len()];
                index += 1;
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    stream.play()?;
    println!("Stream is playing. Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    stream.pause()?;
    println!("Stream paused. Exiting...");

    Ok(())
}