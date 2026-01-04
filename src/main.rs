use log::info;
use std::fs;

fn main() {
    env_logger::init();

    let file_path = "./t8.shakespeare.txt";
    let buffer = fs::read(file_path).expect("unable to read file into bytes");

    info!("Size of {} is {} bytes", &file_path, &buffer.len());

    // convert buffer to string view
    let buffer = String::from_utf8(buffer).expect("unable to convert buffer to string");
    let content: &str = &buffer;

    for token in content.split_whitespace() {
        info!("{:?}", token);
    }
}
