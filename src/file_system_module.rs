// Created by HailyK on 26/3/2024
// file system module used for reading and writing files

use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

// save the file for this instance of FileSystem
pub struct FileSystem {
    file: Result<File, Error>,
}

impl FileSystem {
    /// create new instance of FileSystem
    /// @constructor
    /// @param file_path: &str
    pub fn new(file_path: &str) -> FileSystem {
        let path = Path::new(file_path);
        let file = File::open(path);

        if file.is_err() {
            println!("Error opening file: {}", file.err().unwrap());
            println!("Exiting...");
            std::process::exit(1);
        }

        return FileSystem { file };
    }

    /// Read file as vector seperated by deliminator
    /// @param deliminator: &str
    /// @return Vec<String>
    pub fn read_as_vector(&self, deliminator: &str) -> Vec<String> {
        // vector for storying all the hashes in file
        let mut result: Vec<String> = Vec::new();

        // data read from file
        let mut data = String::new();

        // read data from file
        let _ = &self.file.as_ref().unwrap().read_to_string(&mut data);

        // split data by deliminator
        let data = data.split(deliminator);

        // store data in result vector
        for item in data {
            result.push(item.to_string());
        }

        // return the result
        result
    }
}
