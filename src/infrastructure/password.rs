pub fn hash(password: &str) -> Result<String, String> {
    let cost: u32 = 8;
    let hashed_password = bcrypt::hash(password, cost).expect("Failed to hash password");
    Ok(hashed_password)
}
