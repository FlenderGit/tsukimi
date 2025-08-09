static SERVICE_NAME: &str = "tsukimi";
static USERNAME: &str = "github_access_token";

pub fn store_token(token: &str) -> Result<(), keyring::Error> {
    let store = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    store.set_password(token)?;
    println!("Token stored successfully.");
    Ok(())
}

pub fn read_token() -> Result<String, keyring::Error> {
    println!("Reading token from keyring...");
    let entry = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    let token = entry.get_password()?;
    println!("Token read successfully.");
    Ok(token)
}
