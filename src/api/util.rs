use crypto::{sha2::Sha512, digest::Digest};

pub fn hash_password(password: &str) -> String {
    let mut alg = Sha512::new();

    alg.input_str(password);
    alg.result_str()
}

#[allow(unused)]
pub fn display_serialize<T, S>(x: &T, s: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: serde::Serializer,
{
    s.collect_str(x)
}
