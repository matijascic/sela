use anyhow::Result;
mod audio;

use audio::engine::AudioEngine;

use cpal::{self, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig, SupportedStreamConfig};

fn main() -> Result<()> {
    let host = cpal::default_host();
    let device = AudioEngine::select_device(&host)?;
    let config = AudioEngine::select_config(&device)?;

    println!(
        "Using device '{}' with config {:?}",
        device.name()?,
        config
    );

    let mut reader = hound::WavReader::open("resources/corvette.wav")?;
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