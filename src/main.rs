use active_win_pos_rs::get_active_window;
use std::process::Command;
use std::fs::{self, File};
use std::path::Path;
use std::io::BufReader;
use serde::Deserialize;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};
use rodio::{Decoder, OutputStream, Sink};
use winmix::WinMix;

#[derive(Deserialize)]
struct SudoF4Config {
    enabled: bool
}

#[derive(Deserialize)]
struct ZenConfig {
    enabled: bool
}

#[derive(Deserialize)]
struct Config {
    sudo_f4: SudoF4Config,
    zen: ZenConfig
}

fn load_config() -> Config {
    let config_name = "uwpm.json";

    // TODO: remove this if the WD remains . all the time.
    // I left this here in case I want to load the config from appdata in the future
    let config_path = Path::new(".").join(config_name);

    if !config_path.exists() {
        println!("Creating default config uwpm.json");

        fs::write(
            &config_path,
            r#"{
    "sudo_f4": {
        "enabled": true
    },
    "zen": {
        "enabled": true
    }
}"#,
        )
        .expect("Cannot create default config (insufficient permissions?)");
    }

    let config_file = fs::read_to_string(config_path)
        .expect("Cannot read config file (maybe the file doesn't exist?)");

    serde_json::from_str(&config_file).expect("File short.json is invalid")
}

fn play_audio(sink: &Sink, file: &str) {
    if Path::new(&file).exists() {
        sink.stop();

        let sound = BufReader::new(File::open(file).unwrap());
        let sound = Decoder::new(sound).unwrap();

        sink.append(sound);
    }
}

fn main() {
    let config = load_config();

    println!(r#"

  _   _ __      __ ( )           
 | | | |\ \    / / |/  ___   
 | |_| | \ \/\/ /     (_-<   
  \___/   \_/\_/      /__/
 (                              *                             
 )\ )                         (  `                            
(()/(      (  (      (   (    )\))(      )      (             
 /(_)) (   )\))(    ))\  )(  ((_)()\  ( /(   (  )(    (   (   
(_))   )\ ((_)()\  /((_)(()\ (_()((_) )(_))  )\(()\   )\  )\  
| _ \ ((_)_(()((_)(_))   ((_)|  \/  |((_)_  ((_)((_) ((_)((_) 
|  _// _ \\ V  V // -_) | '_|| |\/| |/ _` |/ _|| '_|/ _ \(_-< 
|_|  \___/ \_/\_/ \___| |_|  |_|  |_|\__,_|\__||_|  \___//__/ 
                                                              

"#);

    let (_stream, audio_out) = OutputStream::try_default().unwrap();

    let mut hkm = HotkeyManager::new();

    if !config.sudo_f4.enabled {
        println!("[SudoF4] Disabled, nothing to do...");
    } else {
        println!("[SudoF4] Initializing...");

        let audio_sink = Sink::try_new(&audio_out).unwrap();

        hkm.register(VKey::F4, &[ModKey::Win], move || {
            let win = get_active_window();

            if win.is_err() {
                println!("[SudoF4] Failed to get the current active window");
                return;
            }

            let win = win.unwrap();

            let output = Command::new("taskkill")
                .args(["/F", "/PID", &win.process_id.to_string()])
                .output()
                .expect("[SudoF4] Failed to execute 'taskkill'");

            if output.status.success() {
                println!("[SudoF4] Killed process with PID {}: {}", win.process_id, win.title);
                play_audio(&audio_sink, "assets/SudoF4_ok.ogg");
            } else {
                println!(
                    "[SudoF4] Failed to terminate process with PID {}: \n{}",
                    win.process_id, String::from_utf8(output.stderr).unwrap()
                );
                play_audio(&audio_sink, "assets/SudoF4_fail.ogg");
            }
        })
        .unwrap();

        println!("[SudoF4] Listening for Win + F4...");
    }

    if !config.zen.enabled {
        println!("[Zen] Disabled, nothing to do...");
    } else {
        println!("[Zen] Initializing...");

        let audio_sink = Sink::try_new(&audio_out).unwrap();

        hkm.register(VKey::F2, &[ModKey::Win], move || {
            let win = get_active_window();

            if win.is_err() {
                println!("[Zen] Failed to get the current active window");
                return;
            }

            let win = win.unwrap();

            unsafe {
                let winmix = WinMix::default();
                let sessions = winmix.enumerate().unwrap();

                let mut zen_mode = false;

                let focused = Path::new(&win.process_path).file_name().unwrap();

                // Check if we are in zen mode: at least one program is muted.
                // (we can't store and toggle zen_mode outside the callback
                //  because hkm.register wants a Fn callback, not a FnMut)
                for session in &sessions {
                    let exe = Path::new(&session.path).file_name().unwrap();

                    if exe != focused && session.pid != std::process::id() {
                        if session.vol.get_mute().unwrap() {
                            zen_mode = true;
                            break;
                        }
                    }
                }

                // Now do the actual toggling
                zen_mode = !zen_mode;

                for session in sessions {
                    let exe = Path::new(&session.path).file_name().unwrap();

                    if exe != focused && session.pid != std::process::id() {
                        session.vol.set_mute(zen_mode).unwrap();
                    }
                }

                if zen_mode {
                    println!("[Zen] Zen mode ENGAGED for {:?} (PID {} | {})", &focused, win.process_id, &win.title);
                    play_audio(&audio_sink, "assets/Zen_engage.ogg");
                } else {
                    println!("[Zen] Zen mode disengaged");
                    play_audio(&audio_sink, "assets/Zen_disengage.ogg");
                }
            }
        })
        .unwrap();

        println!("[Zen] Listening for Win + F2...");
    }

    hkm.event_loop();
}
