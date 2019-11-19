pub(crate) const SEARCH_X: usize = 100;
pub(crate) const SEARCH_Y: usize = 100;

mod pause;
mod loader;
mod process;

use pause::pause;
use std::env;
use std::time::Instant;
use loader::loader;
use process::brute;



fn main() {
    // Word list
    let mut final_word_contents: Vec<String> = Vec::new();
    let mut search_arr = [['#';SEARCH_X];SEARCH_Y];
    let mut results: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let process_type = &args[1];
    let word_filename = &args[2];
    let search_filename = &args[3];

    println!("Word File: {} Search File: {}", word_filename.clone(), search_filename.clone());
    pause();
    let start = Instant::now();
    let max_word_length = loader(word_filename, search_filename, &mut final_word_contents, &mut search_arr);
    let duration = start.elapsed();
    println!("Load of the files took {:?}", duration);
    println!("Max Word Length is {}", max_word_length);
    

    if process_type == "brute" {
        results = brute(max_word_length, &final_word_contents, search_arr);
    } else {
        results.push("No Match".to_owned());
    }
    
    let duration = start.elapsed();
    let l = results.iter().len();
    for line in results {
        println!("{}",line);
    }
    println!("process file took {:?} for a total find of {}", duration, l);
   
    
}
