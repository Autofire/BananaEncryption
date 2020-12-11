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
	fn reset_ui( &mut self, ctx: &mut Context) {
		MainView::target_file_set( &mut ctx.widget(), String::from("No file"));
		MainView::error_set( &mut ctx.widget(), String::new());
		ctx.child("pager").set::<usize>("current_index", 0);
		//ctx.child("decrypt_password").set::<String>("text", String::new());
		//ctx.child("encrypt_password").set::<String>("text", String::new());
		//ctx.child("encrypt_password2").set::<String>("text", String::new());

		self.path = String::new();
	}

	fn show_success_page(&mut self, message: String, ctx: &mut Context) {
		ctx.child("pager").set::<usize>("current_index", 3);
		MainView::success_set( &mut ctx.widget(), message);
	}
		

	fn process_new_file( &mut self, path: String, ctx: &mut Context) {
		self.reset_ui(ctx);

		match get_file_type(&path) {
			FileType::Encrypted => {
				ctx.child("pager").set::<usize>("current_index", 2);
			}
			FileType::Decrypted => {
				ctx.child("pager").set::<usize>("current_index", 1);
			}
		}
		self.path = path.clone();
		MainView::target_file_set(&mut ctx.widget(), path);
	}
	
	fn process_confirm( &mut self, ctx: &mut Context) {
		match get_file_type(&self.path) {
			FileType::Encrypted => {
				self.do_decrypt(ctx);
			}
			FileType::Decrypted => {
				self.do_encrypt(ctx);
			}
		}
	}

	fn do_decrypt( &mut self, ctx: &mut Context) {
		let password = ctx.child("decrypt_password").get::<String>("text").clone();
		if password.is_empty() {
			self.show_error("No password", ctx);
		}
		else {
			self.show_io_result(decrypt_file(&self.path, &password, &self.pg), "Decryption", ctx);
		}
	}
		
	fn do_encrypt( &mut self, ctx: &mut Context) {
		let password = ctx.child("encrypt_password").get::<String>("text").clone();
		let password2 = ctx.child("encrypt_password2").get::<String>("text").clone();
		if password != password2 {
			self.show_error("Password mismatch", ctx);
		}
		else if password.is_empty() {
			self.show_error("No password", ctx);
		}
		else if password.len() < 6 {
			self.show_error("Password too short", ctx);
		}
		else {
			self.show_io_result(encrypt_file(&self.path, &password, &self.pg), "Encryption", ctx);
		}
	}

	fn show_io_result( &mut self, r: std::io::Result<()>, op_name: &str, ctx: &mut Context) {
		match r {
			Ok(_) => {
				self.show_success_page(format!("{} successful", op_name), ctx);
			},
			Err(e) => {
				if let Some(inner) = e.into_inner() {
					self.show_error(&format!("{}", inner)[..], ctx);
				}
				else {
					self.show_error(&format!("{} failed", op_name)[..], ctx);
				}
			}
		}
	}

	fn show_error( &mut self, msg: &str, ctx: &mut Context) {
		println!("{}", msg);
		MainView::error_set( &mut ctx.widget(), msg);
	}
}

impl State for MainState {

    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
        println!("Generating primes");
        self.pg = PrimeGenerator::new(1000000);
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
			_registry: &mut Registry,
			ctx: &mut Context
	) {

		for message in messages.read::<Message>() {
			match message {
				Message::NewFile(p) => {
					self.process_new_file(p, ctx);
				},
                Message::Confirm => {
					self.process_confirm(ctx);
                },
				Message::ClearFile => {
					println!("Clear");
					self.reset_ui(ctx);
				}
			}
		}
	}


		

}
