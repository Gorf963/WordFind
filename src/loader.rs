use std::fs;
use crate::{SEARCH_X, SEARCH_Y};

pub fn loader(word_filename: &String, search_filename: &String, final_word_contents: &mut Vec<String>, search_arr: &mut [[char;SEARCH_X];SEARCH_Y]) -> usize
{
    let word_contents = fs::read_to_string(word_filename).expect("Something went wrong with file read");
    let mut max_word_length: usize = 0;

    for line in word_contents.lines() {
        let len = line.chars().count();
        if len > 2 {
            final_word_contents.push(line.to_lowercase());
            //final_word_contents.push(line.chars().rev().collect::<String>().to_lowercase());
            if max_word_length< len {
                max_word_length = len;
            }
        }
    }

    let search_contents = fs::read_to_string(search_filename).expect("Something went wrong with file read").to_lowercase();
    
    let mut i: usize = 0;
    for line in search_contents.lines() {
        let mut j: usize = 0;
        for c in line.chars() {
            search_arr[i][j]=c;
            j+=1;
        }
        i+=1;
    }

    return max_word_length;

}


