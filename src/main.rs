use std::io::{BufRead, Write};

const ABC: &str = "abcdefghijklmnopqrstuvwxyz"; //"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ABC_LEN: u8 = 26;
const ABC_LEN_USIZE: usize = ABC_LEN as usize;

// FROM WIKIPEDIA
const FREQS: [f32; ABC_LEN_USIZE] = [8.4, 1.06, 3.03, 4.18, 17.26, 1.12, 1.27, 0.92, 7.34, 0.31, 0.05, 6.01, 2.96, 7.13, 5.26, 3.01,0.99, 6.55, 8.08, 7.07, 5.74, 1.32, 0.04, 0.45, 0.3, 0.12];

fn index_abc(x: char) -> u8 {
    ABC.chars().position(|n| n == x).unwrap() as u8
}

fn get_abc(x: u8) -> char {
    ABC.chars().nth(x as usize).unwrap()
}

fn get_score(x: &[u8]) -> f32 {
    x.iter().map(|&x| FREQS[x as usize]).sum()
}

fn encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let key_len = key.len();
    for (i, char) in plain.iter().enumerate() {
        res.push((char + key[i % key_len]) % ABC_LEN);
    }
    res
}

fn decrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let key_len = key.len();
    for (i, char) in plain.iter().enumerate() {
        // negative modulo is weird
        // added 26 so that it stays positive
        // and doesn't overflow (u8)
        res.push((char + 26 - key[i % key_len]) % ABC_LEN);
    }
    res
}

fn to_vec(v: &str) -> Vec<u8> {
    v.chars().map(|c| index_abc(c)).collect()
}

fn to_str(v: &[u8]) -> String {
    v.into_iter().map(|&x| get_abc(x)).collect()
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

    let msg: Vec<u8> = to_vec(&message.trim().to_lowercase());
    let key: Vec<u8> = to_vec(&key.trim().to_lowercase());
    let encrypted = encrypt(&msg, &key);
    println!("Message encrypté: {}", to_str(&encrypted));

    let key_size = key.len();
    let mut key_chars: Vec<Vec<(u8, f32)>> = vec![Vec::new(); key_size];
    let encr_len = encrypted.len();
    let mut new_chars: Vec<Vec<u8>> = Vec::new();

    assert!(key_size <= encr_len - 1); // for chars count

    for key_index in 0..key_size {
        for key_char in 0..ABC_LEN {
            // if the key isn't random, it counts too
            let mut score = FREQS[key_char as usize];
            for c in (key_index..encr_len).step_by(key_size) {
                score += FREQS[((encrypted[c] + 26 - key_char) % ABC_LEN) as usize];
            }
            let chars_count = (encr_len - key_index) / key_size;
            score /= chars_count as f32;
            key_chars[key_index].push((key_char, score));
        }
        // f32 doesn't implement Ord... can't compare :(
        key_chars[key_index].sort_by_key(|(_, y)| (y * 100_000.0) as u32);
        new_chars.push(key_chars[key_index].iter().copied().rev().map(|(a,_)| a).collect());
    }

    for key_index in 0..key_size {
        if !new_chars[key_index].contains(&key[key_index]) {
            println!("Key index #{key_index}")
        }
        println!("{:?}", new_chars[key_index].iter().map(|&x| get_abc(x)).collect::<String>());
    }
    //let decrypted = decrypt(&encrypted, &key_chars);
    //println!("Clé retrouvée: {}", to_str(&key_chars));
    //println!("Message décrypté: {}", to_str(&decrypted));
    println!("Fini!");
}
