use std::hash::Hasher;

use clap::{Parser, ValueEnum};
use fnv::FnvHasher;
use palette::{FromColor, Okhsl, Srgb};

/// Print a stable color derived from a string, the hostname by default.
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// String to derive the color from [default: hostname]
    seed: Option<String>,

    /// How to print the color
    #[arg(short, long, value_enum, default_value_t = Format::Hex)]
    format: Format,

    /// Okhsl saturation, from 0 to 1
    #[arg(short, long, default_value_t = 0.6, value_parser = unit_interval)]
    saturation: f32,

    /// Okhsl lightness, from 0 to 1
    #[arg(short, long, default_value_t = 0.65, value_parser = unit_interval)]
    lightness: f32,
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    /// `#rrggbb`
    Hex,
    /// ANSI escape setting the foreground
    Fg,
    /// ANSI escape setting the background
    Bg,
}

fn unit_interval(value: &str) -> Result<f32, String> {
    match value.parse() {
        Ok(number) if (0.0..=1.0).contains(&number) => Ok(number),
        _ => Err(format!("`{value}` is not a number from 0 to 1")),
    }
}

/// The seed's FNV-1a hash picks the hue; Okhsl keeps every hue equally vivid.
fn color(seed: &str, saturation: f32, lightness: f32) -> (u8, u8, u8) {
    let mut hasher = FnvHasher::default();
    hasher.write(seed.as_bytes());

    let hue = hasher.finish() as f32 / u64::MAX as f32 * 360.0;
    let rgb: Srgb<u8> = Srgb::from_color(Okhsl::new(hue, saturation, lightness)).into_format();
    rgb.into_components()
}

fn main() {
    let cli = Cli::parse();
    let seed = cli
        .seed
        .unwrap_or_else(|| gethostname::gethostname().to_string_lossy().into_owned());

    let (r, g, b) = color(&seed, cli.saturation, cli.lightness);
    match cli.format {
        Format::Hex => print!("#{r:02X}{g:02X}{b:02X}"),
        Format::Fg => print!("\x1b[38;2;{r};{g};{b}m"),
        Format::Bg => print!("\x1b[48;2;{r};{g};{b}m"),
    }
}
