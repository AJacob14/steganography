use steganography::lsb;

fn main() {
    let result = lsb::encode("test_files/test.png", "test_files/lorem.txt", "test_files/test_encoded.png");
    if result.is_err() {
        println!("{}", result.unwrap_err());
    }

    println!("Decoding...");

    let result = lsb::decode("test_files/test_encoded.png", "test_files/lorem_decoded.txt");
    if result.is_err() {
        println!("{}", result.unwrap_err());
    }
}
