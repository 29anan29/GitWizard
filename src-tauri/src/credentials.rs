use keyring::Entry;

const SERVICE: &str = "gitwizard";

fn entry(username: &str) -> Result<Entry, String> {
    Entry::new(SERVICE, username).map_err(|e| format!("KEYRING_UNAVAILABLE:{e}"))
}

fn map_err(e: keyring::Error) -> String {
    format!("KEYRING_UNAVAILABLE:{e}")
}

pub fn save(username: String, secret: String) -> Result<(), String> {
    entry(&username)?
        .set_password(&secret)
        .map_err(map_err)
}

pub fn load(username: String) -> Result<Option<String>, String> {
    match entry(&username)?.get_password() {
        Ok(secret) => Ok(Some(secret)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(map_err(e)),
    }
}

pub fn delete(username: String) -> Result<(), String> {
    match entry(&username)?.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(map_err(e)),
    }
}
