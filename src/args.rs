use crate::wallpapers::get_wallpaper_by_name;
use crate::Wallpaper;

pub fn parse_args() -> Result<Box<dyn Wallpaper>, &'static str> {
    let mut args = std::env::args();

    if args.len() <= 1 {
        return Err("Usage: wallpepper <name> [options]");
    }

    // Skip first argument (executable name)
    args.next();

    let name = args.next().unwrap();

    let wallpaper_args: Vec<String> = args.collect();

    get_wallpaper_by_name(name.as_str(), wallpaper_args)
}
