use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

#[derive(Debug, serde::Deserialize)]
struct Note {
    pitch: String,
    octave: u8,
    duration: f32,
}

#[derive(Debug, serde::Deserialize)]
struct Tune {
    title: String,
    author: String,
    tempo: u16,
    instrument: String,
    notes: Vec<Note>,
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
fn frequency_of_note(pitch: &str, octave: u8) -> f32 {
    //Delegate to the non-momentoized function on the first call
    //and memoize the result for subsequent calls
    //

    //Create a static mutable reference to a HashMap
    //
    //The static keyword means that the variable is only created once

    let mut memoized_frequencies: HashMap<(String, u8), f32> = HashMap::new();
    if memoized_frequencies.contains_key(&(pitch.to_string(), octave)) {
        return memoized_frequencies[&(pitch.to_string(), octave)];
    } else {
        let frequency = calculate_frequency_of_note(pitch, octave);
        memoized_frequencies.insert((pitch.to_string(), octave), frequency);
        return frequency;
    }
}
fn main() {
    // Read in first input argument as location
    let arguments: Vec<String> = env::args().collect();

    let location = &arguments[1];

    // Open the JSON file and parse it into a Tune struct
    let file = File::open(location).unwrap();
    let reader = BufReader::new(file);
    let tune: Tune = serde_json::from_reader(reader).unwrap();

    println!(
        "Playing tune {:?} by {:?} for {:?}",
        tune.title, tune.author, tune.instrument
    );
    // Compute the duration of a quarter note in seconds based on the tempo
    let quarter_note_duration = 1000.0 * ((60.0 / f32::from(tune.tempo)) as f32);

    // Create a Rodio Sink for audio playback
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Play each note in the tune
    for note in tune.notes {
        // Compute the frequency of the note based on the pitch and octave
        let frequency = frequency_of_note(&note.pitch, note.octave);
        //output the note to standard output
        //
        println!("Playing note: {:?}", note.pitch);
        // Compute the duration of the note in seconds
        let duration = (quarter_note_duration * note.duration) as u64;
        println!("For {:?} seconds", duration);
        let source = SineWave::new(frequency)
            .take_duration(Duration::from_millis(duration))
            .amplify(0.20);
        sink.append(source);
    }

    // Wait for the audio to finish playing before exiting
    sink.play();
    sink.sleep_until_end();
}

