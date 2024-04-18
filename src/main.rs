use naive_rsa::rsa::RSA;

fn main() {
    let rsa = RSA::new();

    let message = "Hey Alice, my username is bob and the password is !#Bob_The_Ripper ".to_string();

    let encrypted_msg = rsa.encrypt(message.clone());
    println!("Encrypted message: {:?}", encrypted_msg);

    let decrypted_msg = rsa.decrypt(encrypted_msg);
    println!("Decrypted message: {}", decrypted_msg);
}
