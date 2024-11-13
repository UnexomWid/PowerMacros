#SingleInstance Force

TrySoundPlay(path) {
    try SoundPlay path
}

; Unified Enter
NumpadEnter::Enter

; Universal Advance
#HotIf GetKeyState("ScrollLock", "T")
Right::Enter
#HotIf

ScrollLock::{
    If GetKeyState("ScrollLock", "T") {
        SetScrollLockState("Off")
        TrySoundPlay "assets/UniversalAdvance_disengage.wav"
    } else {
        SetScrollLockState("On")
        TrySoundPlay "assets/UniversalAdvance_engage.wav"
    }
}

; SudoF4
#F4::{
    active_name := WinGetProcessName("A")
    ProcessClose active_name
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

; MaximizeAny
#F5::{
    ;           WM_SYSCOMMAND  SC_MAXIMIZE unused  No control  Active window
    PostMessage 0x0112,        0xF030,     0,      ,           "A"
    TrySoundPlay "assets/MaximizeAny.wav"
}