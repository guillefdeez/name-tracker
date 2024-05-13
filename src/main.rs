slint::include_modules!();
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), slint::PlatformError> {
  	let ui = MiVentana::new()?;
  	let mut file = File::create_new("names.txt")
		.expect("Error al crear archivo");

	ui.on_save_name(move |string|{
		let name = string.as_bytes();
		file.write(name)
		.expect("Error al escribir archivo");
		file.write(b"\n").expect("Error al escribir al archivo");
	});

  	ui.run()
}
