use std::env;
use std::io::Cursor;
use std::process::{Command, exit};
use rodio::{Decoder, OutputStream, source::Source};

// Embed the sound files into the binary
const SUCCESS_SOUND: &[u8] = include_bytes!("../sounds/ding.wav");
// TODO: Add failure sound

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: dingon <command> [args...]");
        exit(1);
    }

    let command = &args[1];
    let command_args = &args[2..];

    match Command::new(command).args(command_args).status() {
        Ok(status) => {
            if status.success() {
                play_sound(SUCCESS_SOUND);
                exit(0);
            } else {
                play_sound(SUCCESS_SOUND);
                exit(status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            play_sound(SUCCESS_SOUND);
            exit(1);
        }
    }
}

fn play_sound(sound_data: &'static [u8]) {
    // Set up the output stream for audio playback
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Sleep for the duration of the sound to ensure it plays completely
    std::thread::sleep(std::time::Duration::from_secs(2));
}
