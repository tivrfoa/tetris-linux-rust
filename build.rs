use std::process::Command;
use std::path::Path;

fn main() {
	let out_dir = "run";

    Command::new("gcc").args(&["lib/View.c", "-c", "-fPIC", "-o"])
                       .arg(&"run/hello.o")
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
	println!("cargo:rustc-link-lib=static=hello");
	println!("cargo:rustc-flags=-lSDL2 -lSDL2_gfx")
}