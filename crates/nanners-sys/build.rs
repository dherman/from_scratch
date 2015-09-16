extern crate gcc;

// use std::path::Path;
use std::process::Command;

fn object_path(libname: &str) -> String {
    // FIXME: Debug vs Release based on envvar (DEBUG)
    format!("build/Release/obj.target/{}/src/{}.o", libname, libname)
}

/*
pub fn write(path: &Path, string: &str) -> io::Result<()> {
    let mut f = try!(File::create(path));
    f.write_all(string.as_bytes())
}
*/

fn main() {
    /*
    let c_src = String::from_utf8_lossy(&Command::new("node").current_dir(&Path::new("sizes")).arg("build.js").output().ok().unwrap().stdout);
    write(&Path::new("sizes/sizes.cc"), &c_str[..]).ok().unwrap();
    Command::new("npm").current_dir(&Path::new("sizes")).arg("install").status().ok().unwrap();
    Command::new("npm").current_dir(&Path::new("sizes")).arg("run").arg("configure").status().ok().unwrap();
    Command::new("npm").current_dir(&Path::new("sizes")).arg("run").arg("build").status().ok().unwrap();
    Command::new("gcc").current_dir(&Path::new("sizes")).arg("-o").arg("sizes").(object_path("sizes")).status().ok().unwrap();
    let sizes_json = String::from_utf8_lossy(&Command::new("./sizes").current_dir(&Path::new("sizes")).output().ok().unwrap().stdout);
    
     */

    // 1. Build the object file from source using node-gyp.
    Command::new("npm").arg("install").status().ok().unwrap();
    Command::new("npm").arg("run").arg("configure").status().ok().unwrap();
    Command::new("npm").arg("run").arg("build").status().ok().unwrap();

    // 2. Link the library from the object file using gcc.
    gcc::Config::new()
        .object(object_path("nanners"))
        .compile("libnanners.a");
}
