// Created by HailyK on 26/3/2024
// entry point for the hash cracker

use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use crate::cracker::crack_manager;

mod ciphers;
mod cracker;
mod file_system_module;

static CHARLIST: &str =
    "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789!@#$%&*|?";

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
    for mut index in 0..args.len() {
        let first_char = args.get(index).unwrap().chars().next().unwrap();
        if first_char == '-' {
            // remove the '-' from the flag
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
    
    let mut hash_map: HashSet<String> = HashSet::new();
    for item in hash_list {
        if item.contains("\r") {
            let new_item = item.replace("\r", "");
            hash_map.insert(new_item);
        } else {
            hash_map.insert(item);
        }
    }

    // check for multi-threading flag
    let mut multi_thread = false;
    for flag in cracker_flags {
        if flag == "-m" {
            multi_thread = true;
        }
    }
    
    // convert password charset to vector
    let password_charset: Vec<String> = CHARLIST.chars().map(|c| c.to_string()).collect();
    
    // setup arc variables
    let hash_map = Arc::new(hash_map);
    let password_charset = Arc::new(password_charset);
    
    // call the cracker
    crack_manager(hash_map, password_charset, 8, ciphers::Algorithm::Md5,
                  multi_thread);
    
    
    
}