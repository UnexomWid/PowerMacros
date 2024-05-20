use active_win_pos_rs::get_active_window;
use std::path::Path;
use windows::core::w;
use windows::Win32::Storage::FileSystem::{ReadFile, PIPE_ACCESS_INBOUND};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, PIPE_READMODE_BYTE, PIPE_WAIT,
};
use winmix::WinMix;

unsafe fn toggle_zen_mode(state: bool) {
    let winmix = WinMix::default();

    let win = get_active_window().unwrap();
    let focused = Path::new(&win.process_path).file_name().unwrap();

    let sessions = winmix.enumerate().unwrap();

    for session in sessions {
        let exe = Path::new(&session.path).file_name().unwrap();

        let should_mute = state && exe != focused && exe != "AutoHotkey64.exe";

        session.vol.set_mute(should_mute).unwrap();
    }

    if state {
        println!(
            "[ZenMode] ENGAGED for {:?} (PID {} | {})",
            &focused, win.process_id, &win.title
        );
    } else {
        println!("[ZenMode] Disengaged");
    }
}

fn main() {
    println!(
        r#"

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
                                                              

"#
    );

    unsafe {
        let pipe = CreateNamedPipeW(
            w!("\\\\.\\pipe\\powermacros"),
            PIPE_ACCESS_INBOUND,
            PIPE_READMODE_BYTE | PIPE_WAIT,
            1,    // nMaxInstances
            1,    // nOutBufferSize
            1,    // nInBufferSize
            0,    // nDefaultTimeOut
            None, // lpSecurityAttributes
        );

        let mut buff = [0u8; 1];
        let mut count = 0;

        println!("Listening for hotkeys...");

        loop {
            ConnectNamedPipe(pipe, None).unwrap();

            loop {
                match ReadFile(pipe, Some(&mut buff), Some(&mut count), None) {
                    Ok(()) => {
                        if count == 1 {
                            toggle_zen_mode(buff[0] == 1);
                        }
                    }
                    Err(_) => {
                        DisconnectNamedPipe(pipe).unwrap();
                        break;
                    }
                }
            }
        }
    }
}
