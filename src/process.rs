use crate::{SEARCH_X, SEARCH_Y};
use std::cmp;

pub fn brute(max_word_len: usize, word_list: &Vec<String>, search: [[char;SEARCH_X];SEARCH_Y]) -> Vec<String> {

    let mut temp_results: Vec<String> = Vec::new();
 
    for i in 0..SEARCH_X-1 {
        for j in 0..SEARCH_Y-1 {
            //West
            let word = word_create("W".to_owned(),i,j,max_word_len,search);
            for a in word_list.iter() {
                if start_with(&word, a ) {
                    temp_results.push(format!("West Word: {} found at {} {}",a,i+1,j+1));
                }
            }
            //East
            let word = word_create("E".to_owned(),i,j,max_word_len,search);
            for a in word_list.iter() {
                if start_with(&word, a ) {
                    temp_results.push(format!("East Word: {} found at {} {}",a,i+1,j+1));
                }
            }
            //South
            let word = word_create("S".to_owned(),i,j,max_word_len,search);
            for a in word_list.iter() {
                if start_with(&word, a ) {
                    temp_results.push(format!("South Word: {} found at {} {}",a,i+1,j+1));
                }
            }
            //North
            let word = word_create("N".to_owned(),i,j,max_word_len,search);
            for a in word_list.iter() {
                if start_with(&word, a ) {
                    temp_results.push(format!("North Word: {} found at {} {}",a,i+1,j+1));
                }
            }
        }
        
    }
    return temp_results;
}

fn word_create(direct: String, i: usize, j: usize, max_word_len: usize, search: [[char;SEARCH_X];SEARCH_Y]) ->String {
    let mut word = String::from("");
    if direct == "W" {
        for k in j..cmp::min(SEARCH_X-1, j+max_word_len) {
            word.push(search[i][k]);
        }
    }
    if direct == "E" {
        let mut adj_j: usize = 0;
        let mut k: usize = j;
        if max_word_len < j {adj_j = j-max_word_len;}
        while k > adj_j {
            word.push(search[i][k]);
            k -=1;
        }
    }
    if direct == "S" {
        for k in i..cmp::min(SEARCH_Y-1, i+max_word_len) {
            word.push(search[k][j]);
        }
    }
    if direct == "N" {
        let mut adj_i: usize = 0;
        let mut k: usize = i;
        if max_word_len < i {adj_i = i-max_word_len;}
        while k > adj_i {
            word.push(search[k][j]);
            k -=1;
        }
    }


    return word;
}
fn start_with(txt: &String, search: &String) -> bool {
    //let mut i :usize = 0;
    for (i,c) in search.chars().enumerate() {
        if i<txt.len(){
            if txt.chars().nth(i).unwrap() != c {
                return false;
            }
        } else {return false;}
    }
    return true;
}