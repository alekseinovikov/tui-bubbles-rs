use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossterm::event::KeyCode;
use std::f32::consts::PI;

pub(crate) fn generate_sound(frequency: f32, duration_ms: u64) {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No output device available");

    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate().0 as f32;
    let mut sample_clock = 0f32;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = (sample_clock * frequency * 2.0 * PI / sample_rate).sin();
                    sample_clock += 1.0;
                }
            },
            |err| eprintln!("An error occurred on the audio stream: {}", err),
            None,
        )
        .expect("Failed to build output stream");

    stream.play().expect("Failed to play stream");

    std::thread::sleep(std::time::Duration::from_millis(duration_ms));

    stream.pause().expect("Failed to pause stream");
}

pub(crate) fn get_frequency_for_key(key_code: &KeyCode) -> Option<f32> {
    const A4_FREQUENCY: f32 = 440.0;

    let key_frequencies: [(char, i32); 24] = [
        ('q', -9),  // F4
        ('2', -8),  // F#4/Gb4
        ('w', -7),  // G4
        ('3', -6),  // G#4/Ab4
        ('e', -5),  // A4
        ('r', -4),  // A#4/Bb4
        ('5', -3),  // B4
        ('t', -2),  // C5
        ('6', -1),  // C#5/Db5
        ('y', 0),   // D5
        ('7', 1),   // D#5/Eb5
        ('u', 2),   // E5
        ('i', 3),   // F5
        ('9', 4),   // F#5/Gb5
        ('o', 5),   // G5
        ('0', 6),   // G#5/Ab5
        ('p', 7),   // A5
        ('[', 8),   // A#5/Bb5
        ('=', 9),   // B5
        ('z', -10), // E4
        ('s', -8),  // F#4/Gb4
        ('d', -6),  // G#4/Ab4
        ('g', -4),  // A#4/Bb4
        ('h', -2),  // C5
    ];

    if let KeyCode::Char(ch) = key_code {
        for &(key, offset) in &key_frequencies {
            if key == *ch {
                return Some(A4_FREQUENCY * 2f32.powf(offset as f32 / 12.0));
            }
        }
    }

    None
}