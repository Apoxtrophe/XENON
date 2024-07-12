pub fn vigenere_encrypt(plaintext: &str, key1: &str, key2: Option<&str>) -> String {
    let key1 = generate_key(key1, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let plaintext = plaintext.to_uppercase();
    
    if let Some(key2) = key2 {
        let key2 = generate_key(key2, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let table = generate_vigenere_table(&key1, &key2);
        plaintext.chars().enumerate().filter_map(|(index, plain_char)| {
            table[0].iter().position(|&c| c == plain_char)
                .map(|position| table[(index % key2.len()) + 1][position])
        }).collect()
    } else {
        let key1 = key1.chars().cycle();
        plaintext.chars().zip(key1).map(|(p, k)| {
            let p = p as u8 - b'A';
            let k = k as u8 - b'A';
            (b'A' + (p + k) % 26) as char
        }).collect()
    }
}

pub fn vigenere_decrypt(ciphertext: &str, key1: &str, key2: Option<&str>) -> String {
    let key1 = generate_key(key1, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let ciphertext = ciphertext.to_uppercase();
    
    if let Some(key2) = key2 {
        let key2 = generate_key(key2, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let table = generate_vigenere_table(&key1, &key2);
        ciphertext.chars().enumerate().filter_map(|(index, encrypted_char)| {
            let row = &table[(index % key2.len()) + 1];
            row.iter().position(|&c| c == encrypted_char)
                .map(|position| table[0][position])
        }).collect()
    } else {
        let key1 = key1.chars().cycle();
        ciphertext.chars().zip(key1).map(|(c, k)| {
            let c = c as u8 - b'A';
            let k = k as u8 - b'A';
            (b'A' + (c + 26 - k) % 26) as char
        }).collect()
    }
}
pub fn atbash_transform(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_lowercase() {
                    (b'z' - (c as u8 - b'a')) as char
                } else {
                    (b'Z' - (c as u8 - b'A')) as char
                }
            } else {
                c
            }
        })
        .collect()
}

pub fn generate_vigenere_table(key1: &str, key2: &str) -> Vec<Vec<char>> {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let key1: Vec<char> = key1.to_uppercase().chars().collect();
    let key2: Vec<char> = key2.to_uppercase().chars().collect();

    let filtered_alphabet: Vec<char> = alphabet.iter().filter(|&&c| !key1.contains(&c)).cloned().collect();
    let combined_alphabet: Vec<char> = [key1.as_slice(), filtered_alphabet.as_slice()].concat();
    
    let mut table: Vec<Vec<char>> = vec![combined_alphabet.clone()];
    for &c in key2.iter() {
        let mut index = combined_alphabet.iter().position(|&x| x == c).unwrap();
        let row: Vec<char> = (0..26).map(|_| {
            let ch = combined_alphabet[index % 26];
            index += 1;
            ch
        }).collect();
        table.push(row);
    }
    table
}

pub fn generate_key(key: &str, default: &str) -> String {
    if key.is_empty() { default.to_string() } else { key.to_string() }
}
