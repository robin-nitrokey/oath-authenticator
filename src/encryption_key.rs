use trussed::{types::KeyId, Client};

pub trait EncryptionKeyGetter {
    /// Gets encryption KeyID
    fn get_encryption_key<C: Client>(client: &mut C) -> KeyId;

    /// Gets encryption KeyID for the given password
    fn get_encryption_key_for_password<C: Client>(client: &mut C, password: &[u8]) -> KeyId;
}
