// frequency_calculator.rs
use std::collections::HashMap;

pub struct FrequencyCalculator {
    calculated_frequencies: HashMap<(String, u8), f32>,
}

impl FrequencyCalculator {
    pub fn new() -> FrequencyCalculator {
        FrequencyCalculator {
            calculated_frequencies: HashMap::new(),
        }
    }

    pub fn frequency_of_note(&mut self, pitch: &str, octave: u8) -> f32 {
        if self
            .calculated_frequencies
            .contains_key(&(pitch.to_string(), octave))
        {
            return self.calculated_frequencies[&(pitch.to_string(), octave)];
        } else {
            let frequency = calculate_frequency_of_note(pitch, octave);
            self.calculated_frequencies
                .insert((pitch.to_string(), octave), frequency);
            return frequency;
        }
    }
}

fn calculate_frequency_of_note(pitch: &str, octave: u8) -> f32 {
    let base_frequency = 440.0; // A4
    let base_octave = 4;
    let semitones_from_a = match pitch {
        "A" => 0,
        "A#" | "Bb" => 1,
        "B" => 2,
        "C" => 3,
        "C#" | "Db" => 4,
        "D" => 5,
        "D#" | "Eb" => 6,
        "E" => 7,
        "F" => 8,
        "F#" | "Gb" => 9,
        "G" => 10,
        "G#" | "Ab" => 11,
        _ => panic!("Invalid pitch"),
    };

    let semitones_from_base = (octave as i32 - base_octave as i32) * 12 + semitones_from_a;
    base_frequency * (2f32).powf(semitones_from_base as f32 / 12f32)
}
