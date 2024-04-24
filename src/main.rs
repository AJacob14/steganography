use steganography::lsb;

fn main() {
    // Must be png (or other loseless image type)
    let result = lsb::encode("test_files/puppy.png", "test_files/top_secret.png", "test_files/test_encoded.png");
    if result.is_err() {
        println!("{}", result.unwrap_err());
    }

    println!("Decoding...");

    let result = lsb::decode("test_files/test_encoded.png", "test_files/message.png");
    if result.is_err() {
        println!("{}", result.unwrap_err());
    }
}
