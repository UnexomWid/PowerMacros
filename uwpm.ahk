#SingleInstance Force

TrySoundPlay(path) {
    try {
        SoundPlay path
    } catch { }
}

; Unified Enter
NumpadEnter::Enter

; SudoF4
#F4::{
    active_id := WinGetID("A")
    WinKill active_id
    TrySoundPlay "assets/SudoF4.wav"
}

; ZenMode
#F2::{
    static zen := false
    zen := !zen

    static file := FileOpen("\\.\pipe\powermacros","w")

    if (zen) {
        file.WriteUChar(1)
        TrySoundPlay "assets/Zen_engage.wav"
    } else {
        file.WriteUChar(0)
        TrySoundPlay "assets/Zen_disengage.wav"
    }

    file.Read(0)
}