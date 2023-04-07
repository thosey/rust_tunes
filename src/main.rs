use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::env;

use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

#[derive(Debug, serde::Deserialize)]
struct Note {
  pitch : String, octave : u8, duration : f32,
}

#[derive(Debug, serde::Deserialize)]
struct Tune {
  title : String,
          author : String,
                   tempo : u16,
                           instrument : String,
                                        notes : Vec<Note>,
}

fn main() {
    // Read in first input argument as location
  let arguments: Vec<String> = env::args().collect();

let location = &arguments[1];

  // Open the JSON file and parse it into a Tune struct
  let file = File::open(location).unwrap();
  let reader = BufReader::new (file);
  let tune : Tune = serde_json::from_reader(reader).unwrap();

  // Compute the duration of a quarter note in seconds based on the tempo
  let quarter_note_duration = 60.0 / f32::from(tune.tempo);

  // Create a Rodio Sink for audio playback
  let(_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();

  // Play each note in the tune
  for
    note in tune.notes {
      // Compute the frequency of the note based on the pitch and octave

    let frequency = match note.pitch.as_str() {
        "C" => 261.63,
        "D" => 293.66,
        "E" => 329.63,
        "F" => 349.23,
        "G" => 392.00,
        "A" => 440.00,
        "B" => 493.88,
        _ => 0.0,
      };

   //output the note to standard output
   //
println!("Playing note: {:?}", note.pitch);


      // Compute the duration of the note in seconds
      let duration = (quarter_note_duration * note.duration) as u64;
      let source = SineWave::new (frequency).take_duration(Duration::from_secs(duration)).amplify(0.20);
      sink.append(source);
    }

  // Wait for the audio to finish playing before exiting
  sink.sleep_until_end();
}
