use hmac::{Hmac, Mac};
use sha2::Sha256;

pub async fn hmac_sha256(key: &[u8], contents: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
    mac.update(contents);
    Ok(mac.finalize().into_bytes().to_vec())
}
