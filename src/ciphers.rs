// created by HailyK on 26/3/2024
// cipher module used for generating hashes with configurable options

use md5;

#[derive(Clone, Copy)]
pub enum Algorithm {
    Md5,
}

/// generate hash based on the algorithm
/// @param algorithm: Algorithm
/// @param data: &str
/// @return String
/// @public
pub fn generate_hash(algorithm: Algorithm, data: &str) -> String {
    match algorithm {
        Algorithm::Md5 => {
            let data_bytes = data.as_bytes();
            let digest = md5::compute(data_bytes);
            format!("{:x}", digest)
        }
    }
}
