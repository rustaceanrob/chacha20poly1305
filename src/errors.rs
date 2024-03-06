use std::error::Error;

#[derive(Debug)]
pub enum ChaCha20Poly1305DecryptionError {
    UnauthenticatedAdditionalData(String),
    CiphertextTooShort(String),
}

impl std::fmt::Display for ChaCha20Poly1305DecryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChaCha20Poly1305DecryptionError::UnauthenticatedAdditionalData(s) => write!(f, "Cipher error: {}", s),
            ChaCha20Poly1305DecryptionError::CiphertextTooShort(s) => write!(f, "Cipher error: {}", s),
        }
    }
}

impl Error for ChaCha20Poly1305DecryptionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChaCha20Poly1305DecryptionError::UnauthenticatedAdditionalData(_s) => None,
            ChaCha20Poly1305DecryptionError::CiphertextTooShort(_s) => None,
        }
    }
}