// Created by HailyK on 26/3/2024
// entry point for the hash cracker

use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use crate::cracker::crack_manager;

mod ciphers;
mod cracker;
mod file_system_module;

// password charset
static CHARLIST: &str =
    "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789!@#$%&*|?";

//global variables
static PASSWORD_LENGTH: usize = 8;

/// entry point for the hash cracker
fn main() {
    // get command line arguments
    let mut args: Vec<String> = env::args().collect();

    // check if there are any arguments
    if args.len() == 1 {
        println!("No hash input provided. Exiting...");
        return;
    }

    // remove the running file name from the arguments
    args.remove(0);

    // check for flags in arguments
    let mut cracker_flags: Vec<String> = Vec::new();
    for index in 0..args.len() {
        let first_char = args.get(index).unwrap().chars().next().unwrap();
        // check for the items in command line that has '-' as the first character
        if first_char == '-' {
            let flag = args.get(index).unwrap().clone();
            cracker_flags.push(flag);
        }
    }

    // ge the file path, which after removing all option should be at index 0;
    if args.get(0).is_none() {
        println!("Hash file not provided. exiting....");
        return;
    }

    //create read stream for hash file
    println!("Reading HashList from '{}'", args.get(0).unwrap());
    let mut file = file_system_module::FileSystem::new(args.get(0).unwrap()).unwrap();
    let hash_list = file.read_as_vector("\n").unwrap();
    
    let num_hashes = hash_list.len();
    println!("{} hash imported", num_hashes);
    
    //insert into hash map
    let mut hash_map: HashSet<String> = HashSet::new();
    for mut item in hash_list {
        // check for character return
        if item.contains("\r") {
            // remove the character return, specifically for windows
            item = item.replace("\r", "");
        }
        // insert the item into the hash map
        hash_map.insert(item);
    }

    // check for multi-threading flag
    let mut multi_thread = false;
    for flag in cracker_flags {
        if flag == "-m" {
            multi_thread = true;
        } else if flag == "-s" {
            multi_thread = false;
        }
    }
    
    // output current character set
    println!("Using character set: {}", CHARLIST);
    
    // convert password charset to vector
    let password_charset: Vec<String> = CHARLIST.chars().map(|c| c.to_string()).collect();
    
    // setup arc variables to allow multiple threads to access the same data
    let hash_map = Arc::new(hash_map);
    let password_charset = Arc::new(password_charset);
    
    // call the cracker
    crack_manager(hash_map, password_charset, PASSWORD_LENGTH, ciphers::Algorithm::Md5,
                  multi_thread);
    
    
    
}
