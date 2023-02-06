#![feature(proc_macro_hygiene, decl_macro)]
use std::process::{ChildStdout, Command, Stdio};

use rocket::{
    http::{hyper::buffer::BufReader, ContentType},
    response::{Content, Stream},
};

#[macro_use]
extern crate rocket;

#[get("/countdown?<time>&<width>&<height>&<format>")]
fn index(
    time: u32,
    width: Option<u32>,
    height: Option<u32>,
    format: Option<String>,
) -> Content<Stream<BufReader<ChildStdout>>> {
    let file_format = format.unwrap_or_else(|| "mp4".to_string());

    let output = Command::new("/Users/edgravill/code/rust/countdown/target/release/animation")
        .arg("-c")
        .arg(time.to_string())
        .arg("-w")
        .arg(width.unwrap_or_else(|| 400).to_string())
        .arg("-a")
        .arg(height.unwrap_or_else(|| 300).to_string())
        .arg("-o")
        .arg(file_format.to_string())
        .stdout(Stdio::piped())
        .spawn();

    let reader = BufReader::new(output.unwrap().stdout.unwrap());

    match &file_format.as_str() {
        &"webp" => return Content(ContentType::WEBP, Stream::chunked(reader, 10)),
        &"gif" => return Content(ContentType::GIF, Stream::chunked(reader, 10)),
        _ => return Content(ContentType::MP4, Stream::chunked(reader, 10)),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
