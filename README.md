# Wayland Terminal Screenshotter <wl-termshot>

A screenshot tool for Wayland that uses TUI to configure screenshot options.

## Usage

Just type `wl-termshot` and you can start taking screenshot! Screenshots would be saved to $GRIM_DEFAULT_DIR in the case that `temporary` is not set.

## Features
- Select a specific area to take screenshot
- Option to choose if wl-termshot copy screenshot to clipboard for every screenshot
- Option to let screenshot saved to /tmp
- Option to customize screenshot file name (default is screen_YYYY-MM-DD-hh:mm:ss.png)

## Limitation

Only supports Linux now. Tested on Hyprland with Arch Linux.

## Roadmap
[] Add "--help" and stuff like that
[] Prevent panicing when dependencies are not installed
[] Use a environment variable other than $GRIM_DEFAULT_DIR
[] Save current directory when environment variables are not set
[] Make error looks nicer
[] Freeze screen using `hyprpicker`
[] Support different image format 
[] Maybe use TUI library other than Cursive for better looking
[] Add CLI interface, perhaps using Clap
[] Support X11 and other platforms (low prir)

## Dependencies
- `grim` for taking screenshot
- `setsid` for generating process (should be provided on Linux)
- `libnotify`
- `slurp` for selecting area [Optional]
- `wl-clipboard` for copying screenshot [Optional]

## Install
Currently only support building from source. Dependent on the Rust toolchain.

```bash
git clone https://github.com/inker0/wl-termshot.git
cd wl-termshot
cargo build --release
```

The executable `wl-termshot` is located at `./target/release/`.

## Tips

You can create keybinds for wl-termshot in Hyprland like this:
```
bind = SUPER SHIFT, S, exec, $terminal --class="wl-termshot.floating" -e wl-termshot # Create keybind
windowrule = float,class:wl-termshot.floating # Float window
windowrule = size 700 350,class:wl-termshot.floating # Set size
```

## Contributing

This is my personal project (just for personal use). If you find wl-termshot useful or you find a issue inside this tool, freely consider contributing through issues and pull requests. 
## License
MIT License.

