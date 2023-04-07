use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

mod frequency_calculator;
use frequency_calculator::FrequencyCalculator;

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

fn main() {
    // Read in first input argument as location
    let arguments: Vec<String> = env::args().collect();

    let location = &arguments[1];
    let mut frequency_calculator = FrequencyCalculator::new();
    // Open the JSON file and parse it into a Tune struct
    let file = File::open(location).unwrap();
    let reader = BufReader::new(file);
    let tune: Tune = serde_json::from_reader(reader).unwrap();

    println!(
        "Playing tune {:?} by {:?} for {:?}",
        tune.title, tune.author, tune.instrument
    );
    let quarter_note_duration = 1000.0 * ((60.0 / f32::from(tune.tempo)) as f32);

    // Create a Rodio Sink for audio playback
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    loop {
        // Play each note in the tune
        for note in &tune.notes {
            let frequency = frequency_calculator.frequency_of_note(&note.pitch, note.octave);
            println!("Playing note: {:?}", note.pitch);
            let duration = (quarter_note_duration * note.duration) as u64;
            println!("For {:?} milliseconds", duration);
            let source = SineWave::new(frequency)
                .take_duration(Duration::from_millis(duration))
                .amplify(0.20);
            sink.append(source);
        }

        sink.play();
        sink.sleep_until_end();
    }
}
