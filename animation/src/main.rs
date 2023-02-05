mod encoder;
mod frames;
use clap::{builder::PossibleValuesParser, value_parser, Arg, Command};

fn main() {
    let matches = Command::new("Countdown animation")
        .arg(
            Arg::new("countdown")
                .short('c')
                .long("countdown")
                .value_name("u32")
                .help("Set the countdown time in seconds")
                .required(true)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .value_name("u32")
                .help("Sets the width of the animation")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("height")
                .short('a')
                .long("height")
                .value_name("u32")
                .help("Sets the height of the animation")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("format")
                .short('o')
                .long("format")
                .help("Sets the output format of the animation (webp, mp4, gif)")
                .value_parser(PossibleValuesParser::new(["webp", "mp4", "gif"])),
        )
        .get_matches();

    let default_format = "mp4".to_string();

    let countdown = matches.get_one::<u32>("countdown").unwrap_or(&30);
    let width = matches.get_one::<u32>("width").unwrap_or(&400);
    let height = matches.get_one::<u32>("height").unwrap_or(&300);
    let str_format = matches
        .get_one::<String>("format")
        .unwrap_or(&default_format);

    let format = encoder::Format::new(str_format, countdown.clone(), width.clone(), height.clone());

    format.process_file();
}
