// Created by HailyK on 26/3/2024
// file system module used for reading and writing files

use std::fs::File;
use std::io::Read;
use std::path::Path;

// save the file for this instance of FileSystem
pub struct FileSystem {
    file: File,
}

impl FileSystem {
    /// create new instance of FileSystem
    /// @constructor
    /// @param file_path: &str
    /// @return Result<FileSystem, &'static str>
    pub fn new(file_path: &str) -> Result<FileSystem, &'static str> {
        let path = Path::new(file_path);
        let file = File::open(path).unwrap();
        
        Ok(FileSystem {
            file,
        })
    }

    /// Read file as vector seperated by deliminator
    /// @param deliminator: &str
    /// @return Result<Vec<String>, Error>
    pub fn read_as_vector(&mut self, deliminator: &str) -> Result<Vec<String>, &'static str> {
        // vector for storying all the hashes in file
        let mut result: Vec<String> = Vec::new();

        // data read from file
        let mut data = String::new();

        // read data from file
        let _ = &self.file.read_to_string(&mut data).unwrap();

        // split data by deliminator
        let data = data.split(deliminator);

        // store data in result vector
        for item in data {
            result.push(item.to_string());
        }

        // return the result
        Ok(result)
    }
}
