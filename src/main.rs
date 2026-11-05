use std::io::{BufRead, Write};

use itertools::Itertools;

const ABC: &str = "abcdefghijklmnopqrstuvwxyz"; //"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ABC_LEN: u8 = 26;

fn index_abc(x: char) -> u8 {
    ABC.chars().position(|n| n == x).unwrap() as u8
}
fn get_abc(x: u8) -> char {
    ABC.chars().nth(x as usize).unwrap()
}

fn encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut i = 0;
    for char in plain {
        res.push((char + key[i]) % ABC_LEN);
        i += 1;
        i %= key.len();
    }
    res
}

fn decrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut i = 0;
    for char in plain {
        res.push((char - key[i] + 26) % ABC_LEN);
        i += 1;
        i %= key.len();
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

    let msg: Vec<u8> = message.trim().to_lowercase().chars().map(|c| index_abc(c)).collect();
    let key: Vec<u8> = key.trim().to_lowercase().chars().map(|c| index_abc(c)).collect();
    let encrypted = encrypt(&msg, &key);
    println!("Message encrypté: {encrypted:?}");

    let key_size = key.len();

    for combo in (0..key_size)
        .map(|_| 0..ABC_LEN)
        .multi_cartesian_product() {
            let result: String = decrypt(&encrypted, &combo).into_iter().map(|x| get_abc(x)).collect();
            println!("{result}")
        }

    println!("Fini!")
}
