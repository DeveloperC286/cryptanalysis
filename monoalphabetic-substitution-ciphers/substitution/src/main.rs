use std::env;
use std::process::exit;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let key = &args[2];

    println!("\nPerforming subsitution on context of file {}.", filename);
    println!("Using key {} for subsitution.\n", key);

    if key.len() != 26 {
        println!("The key must be of length 26.");
        exit(1);
    }

    let file_contents = fs::read_to_string(filename)
        .expect("Unable to read file."); 

    static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
    ];

    let key_vector:Vec<char> = key.chars().collect();
    let file_contents_lowercase = file_contents.clone().to_ascii_lowercase();
    let mut subsituting_file_contents:Vec<char>= file_contents_lowercase.chars().collect();

    for x in 0..26 { 
        println!("{} -> {}", ASCII_LOWER[x], key_vector[x]);
        
        for (index, character) in file_contents_lowercase.chars().enumerate() { 
            if character == ASCII_LOWER[x] {
                subsituting_file_contents[index] = key_vector[x];  
            }
        } 
    } 


    let subsituted_file_contents:String = subsituting_file_contents.iter().collect();

    println!("\n{}", subsituted_file_contents);

}
