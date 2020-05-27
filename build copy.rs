extern crate cmake;

use cmake::Config;

fn main() {
	let dst = Config::new("libbadmath")
		.cflag("-lSDL2")
		.cflag("-lSDL2_gfx")
		// .build_arg("-lSDL2")
		.build();

	println!("cargo:rustc-link-search=native={}", dst.display());
	println!("cargo:rustc-link-lib=static=badmath");
	//println!("cargo::rustc-flags=-lSDL2 -lSDL2_gfx")
	println!("cargo::rustc-flags=-lSDL2 -lSDL2_gfx")
}
