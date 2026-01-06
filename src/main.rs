use log::{error, info};
use std::{env, fs, time::Instant};

#[derive(Default, Debug, Clone)]
struct FreqKV {
    key: String,
    value: usize,
    occupied: bool,
}

#[derive(Default)]
struct FreqKVs {
    items: Vec<FreqKV>,
    count: usize,
    capacity: usize,
}

// using option because it is possible to find None
fn find_key<'a>(hay_stack: &'a mut FreqKVs, needle: &str) -> Option<&'a mut FreqKV> {
    for i in 0..hay_stack.count {
        if hay_stack.items[i].key == needle {
            return Some(&mut hay_stack.items[i]); // return answer if found
        }
    }

    None // Return None if not found
}

fn naive_analysis(content: &str) {
    let start = Instant::now();

    let mut freq = FreqKVs::default();
    let content = content.trim_start();
    let token = content.split_whitespace();

    for t in token {
        let kv = find_key(&mut freq, t);
        if let Some(kv) = kv {
            kv.value += 1;
        } else {
            push_item_to_array(
                &mut freq,
                FreqKV {
                    key: t.to_string(),
                    value: 1,
                    occupied: true,
                },
            );
        }
    }

    let duration = start.elapsed();

    info!(" Tokens: {}", &freq.count);

    freq.items.sort_by(|a, b| b.value.cmp(&a.value));

    info!(" Top 10 tokens");
    for (i, kv) in freq.items.iter().take(10).enumerate() {
        info!("   {}: {} => {}", i, kv.key, kv.value);
    }

    info!(" Elapsed time {:.4}s", duration.as_secs_f32());
}

fn push_item_to_array(freq: &mut FreqKVs, new_kv: FreqKV) {
    freq.items.push(new_kv);
    freq.count += 1;
    freq.capacity = freq.items.capacity();
}

// Thinking: must I pass the buf_size?
// Maybe the buf_size allows to use sub-buffers in the buffer.
// Is this good design?
fn hash(buf: &[u8], buf_size: usize) -> u32 {
    let mut result: u32 = 0;
    for byte in buf.iter().take(buf_size) {
        (result, _) = result.overflowing_mul(13);
        result += *byte as u32;
    }

    result
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("No input is provided");
        info!("Usage: cargo run <input.txt>");
        panic!();
    }

    let file_path = &args[1];
    let buffer = fs::read(file_path).expect("unable to read file into bytes");

    info!("Analysing {}", &file_path);
    info!(" Size: {} bytes", &buffer.len());

    // convert buffer to string view
    let buffer = String::from_utf8(buffer).expect("unable to convert buffer to string");
    let content: &str = &buffer;

    let content = content.trim_start();
    let token = content.split_whitespace();

    const N: usize = 1000;
    let mut slots = vec![FreqKV::default(); N]; // !NOTE: vec allocation 

    info!(" Just Tokens: ");
    for (i, t) in token.take(100).enumerate() {
        let h = hash(t.as_bytes(), t.len()) % N as u32;
        info!("   {}: 0x{:08X} = {}", i, h, t);

        if slots[h as usize].occupied {
            if slots[h as usize].key == t {
                slots[h as usize].value += 1;
            } else {
                info!("  Collided 0x{:08X} at {}", h, i);
                break;
            }
        } else {
            slots[h as usize].occupied = true;
            slots[h as usize].key = t.to_string();
            slots[h as usize].value = 1;
        }
    }

    // naive_analysis(content);
}
