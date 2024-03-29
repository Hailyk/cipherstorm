// Created by HailyK on 26/3/2024
// single threaded cpu cracking

use crate::cipher_module;
use regex::Regex;
use std::process::exit;
use std::time;

pub struct CrackResult {
    // hash that was cracked
    pub(crate) hash: String,
    // plain text of the hash
    pub(crate) plain_text: String,
    // time taken to crack the hash
    pub(crate) time_taken: time::Duration,
}

static CHARLIST: &str =
    "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789!@#$%^&*()_+-=[]{}|\";:,.<>?/";

pub fn crack_single_cpu(hash_list: &Vec<String>, password_length: u32, algorithm: &String) -> Vec<CrackResult> {
    println!("Cracking hash using single thread");

    // create result object
    let mut result = Vec::new();

    // check if algorithm is empty
    if algorithm == "" {
        println!("Algorithm not provided. Exiting...");
        exit(-1);
    }

    // check if algorithm is valid
    let mut algorithm_selected = cipher_module::Algorithm::Md5;
    match algorithm.as_str() {
        "md5" => {
            println!("md5 selected");
            algorithm_selected = cipher_module::Algorithm::Md5;
        }
        _ => {
            println!("Algorithm not supported. Exiting...");
            exit(-1)
        }
    }

    // check if hash list is empty
    if hash_list.len() == 0 {
        println!("No hash to crack. Exiting...");
        exit(-1)
    }

    // check if first hash is empty
    if hash_list[0] == "" {
        println!("Hash is empty. Exiting...");
        exit(-1)
    }

    // check if password length is less than 1
    if password_length < 1 {
        println!("Password length should be greater than 0. Exiting...");
        exit(-1)
    }

    // check for hash type pattern match
    match algorithm_selected {
        cipher_module::Algorithm::Md5 => {
            // check if all hash match pattern
            for hash in hash_list {
                let re = Regex::new(r"^[a-f0-9]{32}$").unwrap();
                if !re.is_match(&hash) {
                    println!("Hash is not md5. Hash: {}", hash_list[0]);
                    exit(255)
                }
            }
        }
    }

    // setup timer
    let start_time = time::Instant::now();
    
    // commence cracking
    let mut password_hash: String = String::new();
    let mut attempt_password: String = String::from("");
    while attempt_password.len() <= password_length as usize {
        // generate hash
        let hash = cipher_module::Cipher {
            algorithm: algorithm_selected,
        };
        password_hash = hash.generate_hash(&attempt_password);

        // check if hash is in hash list
        for hash_in_hashes in hash_list {
            if password_hash.eq(hash_in_hashes) {
                let time_taken = start_time.elapsed();
                
                println!("found: {}:{}  time:{}", attempt_password, password_hash, time_taken.as_secs());
                // hash match
                result.push(CrackResult {
                    hash: hash_in_hashes.clone(),
                    plain_text: attempt_password.clone(),
                    time_taken: time_taken,
                });
            }
        }

        // generate next password
        attempt_password = genrate_next_password(&String::from(CHARLIST), attempt_password.clone());
    }

    return result;
}

fn genrate_next_password(chars_list: &String, mut password: String) -> String {
    let mut new_password: String = String::new();
    
    // check if password is empty
    if password == "" {
        return chars_list.chars().nth(0).unwrap().to_string();
    }
    
    // loop through the password from the last char to the first char
    let mut index = password.len() - 1;
    while index >= 0 {
        let current_char = password.chars().nth(index).unwrap();
        let next_char = increment_char(chars_list, current_char);

        if next_char == chars_list.chars().nth(0).unwrap() {
            // increment char before current char
            new_password = next_char.to_string() + &new_password;
            if index == 0 {
                // add new char to the password
                new_password = next_char.to_string() + &new_password;
                break;
            }
            index -= 1;
        } else {
            // set the fist char in string to the next char
            new_password = next_char.to_string() + &new_password;
            break;
        }
    }
    
    // replace the necessary portion of the password with the new password
    let new_password_length = new_password.len() as isize - 1;
    let password_length = password.len() as isize - 1;
    if (((password.len() as isize) - 1) - (new_password_length)) < 0 {
        password.replace_range(0.., &new_password);
    } else {
        password.replace_range((password_length - new_password_length) as usize.., &new_password);
    }
    
    return password;
}

/// increment the char by one, if the char is the last char in the list, then increment the char
/// before it and set the current char to the first char in the list
/// @param list: String, 1 char list
/// @param current_char: char
fn increment_char(list: &String, current_char: char) -> char {
    let index_found = list.find(current_char);

    if index_found == None {
        // char not found in list
        println!("Char not found in list. Exiting...");
        exit(-1);
    }

    let mut index = index_found.unwrap();

    if index == list.len() - 1 {
        // char is the last char in the list
        index = 0;
    } else {
        // increment the char
        index += 1;
    }

    return list.chars().nth(index).unwrap();
}
