#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref ALPHABET_MAP: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('a', 1);
        m.insert('b', 2);
        m.insert('c', 3);
        m.insert('d', 4);
        m.insert('e', 5);
        m.insert('f', 6);
        m.insert('g', 7);
        m.insert('h', 8);
        m.insert('i', 9);
        m.insert('j', 10);
        m.insert('k', 11);
        m.insert('l', 12);
        m.insert('m', 13);
        m.insert('n', 14);
        m.insert('o', 15);
        m.insert('p', 16);
        m.insert('q', 17);
        m.insert('r', 18);
        m.insert('s', 19);
        m.insert('t', 20);
        m.insert('u', 21);
        m.insert('v', 22);
        m.insert('w', 23);
        m.insert('x', 24);
        m.insert('y', 25);
        m.insert('z', 26);
        m
    };
}

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn invert_key(key: Vec<char>) -> Vec<char> {
    let mut inverting_key = ALPHABET.clone().to_vec();

    for x in 0..key.len() {
        let i = ALPHABET_MAP.get(&key[x]).unwrap() - 1;
        inverting_key[i as usize] = ALPHABET[x];
    }

    return inverting_key;
}

pub fn encipher(key: Vec<char>, file_contents: String) -> String {
    let mut subsituting_file_contents: Vec<char> = file_contents.chars().collect();

    for (index, character) in file_contents.chars().enumerate() {
        if ALPHABET_MAP.contains_key(&character) {
            let i = ALPHABET_MAP.get(&character).unwrap() - 1;
            subsituting_file_contents[index] = key[i as usize];
        }
    }

    return subsituting_file_contents.iter().collect();
}

#[cfg(test)]
mod tests;
