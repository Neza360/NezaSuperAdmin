mod hash_password;

fn main() {
    let password = "super_secret";

    // Hash it
    let hashed = hash_password::hash_password(password).unwrap();
    println!("Stored hash: {}", hashed);

    // Verify (correct)
    let valid = hash_password::verify_password("super_secret", &hashed);
    println!("Password correct? {}", valid);

    // Verify (wrong)
    let valid = hash_password::verify_password("wrong_pass", &hashed);
    println!("Password correct? {}", valid);
}
