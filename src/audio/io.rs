use anyhow::{Result, bail};


pub fn load_file_as_f32(file: &str) -> Result<Vec<f32>> {
    let mut reader = hound::WavReader::open(file)?;
    // let spec = reader.spec();
    // println!("Sample format: {:?}", spec.sample_format);
    // println!("Bits per sample: {:?}", spec.bits_per_sample);
    // println!("Sample rate: {}", spec.sample_rate);
    // println!("Channels: {}", spec.channels);
    
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .filter_map(Result::ok)
        .map(|s| s as f32 / i16::MAX as f32)
        .collect();

    println!("Loaded {} samples", samples.len());
    Ok(samples)
}