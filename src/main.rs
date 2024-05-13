slint::include_modules!();
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::Path;
use std::fs::OpenOptions;


fn main() -> Result<(), slint::PlatformError> {
	let ui = MiVentana::new()?;
	
	match Path::new("names.txt").try_exists(){
		Ok(false)=> {
			let mut file = File::create_new("names.txt")
				.expect("Error al crear archivo");
			ui.on_save_name(move |string|{
				let name: &[u8] = string.as_bytes();
				file.write(name)
					.expect("Error al escribir archivo");
				file.write(b"\n").expect("Error al escribir al archivo");
			});
		}
		Ok(true)=>{
			ui.on_save_name(move |string|{
				let name: &[u8] = string.as_bytes();
				let file = OpenOptions::new()
					.append(true)
					.open("names.txt")
					.expect("Unable to open file");
				let mut file = BufWriter::new(file);
				file.write_all(name,).expect("Unable to write data");
				file.write_all(b"\n").expect("Unable to write data");
			});
		}
		Err(_)=>println!("Error al comprobar si el archivo existe")
	};
	ui.run()
}
