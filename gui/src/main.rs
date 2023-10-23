mod audio;
mod settings;
use audio::audio::AudioRecorder;
use settings::settings::settings_window;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let mut device_num = String::new();

    io::stdin()
        .read_line(&mut device_num)
        .expect("Failed to read line");

    //let rec = Arc::new(Mutex::new(AudioRecorder::new(cpal::default_host())));
    //let rec_clone = Arc::clone(&rec);
    //thread::spawn(move || {
    //    let rec = AudioRecorder::new(cpal::default_host());
    //});

    settings_window();
    //rec.record(audio_options);
}
