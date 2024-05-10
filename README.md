# About <a href="https://www.rust-lang.org/"><img align="right" src="https://img.shields.io/badge/Rust-1%2E73-f74c00?logo=Rust" alt="Rust 1.73" /></a>

**UW's PowerMacros** is a Windows program that empowers the user with various *niche* macros that do very specific, powerful and magic things.

For now, it contains these 2 macros:

- **SudoF4**: forcefully close the current window via `Win + F4`, bypassing `Alt + F4` hooks ([read more here](https://github.com/UnexomWid/SudoF4)) 
- **ZenMode**: mute everything except the focused window and system sounds via `Win + F2`

This project is tailored specifically for me. I'm still developing macros..

Code is messy. I plan to add other functionality, not just hotkeys.
I'm focusing on getting stuff working rather than making it clean, because I'll refactor everything when I'm satisfied with the amount of content.

PRs for new macros are welcome.
However, if they don't fit within my use cases, feel free to fork this project and empower yourself with your own specific macros! ツ

# Installation

You first need to install [Rust](https://www.rust-lang.org/).

Then clone and install PowerMacros:

```sh
git clone https://github.com/UnexomWid/PowerMacros

cd PowerMacros

cargo install --path .
```


# Usage

Open a CMD window and run:

```sh
cd <path_to_powermacros_repo>

uwpm
```

Keep it running in the background. If something doesn't work, run CMD as admin.

You can disable specific macros via `uwpm.json` in the current working directory (a default config is created on the first run).

It's recommended to run PowerMacros from a dedicated directory where it can store the config, like the repo dir. This is why you should CD into it before running.

If you don't CD into a dedicated dir before running, the config will be created in the working dir, which will most likely be `C:/Windows/System32`, and you want to avoid that.

## Custom sounds

Whenever you activate a macro, a custom sound can be played. Place them in `assets/` next to the config file and name them like this:

- `SudoF4_ok.ogg`: SudoF4 killed the current window
- `SudoF4_fail.ogg`: SudoF4 failed to kill
- `Zen_engage.ogg`: Zen Mode was engaged
- `Zen_disengage.ogg`: Zen Mode was disengaged

# License <a href="https://github.com/UnexomWid/PowerMacros/blob/master/LICENSE"><img align="right" src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>

**PowerMacros** was created by [UnexomWid](https://uw.exom.dev). It is licensed under [MIT](https://github.com/UnexomWid/PowerMacros/blob/master/LICENSE-MIT) OR [Apache 2](https://github.com/UnexomWid/PowerMacros/blob/master/LICENSE-APACHE).