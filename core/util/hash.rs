use blake3::Hasher;
use common::AppResult;


pub fn hash_file(data: &[u8]) -> AppResult<String> {
    let mut hasher = Hasher::new();
    hasher.update(data);

    Ok(hasher.finalize().to_hex().to_string())
}
