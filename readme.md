# Hash Table

An educational and recreational attempt to program a hash table implementation using Rust.

For the hash function, I used two function:

- A basic hash function
- the djb2 hash function

> This is not for production purposes. Please use std::Hashmap.

The code can be tested with

```bash
cargo run -- t8.shakespeare.txt
```

This tests the code with the shakespeare text file and returns the frequency of words.

## Resources

- The project was inspired by Tsoding Daily's implementation available at: https://www.youtube.com/watch?v=n-S9DBwPGTo

- The shakespeare file is available at: https://ocw.mit.edu/ans7870/6/6.006/s08/lecturenotes/files/t8.shakespeare.txt
