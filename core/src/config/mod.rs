use keyring::KeyringError;

const APP_NAME : &str = "kaf9s";

pub fn get_secret_from_keyring(input : &str) -> Result<String, KeyringError> {
    let key = get_key(&input);
    let key_store = keyring::Keyring::new(&key, "");
    key_store.get_password()
}

pub fn get_key(input : &str) -> String {
    format!("{}/{}", APP_NAME, input)
}

pub fn set_secret_in_keyring(input : &str, val : &str) -> Result<(), KeyringError> {
    let key = get_key(&input);
    let key_store = keyring::Keyring::new(&key, "");
    key_store.set_password(val)
}