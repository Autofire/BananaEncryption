#[derive(Debug)]
pub enum FileType {
	Encrypted,
	Decrypted,
}

pub fn get_file_type(path: &String) -> FileType {
	if path.ends_with(".banana") {
		return FileType::Encrypted;
	}
	else {
		return FileType::Decrypted;
	}
}

pub fn encrypt_file(path: &String, password: &String) -> bool {
    println!("encrypt: {}, {}",path,password);
    return true;
}

pub fn decrypt_file(path: &String, password: &String) -> bool {
    println!("decrypt: {}, {}",path,password);
    return true;
}
