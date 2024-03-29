// Created by HailyK on 27/3/2024
// cracker manager module used for managing the hash cracker, flags and hash list

use crate::cracker;
use crate::cracker::CrackResult;

pub struct CrackerManager {
    // flags for the cracker
    flags: Vec<String>,
    // hash list to crack
    hash_list: Vec<String>,
    // algorithm to use for cracking
    algorithm: String,
    // password length
    length: u32,
}

impl CrackerManager {
    /// create new instance of CrackerManager
    /// @constructor
    /// @param flags: Vec<String>
    /// @param hash_list: Vec<String>
    pub fn new(
        flags: Vec<String>,
        hash_list: Vec<String>,
        algorithm: &str,
        length: u32,
    ) -> CrackerManager {
        return CrackerManager {
            flags,
            hash_list,
            algorithm: String::from(algorithm),
            length,
        };
    }

    /// run the cracker
    /// @arg self: &CrackerManager
    /// @return Vec<CrackResult>
    pub fn run(&self) -> Vec<CrackResult> {
        // vector for storing cracked hashes
        let mut result: Vec<CrackResult> = Vec::new();

        if self.flags.contains(&String::from("-s")) {
            // run single threaded cracker
            result = cracker::crack_single_cpu(&self.hash_list, self.length, &self.algorithm);
            return result;
        } else {
            // run multi threaded cracker
            if !self.flags.contains(&String::from("-m")) {
                println!("No processor flag provided. defaulting to multi threaded cracker.");
            }
            println!("Multi threaded cracker not implemented yet. Exiting...");
            return result;
        }
    }

    // private helper function to crack hash
}
