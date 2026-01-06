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

fn naive_analysis(content: &str, file_path: &str) {
    info!("Analysing {} using NAIVE METHOD", &file_path);
    info!(" Size: {} bytes", content.len());

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

fn hash_analysis(content: &str, file_path: &str) {
    info!("Analysing {} using HASH METHOD", &file_path);
    info!(" Size: {} bytes", content.len());

    let start = Instant::now();

    let content = content.trim_start();
    let token = content.split_whitespace();

    let mut ht: FreqKVs = Default::default();
    hash_init(&mut ht, 100_000); // !NOTE: vec allocation 

    info!(" Just Tokens: ");
    for (i, t) in token.enumerate() {
        let mut h = hash_djb2(t.as_bytes()) % ht.capacity as u32;
        // info!("   {}: 0x{:08X} = {}", i, h, t);

        // looking for a free hash that will not collide
        for _i in 0..ht.capacity {
            if ht.items[h as usize].occupied && ht.items[h as usize].key != t {
                h = (h + 1) % ht.capacity as u32;
            } else {
                break; // exit loop early
            }
        }

        if ht.items[h as usize].occupied {
            if ht.items[h as usize].key != t {
                error!("Table overflow");
                panic!();
            }
            ht.items[h as usize].value += 1;
        } else {
            ht.items[h as usize].occupied = true;
            ht.items[h as usize].key = t.to_string();
            ht.items[h as usize].value = 1;
        }
    }

    let duration = start.elapsed();

    let mut freq: FreqKVs = Default::default();

    info!(" Slots of the Hash Table:");
    for i in 0..ht.capacity {
        if ht.items[i].occupied {
            push_item_to_array(&mut freq, ht.items[i].clone()); // clone is acceptable here for performance
        }
    }

    // sort in place
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
fn hash(buf: &[u8]) -> u32 {
    let mut result: u32 = 0;
    for b in buf {
        (result, _) = result.overflowing_mul(31);
        result += *b as u32;
    }

    result
}

fn hash_djb2(buf: &[u8]) -> u32 {
    let mut result: u32 = 0;
    for b in buf {
        result = (result << 5).wrapping_add(result).wrapping_add(*b as u32);
    }

    result
}

fn hash_init(ht: &mut FreqKVs, capacity: usize) {
    ht.items = vec![FreqKV::default(); capacity];
    ht.count = 0;
    ht.capacity = capacity;
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

    // convert buffer to string view
    let buffer = String::from_utf8(buffer).expect("unable to convert buffer to string");
    let content: &str = &buffer;

    // naive_analysis(content, file_path);
    hash_analysis(content, file_path);
}
