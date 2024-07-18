use std::sync::{Arc, Mutex};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{aster_score, ioc, match_percentage, substitution_cipher_score, vigenere_decrypt};


pub fn vigenere_hawk(
    keyword1: &str,
    encrypted_text: &str,
    plaintext: &str,
    key_length: usize,
) -> String {
    let mut best_score = 0.0;
    let mut best_keyword2 = String::new();
    let mut best_decrypted = String::new();
    let mut keyword2: Vec<char> = vec!['A'; key_length];

    for _ in 0..2 {
        for i in 0..key_length {
            let mut best_char = keyword2[i];

            for index in 'A'..='Z' {
                keyword2[i] = index;
                let decrypted = vigenere_decrypt(encrypted_text, keyword1, Some(&keyword2.iter().collect::<String>()));
                let score = aster_score(plaintext, &decrypted);

                if score > best_score {
                    best_score = score;
                    best_decrypted = decrypted;
                    best_char = index;
                }
            }

            keyword2[i] = best_char;
        }
    }
    best_keyword2 = keyword2.iter().collect();

    format!("Vigenere HAWK Analysis \n\nBestScore: {}\nBest Keyword: {}\nDecrypted: {}", best_score, best_keyword2, best_decrypted)
}

pub fn vigenere_wolf(
    key_length1: usize,
    key_length2: usize,
    encrypted_text: &str,
    plaintext: &str,
    permutations: usize,
) -> String {
    let alphabet: [char; 26] = [
        'Z', 'Y', 'X', 'W', 'V', 'U', 'T', 'S', 'R', 'Q', 'P', 'O', 'N',
        'M', 'L', 'K', 'J', 'I', 'H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'
    ];

    let mut keyword1: Vec<char> = alphabet[..key_length1].to_vec();

    let mut best_score = 0.0;
    let mut best_keyword1 = String::new();
    let mut best_keyword2 = String::new();

    for _ in 0..10 {  // Increased iterations for better search
        for index in 0..key_length1 {
            let mut best_char = keyword1[index];
            for &i in alphabet.iter() {
                keyword1[index] = i;
                let (keyword2, score) = remora_vigenere(&keyword1.iter().collect::<String>(), encrypted_text, plaintext, key_length2);
                if score > best_score {
                    best_score = score;
                    best_keyword1 = keyword1.iter().collect();
                    best_keyword2 = keyword2;
                    best_char = i;
                }
            }
            keyword1[index] = best_char;
        }
    }

    println!("Post-processing permutations of best_keyword1: {:?}", best_keyword1);
    let best_keyword1_chars: Vec<char> = best_keyword1.chars().collect();
    let best_permutation_score = Arc::new(Mutex::new(best_score));
    let best_permutation_keyword1 = Arc::new(Mutex::new(best_keyword1.clone()));
    let best_permutation_keyword2 = Arc::new(Mutex::new(best_keyword2.clone()));

    let permutations: Vec<_> = best_keyword1_chars.iter().permutations(best_keyword1.len()).take(permutations).collect();
    permutations.par_iter().for_each(|permutation| {
        let perm_keyword: String = permutation.iter().map(|&&c| c).collect();
        let (keyword2, score) = remora_vigenere(&perm_keyword, encrypted_text, plaintext, key_length2);
        
        let mut best_score = best_permutation_score.lock().unwrap();
        let mut best_keyword1 = best_permutation_keyword1.lock().unwrap();
        let mut best_keyword2 = best_permutation_keyword2.lock().unwrap();

        if score > *best_score {
            *best_score = score;
            *best_keyword1 = perm_keyword.clone();
            *best_keyword2 = keyword2.clone();
        }
    });

    let final_best_keyword1 = best_permutation_keyword1.lock().unwrap().clone();
    let final_best_keyword2 = best_permutation_keyword2.lock().unwrap().clone();
    let final_best_score = *best_permutation_score.lock().unwrap();

    let final_decryption = vigenere_decrypt(encrypted_text, &final_best_keyword1, Some(&final_best_keyword2));

    format!(
        "Best permutation keyword1: {}, keyword2: {}, Score: {}\nFinal BEST KEYWORD1: {:?}\nFinal BEST KEYWORD2: {}\n{}",
        final_best_keyword1, final_best_keyword2, final_best_score, final_best_keyword1, final_best_keyword2, final_decryption
    )
}

pub fn remora_vigenere(
    keyword1: &str,
    encrypted_text: &str,
    plaintext: &str,
    key_length: usize,
) -> (String, f64) {
    let mut best_score = 0.0;
    let mut best_keyword2 = String::new();
    let mut keyword2: Vec<char> = vec!['A'; key_length];

    for _ in 0..5 {  // Increased iterations for better search
        for i in 0..key_length {
            let mut best_char = keyword2[i];

            for index in 'A'..='Z' {
                keyword2[i] = index;
                let decrypted = vigenere_decrypt(encrypted_text, keyword1, Some(&keyword2.iter().collect::<String>()));
                let score = aster_score(plaintext, &decrypted);
                if score > best_score {
                    best_score = score;
                    best_char = index;
                }
            }

            keyword2[i] = best_char;
        }
    }
    best_keyword2 = keyword2.iter().collect();
    (best_keyword2, best_score)
}