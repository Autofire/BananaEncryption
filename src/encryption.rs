use crate::primes::PrimeGenerator;

use std::io::Write;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::num::Wrapping;
use rand::Rng;


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

pub fn encrypt_file(path: &String, password: &String, pg: &PrimeGenerator) -> io::Result {

let mut rng = rand::thread_rng();
	let prime_array = rng.gen_range(50, 1000000);
	
	//open file
	let file_path = path;
    let f = File::open(file_path)?;
	let buffer_size = 269420690 * 4;
 let mut reader = BufReader::with_capacity(buffer_size, f);

    let capacity = reader.capacity();
    let buffer = reader.fill_buf()?; //?
    assert!(buffer.len() <= capacity);


let file_contents: Vec<u8> = buffer.to_vec();
let mut file_contents_modified: Vec<u8> = buffer.to_vec();

//get file length
let fin = file_contents.len();


//do algorithms HERE---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//get data here

//call prime heres and load to byte vector. also generate a copy just in case
let heres_the_prime= pg[prime_array].to_string();
let mut prime_vector = heres_the_prime.into_bytes();
let prime_copy = prime_vector.clone();



//gets password and converts to byte vector and as length of the vector
let pass_word = String::from(password);
let pw_vector = pass_word.into_bytes();
let pw_length = pw_vector.len();


//make prime number same length as password
if prime_vector.len() < pw_vector.len(){
	for i in 0..pw_length{
	prime_vector[i] = prime_vector[i % prime_vector.len()]
	}	
}
else{	
	prime_vector.drain(pw_length..prime_vector.len());
}


//maybe?
//let mut wham_bam_thank_you_mam: Vec<u8>;//with_capacity(1000000000);


//generate combo of prime number AND password vectors
let mut wombo_combo = Vec::new();//with_capacity(1000000000);
for i in 0..pw_length{
	let total_this = (pw_vector[i] + prime_vector[i]) % 255;
	wombo_combo.push(total_this);
}


//println!("The bytes: {:?}", pw_vector);
//println!("The bytes: {:?}", prime_vector);
//println!("The bytes: {:?}", wombo_combo);



//alters the content
for i in 0..fin{
	let part_1 = Wrapping(wombo_combo[i % pw_length])
			   + Wrapping(file_contents[i % file_contents.len()]);

	std::mem::replace(&mut file_contents_modified[i], part_1.0);
}



//inserting prime vector into modified content
for i in 0..prime_vector.len(){
	let position = pw_vector[0] as usize;
	let insert = prime_copy[i];
	file_contents_modified.insert(position + i, insert);
}

for i in 0..wombo_combo.len(){
	let position = pw_vector[5] as usize;
	let insert = wombo_combo[i];
	file_contents_modified.insert( i, insert);
}


//puts byte vector into usable version again for putting into file
let final_form: &[u8] = &file_contents_modified;




//testing prints

//println!("The bytes: {:?}", prime_vector);
//println!("password: {:?}", pw_vector);
//println!("wombo combo: {:?}", wombo_combo);
//println!("og content: {:?}", file_contents);
//println!("modified: {:?}", file_contents_modified);



//writes to new file with
let path = file_path;
let path_extension = ".banana";
let final_path_bath = format!("{}{}", path, path_extension);
let mut ofile = File::create(final_path_bath)
                       .expect("unable to create file");


ofile.write_all(final_form).expect("unable to write");


    Ok(())
}




//--------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------


pub fn decrypt_file(path: &String, password: &String, _pg: &PrimeGenerator) -> io::Result<()> {

    let file_path = path;
	let mut short_file_path = file_path.to_string();
    let f = File::open(file_path)?;
	let buffer_size = 269420690 * 4;
 	let mut reader = BufReader::with_capacity(buffer_size, f);

    let capacity = reader.capacity();
    let buffer = reader.fill_buf()?; //?
    assert!(buffer.len() <= capacity);

	//load contents of file to byte vector
	let mut file_contents: Vec<u8> = buffer.to_vec();






//get password from user
	let pass_word = String::from(password);
	let pw_vector = pass_word.into_bytes();
	let pw_length = pw_vector.len();


	let mut old_password = pw_vector.clone();



//do algorithms HERE---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//get wombo_combo out
	let mut wombo_combo: Vec<u8> = Vec::new();
	for i in 0..pw_length{
//	let position = pw_vector[5] as usize;
	let insert = file_contents[i];
	wombo_combo.insert( i, insert);
}


//remove wombo_combo from content vector
file_contents.drain(0.. pw_length);


//get prime from content vector
let mut prime_vector: Vec<u8> = Vec::new();
for i in 0..pw_length{
	let position = pw_vector[0] as usize;
	let insert = file_contents[position + i];
	prime_vector.insert(i, insert);
}



//remove the prime
file_contents.drain(pw_vector[0] as usize.. (pw_vector[0] as usize + pw_length));

//copy the now shorter contents to copy vector
let mut file_contents_modified = file_contents.clone();


//combine extract old password

for i in 0..pw_length{
	old_password[i] = wombo_combo[i] - prime_vector[i];
}
//    println!("The bytes: {:?}", old_password);


//check old vs new password
if pw_vector != old_password{
return Err(())
}



else{
//unalter content
let fin = file_contents.len();
for i in 0..fin{
	let part_1 = Wrapping(file_contents[i % file_contents.len()])
			   - Wrapping(wombo_combo[i % wombo_combo.len()]);

	std::mem::replace(&mut file_contents_modified[i], part_1.0);
}


//last load
let final_form: &[u8] = &file_contents_modified;
//
		   println!("The bytes: {:?}", final_form);



//remove the .banana extension    :::: .banana = 7

for i in 0..7{
	    short_file_path.pop();

}



//writes to new file with
let path = short_file_path;
let final_path_bath = path;
let mut ofile = File::create(final_path_bath)
                       .expect("unable to create file");


ofile.write_all(final_form).expect("unable to write");

}

    Ok(())
}

