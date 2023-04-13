# Music Player

## Overview

The music player is a simple command-line application that reads and plays JSON-formatted music files. It parses the JSON input and plays the notes using a sine wave generator. This document provides an overview of the JSON format, as well as instructions on how to use the Rust Music Player.
JSON Format

The JSON file used as input for the Rust Music Player should follow this structure:

```json
{
  "title": "Song Title",
  "author": "Author Name",
  "tempo": 120,
  "instrument": "Instrument Name",
  "notes": [
    { "pitch": "C", "octave": 4, "duration": 0.25 },
    ...
  ]
}
```
Fields

    title: The title of the song (string).
    author: The author or composer of the song (string).
    tempo: The tempo of the song in beats per minute (integer).
    instrument: The name of the instrument used for playback. Currently, the Rust Music Player only supports playback using a sine wave generator, so this field is for informational purposes only (string).
    notes: An array of note objects, each representing a single note in the song. The note object contains the following fields:
        pitch: The pitch of the note as a string (e.g., "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"). Use "#" for sharps and "b" for flats.
        octave: The octave of the note as an integer (e.g., 4).
        duration: The duration of the note, expressed as a multiple of a quarter note (e.g., 1 for a quarter note, 0.5 for an eighth note, 2 for a half note).

## Usage

    Compile the Music Player:

```sh
cargo build --release
```
    Run the Rust Music Player with the path to a JSON music file as the first argument:

```sh
cargo run --release path/to/music.json
```

The Rust Music Player will read the JSON file, parse the notes, and play the song using a sine wave generator.
Limitations

    The Rust Music Player currently only supports playback using a sine wave generator, so the instrument field in the JSON file is for informational purposes only.
    The player does not support advanced music notation, dynamics, or multiple instruments. It's designed for simple monophonic playback.
    Complex rhythms, time signatures, and key signatures are not supported.

## Future Improvements

    Support for additional instruments and sound generators
    Support for polyphonic playback (multiple voices or instruments)
