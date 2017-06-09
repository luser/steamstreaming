use errors::*;
use openssl::symm::{self, Cipher, Crypter, Mode};

/// Encrypt or decrypt `iv` using AES-256-ECB with `key` as the key.
fn crypt_iv(key: &[u8], iv: &[u8], mode: Mode) -> Result<Vec<u8>> {
    let t = Cipher::aes_256_ecb();
    let mut c = Crypter::new(t, mode, key, None)?;
    c.pad(false);
    let mut out = vec![0; iv.len() + t.block_size()];
    let count = c.update(iv, &mut out)?;
    let rest = c.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}

/// Encrypt `data` using AES-256-CBC with `key` as the key and `iv` as the initialization vector.
pub fn symmetric_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    // First, encrypt the IV.
    let mut out = crypt_iv(key, iv, Mode::Encrypt)?;
    // Then encrypt the data.
    let crypt_data = symm::encrypt(Cipher::aes_256_cbc(), key, Some(&iv), data)?;
    out.extend(crypt_data);
    Ok(out)
}

/// Decrypt `data` using AES-256-CBC with `key` as the key. The first 16 bytes of `data`
/// will be used as the (encrypted) initialization vector.
pub fn symmetric_decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // Get the IV.
    const IV_LEN: usize = 16;
    let (iv_in, data) = data.split_at(IV_LEN);
    let iv = crypt_iv(key, iv_in, Mode::Decrypt)?;
    let plain_data = symm::decrypt(Cipher::aes_256_cbc(), key, Some(&iv), &data)?;
    Ok(plain_data)
}
