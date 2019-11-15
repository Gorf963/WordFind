use std::fs;

pub fn loader(word_filename: &String, search_filename: &String, final_word_contents: &mut Vec<String>)
{
    let word_contents = fs::read_to_string(word_filename).expect("Something went wrong with file read");
    let mut search_arr = vec![vec!['#';200];200];

    for line in word_contents.lines() {
        final_word_contents.push(line.to_lowercase());
        final_word_contents.push(line.chars().rev().collect::<String>().to_lowercase());
    }

    let search_contents = fs::read_to_string(search_filename).expect("Something went wrong with file read");
    
    let mut i: usize = 0;
    for line in search_contents.lines() {
        let mut j: usize = 0;
        for c in line.chars() {
            search_arr[i][j]=c;
            print!("{}",c);
            j+=1;
        }
        println!("");
        i+=1;
    }


}


