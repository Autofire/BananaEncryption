use crate::primes::PrimeGenerator;

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

pub fn encrypt_file(path: &String, password: &String, pg: &PrimeGenerator) -> bool {
    println!("encrypt: {}, {}, {}",path,password, pg[10]);
    //return false;
    return true;
}

pub fn decrypt_file(path: &String, password: &String, pg: &PrimeGenerator) -> bool {
    println!("encrypt: {}, {}, {}",path,password, pg[10]);
    //return false;
    return true;
}
