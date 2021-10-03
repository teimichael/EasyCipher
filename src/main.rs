use easy_cipher::init_cli;
use easy_cipher::cipher::{decrypt, encrypt};

/// Application entry
fn main() {

    // Get Config from the CLI
    let config = init_cli();

    match config.mode.as_ref() {
        "e" => {
            match encrypt(&config) {
                Ok(_) => println!("Encrypted"),
                Err(e) => {
                    eprintln!("Encryption failed due to: {}", e)
                }
            };
        }
        "d" => {
            match decrypt(&config) {
                Ok(_) => println!("Decrypted"),
                Err(e) => {
                    eprintln!("Decryption failed due to: {}", e)
                }
            }
        },
        _ => {
            eprintln!("Invalid mode.");
            return;
        }
    }

}
