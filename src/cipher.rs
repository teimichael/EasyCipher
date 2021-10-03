use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use secrecy::Secret;
use sha3::{Digest, Sha3_512};

use crate::Config;
use crate::utils::get_uuid;

pub fn encrypt(config: &Config) -> Result<String, Box<dyn Error>> {
    // Read file
    println!("Loading file");
    let mut file = File::open(&config.file_path)?;
    let mut file_bytes = Vec::new();
    let _ = file.read_to_end(&mut file_bytes);

    let plaintext = &file_bytes[..];

    // Generate key
    println!("Generating secret key");
    let mut hasher = Sha3_512::new();
    let passphrase_components = format!("{}-{}", get_uuid(), &config.secret);
    hasher.update(passphrase_components.as_bytes());
    let result = hasher.finalize();
    let passphrase = format!("{:x}", result);

    // Calculate encrypted file
    println!("Encrypting file");
    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(plaintext)?;
        writer.finish()?;

        encrypted
    };

    // Write encrypted file to disk
    let path_to_encrypted_file = format!("{}encrypted.enc", &config.output_dir);
    println!("{}", format!("Writing encrypted file to {}", path_to_encrypted_file));
    let mut encrypted_file = File::create(path_to_encrypted_file)?;
    encrypted_file.write_all(&encrypted)?;
    encrypted_file.flush()?;

    // Write secrete key to disk
    let path_to_key_file = format!("{}encrypted.key", &config.output_dir);
    println!("{}", format!("Writing key file to {}", path_to_key_file));
    let mut passphrase_file = File::create(path_to_key_file)?;
    passphrase_file.write_all(passphrase.as_bytes())?;
    encrypted_file.flush()?;

    Ok(passphrase)
}

pub fn decrypt(config: &Config) -> Result<(), Box<dyn Error>> {
    // Read encrypted file
    println!("Loading file");
    let mut encrypted_file = File::open(&config.file_path)?;
    let mut encrypted_file_bytes = Vec::new();
    let _ = encrypted_file.read_to_end(&mut encrypted_file_bytes);
    let encrypted = &encrypted_file_bytes[..];

    // Calculate decrypted file
    println!("Decrypting file");
    let decrypted = {
        let decryptor = match age::Decryptor::new(&encrypted[..]) {
            Ok(d) => {
                match d {
                    age::Decryptor::Passphrase(d) => d,
                    _ => unreachable!(),
                }
            }
            Err(e) => {
                eprintln!("Input file may not be encrypted.");
                return Err(Box::new(e));
            }
        };

        let mut decrypted = vec![];
        let mut reader = match decryptor.decrypt(&Secret::new(config.secret.to_owned()), None) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Secret key may not be correct.");
                return Err(Box::new(e));
            }
        };
        let _ = reader.read_to_end(&mut decrypted);

        decrypted
    };

    // Write decrypted file to disk
    let path_to_decrypted_file = format!("{}decrypted", &config.output_dir);
    println!("{}", format!("Writing decrypted file to {}", path_to_decrypted_file));
    let mut decrypted_file = File::create(path_to_decrypted_file)?;
    decrypted_file.write_all(&decrypted)?;
    decrypted_file.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{File, remove_file};
    use std::io::Read;

    use secrecy::Secret;

    use crate::cipher::encrypt;
    use crate::Config;

    #[test]
    fn file_encryption_correct() {
        // Simulate a configuration
        let config = Config {
            mode: "e".to_string(),
            file_path: "./Cargo.toml".to_string(),
            secret: "a secret for test".to_string(),
            output_dir: "./".to_string(),
        };

        // Encrypt the input file
        let passphrase = encrypt(&config).unwrap();

        // Read origin file
        let mut origin_file = File::open(&config.file_path).unwrap();
        let mut origin_file_bytes = Vec::new();
        let _ = origin_file.read_to_end(&mut origin_file_bytes);
        let origin = &origin_file_bytes[..];

        // Read encrypted file
        let mut encrypted_file = File::open(format!("{}encrypted.enc", &config.output_dir)).unwrap();
        let mut encrypted_file_bytes = Vec::new();
        let _ = encrypted_file.read_to_end(&mut encrypted_file_bytes);
        let encrypted = &encrypted_file_bytes[..];

        // Calculate decrypted file
        let decrypted = {
            let decryptor = match age::Decryptor::new(&encrypted[..]).unwrap() {
                age::Decryptor::Passphrase(d) => d,
                _ => unreachable!(),
            };

            let mut decrypted = vec![];
            let mut reader = decryptor.decrypt(&Secret::new(passphrase), None).unwrap();
            let _ = reader.read_to_end(&mut decrypted);

            decrypted
        };

        // Decrypted file is equal to the origin
        assert_eq!(decrypted, origin);

        // Clear outputs
        remove_file(format!("{}encrypted.enc", &config.output_dir)).unwrap();
        remove_file(format!("{}encrypted.key", &config.output_dir)).unwrap();
    }
}