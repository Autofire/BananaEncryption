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
