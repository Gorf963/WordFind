mod pause;
mod loader;

use pause::pause;
use std::env;
use std::time::Instant;
use loader::loader;

fn main() {
    // Word list
    let mut final_word_contents: Vec<String> = Vec::new();


    let args: Vec<String> = env::args().collect();
    let word_filename = &args[1];
    let search_filename = &args[2];

    println!("Word File: {} Search File: {}", word_filename.clone(), search_filename.clone());
    pause();
    let start = Instant::now();
    loader(word_filename, search_filename, &mut final_word_contents);
    let duration = start.elapsed();
    println!("Load of the files took {:?}", duration);
    println!("File {} had 0 matches", word_filename);
}
