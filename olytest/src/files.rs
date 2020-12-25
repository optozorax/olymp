use std::fs::{create_dir, write, OpenOptions, remove_file, read_to_string, File};
use std::path::Path;
use std::fmt::Display;
use std::io::Write;

pub fn init_common() -> Result<(), std::io::Error> {
    let rustfmt_toml = include_str!("../template/rustfmt.toml");
    let cargo_toml = include_str!("../template/Cargo.toml");
    let gitignore = include_str!("../template/.gitignore");
    let src_lib = include_str!("../src/lib.rs");

    create_dir("src")?;
    create_dir("bin")?;
    create_dir("tests")?;

    write("rustfmt.toml", rustfmt_toml)?;
    write("Cargo.toml", cargo_toml)?;
    write(".gitignore", gitignore)?;
    write("src/lib.rs", src_lib)?;

    Ok(())
}

pub fn create_executable(name: &str) -> Result<(), std::io::Error> {
    let main = include_str!("../template/src/name.rs");
    let bin = include_str!("../template/src/bin/exe.rs");
    write(&format!("src/{}.rs", name), main)?;
    write(&format!("src/bin/{}.rs", name), &bin.replace("~", name))?;
    write(&format!("tests/{}.test", name), "")?;

    writeln!(OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/lib.rs")?, "pub mod {};", name)
}

pub fn create_checker(name: &str) -> Result<(), std::io::Error> {
    let bin = include_str!("../template/src/bin/checker.rs");
    write(&format!("src/bin/{}_checker.rs", name), bin)
}

pub fn remove_executable(name: &str) -> Result<(), std::io::Error> {
    remove_file(&format!("src/{}.rs", name))?;
    remove_file(&format!("src/bin/{}.rs", name))?;
    remove_file(&format!("tests/{}.test", name))?;

    let lib_rs = read_to_string("src/lib.rs")?.replace(&format!("pub mod {};", name), "");
    write("src/lib.rs", lib_rs)
}

pub fn remove_checker(name: &str) -> Result<(), std::io::Error> {
    remove_file(&format!("src/bin/{}_checker.rs", name))
}

pub fn write_test<T: Display>(name: &str, content: T) -> Result<(), std::io::Error> {
    let mut file = File::create(&format!("tests/{}.test", name))?;
    write!(file, "{}", content)
}

pub fn get_current_executables() -> Result<Vec<String>, std::io::Error> {
    let binaries_path = Path::new("src/bin");
    let mut result = Vec::new();
    for entry in binaries_path.read_dir()? {
        let entry = entry?.path();
        let prefix = "src/bin/";
        let postfix = ".rs";
        let s: &str = entry.to_str().unwrap();
        let s = s.get(prefix.len()..s.len() - postfix.len()).unwrap().to_string();
        if !s.ends_with("_checker") {
            result.push(s);
        }
    }
    result.sort_unstable();
    Ok(result)
}