use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct MidiPlayer {
    sink: Sink,
}

impl MidiPlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self { sink }
    }

    pub fn play_midi(&self, file_path: &str, loop_track: bool) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        if loop_track {
            self.sink.append(source.repeat_infinite());
        }
        self.sink.play();
        println!("Playing MIDI: {}", file_path);
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}

// Integration example
pub fn init_midi_player() {
    let player = MidiPlayer::new();
    player.play_midi("assets/music/light_of_the_seven_inspired.mid", true);
}
