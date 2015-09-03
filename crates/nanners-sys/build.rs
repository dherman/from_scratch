extern crate gcc;

use std::process::Command;

fn object_path(libname: &str) -> String {
    format!("build/Release/obj.target/{}/src/{}.o", libname, libname)
}

fn main() {
    // 1. Build the object file from source using node-gyp.
    Command::new("npm").arg("install").status().ok().unwrap();
    Command::new("npm").arg("run").arg("configure").status().ok().unwrap();
    Command::new("npm").arg("run").arg("build").status().ok().unwrap();

    // 2. Link the library from the object file using gcc.
    gcc::Config::new()
        .object(object_path("nanners"))
        .compile("libnanners.a");
}
