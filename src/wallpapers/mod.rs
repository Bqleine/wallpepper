use sdl2::render::WindowCanvas;

mod chroma;
mod outrun;
mod video;

pub trait Wallpaper {
    /// This function is called to create your wallpaper struct and load what it needs.
    fn new(args: Vec<String>) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// This function is called every loop cycle, draw your wallpaper onto the SDL Canvas here.
    ///
    /// How often this is called depends on the frame rate set by the [Wallpaper::frame_rate()] function.
    fn draw(&mut self, canvas: &mut WindowCanvas);

    /// The number of times per second the wallpaper is redrawn, keeping this as default should be good for most cases.
    fn frame_rate(&self) -> f64 {
        30.0
    }
}

/// Returns a [Wallpaper] object corresponding to its name entered in the command line.
pub fn get_wallpaper_by_name(
    name: &str,
    wallpaper_args: Vec<String>,
) -> Result<Box<dyn Wallpaper>, &'static str> {
    Ok(match name {
        "chroma" => Box::new(chroma::Chroma::new(wallpaper_args)?),
        "video" => Box::new(video::Video::new(wallpaper_args)?),
        "outrun" => Box::new(outrun::Outrun::new(wallpaper_args)?),
        _ => return Err("Unknown wallpaper type"),
    })
}
