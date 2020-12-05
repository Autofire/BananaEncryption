use orbtk::prelude::*;
use crate::primes::PrimeGenerator;

use crate::{
	MainView,
	get_file_type, FileType, encrypt_file, decrypt_file
};

pub enum Message {
	NewFile(String),
    Confirm,
	ClearFile,
}

#[derive(Default, AsAny)]
pub struct MainState {
	path: String,
    pg: PrimeGenerator
}

impl MainState {
	fn reset_ui(
			&mut self,
			_registry: &mut Registry,
			ctx: &mut Context
	) {
		MainView::target_file_set( &mut ctx.widget(), String::from("No file"));
		ctx.child("pager").set::<usize>("current_index", 0);
		//ctx.child("decrypt_password").set::<String>("text", String::new());
		//ctx.child("encrypt_password").set::<String>("text", String::new());
		//ctx.child("encrypt_password2").set::<String>("text", String::new());
	}
}

impl State for MainState {

    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
        println!("Generating primes");
        self.pg = PrimeGenerator::new(100000);
        println!("Done. Generated {} primes", self.pg.len());
    }

	fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
		let decrypt_password = ctx.child("decrypt_password").get::<String>("text").clone();
		MainView::decrypt_ok_set(&mut ctx.widget(), !decrypt_password.is_empty());
		//MainView::decrypt_ok_set(&mut ctx.widget(), true);
	}

	fn messages(
			&mut self,
			mut messages: MessageReader,
			registry: &mut Registry,
			ctx: &mut Context
	) {

		for message in messages.read::<Message>() {
			match message {
				Message::NewFile(p) => {
					match get_file_type(&p) {
						FileType::Encrypted => {
							//Pager::from(ctx.child("input")).navigate(1);
						}
						FileType::Decrypted => {
						}
					}
					println!("Encrypt: {}", p);
					self.path = p.clone();
					MainView::target_file_set(&mut ctx.widget(), p);
				},
                Message::Confirm => {
                    match get_file_type(&self.path) {
						FileType::Encrypted => {
				        	let password = ctx.child("decrypt_password").get::<String>("text").clone();
                            if !password.is_empty() {
                                println!("{}", self.path);
                                println!("{}", password);

                                decrypt_file(&self.path, &password);
                                
                                // Causes it to break
				        	    //ctx.child("decrypt_password").get_mut::<String>("text").truncate(0);
								self.reset_ui(registry, ctx);
                            }
                            else {
                                println!("no password");
                            }
						}
						FileType::Decrypted => {
				        	let password = ctx.child("encrypt_password").get::<String>("text").clone();
				        	let password2 = ctx.child("encrypt_password2").get::<String>("text").clone();
                            if password != password2 {
                                println!("password mismatch");
                            }
                            else if password.is_empty() {
                                println!("no password");
                            }
                            else {
                                println!("{}", self.path);
                                println!("{}", password);
                                
                                if encrypt_file(&self.path, &password) {
									self.reset_ui(registry, ctx);
                                }

                                // Causes it to break
				        	    //ctx.child("decrypt_password").get_mut::<String>("text").truncate(0);
                            }
						}
					}

                }

				Message::ClearFile => {
					println!("Clear");
					self.reset_ui(registry, ctx);
				}
			}
		}
	}


		

}
