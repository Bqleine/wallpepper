# Wallpepper
A tool to display videos and SDL content as a wallpaper on linux minimalist window managers using Xorg

[video](https://user-images.githubusercontent.com/51089082/187094274-226609e6-e18f-4ec5-bdc8-1f7a3b12d5cb.mp4)

## Installing
You can either run it from source or install it with :
```
$ cargo install wallpepper
```

### Dependencies
To run this, you will need ffmpeg and SDL2 installed via your package manager along with their header files (sometimes bundled together, just look it up on your distro's wiki)

## Usage
The basic usage for wallpepper is as follows :
```
$ wallpepper <wallpaper> [wallpaper args]
```

Currently, only two wallpapers are supported:
- chroma, which is just a rainbow
- video, which will display the video file specified in a loop using ffmpeg

### Example usage
```
$ wallpepper chroma
$ wallpepper video path/to/video.mp4
```

## How it works
Wallpepper uses SDL2 and the X11 libraries to draw directly to the root window, on minimalist window managers this will be your background. Others though, like plasma or gnome will draw over this root window so it won't be visible.

You can create your own animated wallpapers using SDL2 with just a few steps:
1. Create a new module for your wallpaper in `wallpapers/`
2. Create a new struct for your wallpaper, you can put whatever you need here
3. Implement the `Wallpaper` trait on your struct
4. Add it to `wallpapers/mod.rs` (see lines 3 and 21 for an example)
5. That's it! You can now try your wallpaper with the name you gave it in the `get_wallpaper_by_name` function!
6. Optional: If you want, you can set the `TEST` constant to `true` to have your wallpaper open in a floating window instead.
7. Optional: Make a pull request to add your wallpaper to the defaults 😎
