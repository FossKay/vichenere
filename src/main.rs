use std::io::{BufRead, Write};

use itertools::Itertools;

const ABC: &str = "abcdefghijklmnopqrstuvwxyz"; //"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ABC_LEN: usize = 26;

fn index_abc(x: char) -> usize {
    ABC.chars().position(|n| n == x).unwrap()
}
fn get_abc(x: usize) -> char {
    ABC.chars().nth(x).unwrap()
}

fn encrypt(plain: &str, key: &str) -> String {
    let mut res = String::new();
    let mut i = 0;
    for char in plain.chars() {
        let key_char = key.chars().nth(i).unwrap();
        i += 1;
        i %= key.len();
        res.push(get_abc((index_abc(char) + index_abc(key_char)) % ABC_LEN));
    }
    res
}

fn decrypt(plain: &str, key: &str) -> String {
    let mut res = String::new();
    let mut i = 0;
    for char in plain.chars() {
        let key_char = key.chars().nth(i).unwrap();
        i += 1;
        i %= key.len();
        res.push(get_abc((index_abc(char) - index_abc(key_char) + 26) % ABC_LEN));
    }
    res
}

fn main() {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let mut message = String::new();
    let mut key = String::new();
    print!("Message: ");
    stdout.flush().unwrap();
    stdin.lock().read_line(&mut message).unwrap();
    print!("Clé: ");
    stdout.flush().unwrap();
    stdin.lock().read_line(&mut key).unwrap();
    message = message.trim_end().to_lowercase();
    key = key.trim_end().to_lowercase();
    let encrypted = encrypt(&message, &key);
    println!("Message encrypté: {encrypted}");

    let key_size = key.len();

    for combo in (0..key_size)
        .map(|_| ABC.chars())
        .multi_cartesian_product() {
            let s: String = combo.iter().collect();
            let result: String = decrypt(&encrypted, &s);
            //println!("{result}")
        }

    println!("Finished")
}
