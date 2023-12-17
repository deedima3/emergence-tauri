use aes::cipher::block_padding::{Pkcs7, UnpadError};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::typenum::U32;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use hkdf::Hkdf;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;

use crate::error::{BackendError, BackendResult};

// AES-256-CBC
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub fn aes256_encrypt(key: GenericArray<u8, U32>, msg: &[u8]) -> Vec<u8> {
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut iv);

    let mut ct = Aes256CbcEnc::new(&key, &iv.into()).encrypt_padded_vec_mut::<Pkcs7>(msg);

    ct.extend_from_slice(&iv);
    ct
}

pub fn aes256_decrypt(key: GenericArray<u8, U32>, msg: &[u8]) -> Result<Vec<u8>, UnpadError> {
    let (ct, iv) = msg.split_at(msg.len() - 16);

    Aes256CbcDec::new(&key, iv.into()).decrypt_padded_vec_mut::<Pkcs7>(ct)
}

pub fn expand_secret_key(
    secret: Vec<u8>,
) -> Result<(GenericArray<u8, U32>, GenericArray<u8, U32>), BackendError> {
    let key = Hkdf::<Sha256>::new(Some(b""), &secret);

    let mut a_key = [0u8; 32];
    let mut b_key = [0u8; 32];

    Ok((
        match Hkdf::expand(&key, b"pattern_key", &mut a_key) {
            Ok(_) => GenericArray::clone_from_slice(&a_key),
            Err(e) => return Err(BackendError::GenericError(e.to_string())),
        },
        match Hkdf::expand(&key, b"apattern_key", &mut b_key) {
            Ok(_) => GenericArray::clone_from_slice(&b_key),
            Err(e) => return Err(BackendError::GenericError(e.to_string())),
        },
    ))
}

pub fn generate_initial_secret_key(
    secret: Vec<u8>,
) -> BackendResult<Vec<u8>, BackendError> {
    let key = Hkdf::<Sha256>::new(Some(b""), &secret);

    let mut sk = [0u8; 32];

    match Hkdf::expand(&key, b"initial_secret", &mut sk) {
        Ok(_) => Ok(sk.to_vec()),
        Err(e) => Err(BackendError::GenericError(format!("{e}"))),
    }
}
pub fn hash_password(pass: String) -> Result<String, BackendError> {
    let salt = SaltString::generate(&mut OsRng);
    let ag = Argon2::default();

    match ag.hash_password(pass.as_bytes(), &salt) {
        Ok(h) => Ok(h.to_string()),
        Err(e) => Err(BackendError::GenericError(e.to_string())),
    }
}

pub fn verify_password(pass: String, hash: String) -> Result<bool, BackendError> {
    let parsed_hash =
        PasswordHash::new(&hash).map_err(|e| BackendError::GenericError(e.to_string()))?;

    Ok(Argon2::default().verify_password(pass.as_bytes(), &parsed_hash).is_ok())
}
