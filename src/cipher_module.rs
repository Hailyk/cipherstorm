// created by HailyK on 26/3/2024
// cipher module used for generating hashes with configurable options

#[derive(Clone, Copy)]
pub enum Algorithm {
    Md5,
}
pub struct Cipher {
    pub(crate) algorithm: Algorithm,
}

impl Cipher {
    pub fn generate_hash(&self, data: &str) -> String {
        match self.algorithm {
            Algorithm::Md5 => {
                let data_bytes = data.as_bytes();
                let digest = md5::compute(data_bytes);
                format!("{:x}", digest)
            }
        }
    }
}
