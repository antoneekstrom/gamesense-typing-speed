# Typing Speed Indicator 
Typing speed indicator for SteelSeries GameSense built with Rust. 😎😎😎

Shows typing speed on the little OLED on your SteelSeries Apex 7 keyboard.

## Build for production
Build the program by running `cargo build --bins --release`. Afterwards you can find the binaries in `target/release`. You will need Rust and Cargo or something to be installed.

## To-Do
- Show more information on the display. Ideas:
   * CPM and WPM
   * Timespan which the program is using for calculating CPM/WPM
   * Count key presses
   * A cool graph
   * Historical data eg. peak and average speed
- Change keyboard illumination based on typing speed. Cold blue or white can gradually change to red when typing fast 😎😎
- Allow for adjusting timings somehow, I think there's something built into GameSense
- Adjust WPM timeframe while typing, like decreasing it when you begin typing or something
- ~~Put binary on Github as a release~~
- Currently, any key counts as a keypress (such as backspace which WHACK haha)

## How it works
Tracks your key presses and when they occur. Computes the typing speed by counting the keys that were pressed during the last 15 seconds and displays the typing speed as WPM (Words Per Minute) on the OLED display anytime you press a key. Key presses are cleared if the next key press takes longer than 7 seconds.

Shows an icon in your system-tray on Windows (not sure what happens on other systems) which to you use to stop the program from running in the background.
