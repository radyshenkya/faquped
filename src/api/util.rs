use crypto::{sha2::Sha512, digest::Digest};

pub fn hash_password(password: &str) -> String {
    let mut alg = Sha512::new();

    alg.input_str(password);
    alg.result_str()
}
