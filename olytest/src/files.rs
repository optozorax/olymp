use regex::bytes::Regex;
use std::error;
use std::ffi::OsStr;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

pub fn ignore_io_error(
    input: Result<(), std::io::Error>,
    kind: ErrorKind,
) -> Result<(), std::io::Error> {
    match input.map_err(|x| (x.kind(), x)) {
        Ok(()) => Ok(()),
        Err((k, _)) if k == kind => Ok(()),
        Err((_, other)) => Err(other),
    }
}

pub fn ignore_already_exists(input: Result<(), std::io::Error>) -> Result<(), std::io::Error> {
    ignore_io_error(input, ErrorKind::AlreadyExists)
}

pub fn ignore_not_found(input: Result<(), std::io::Error>) -> Result<(), std::io::Error> {
    ignore_io_error(input, ErrorKind::NotFound)
}

#[derive(Debug)]
pub struct FileIoError {
    pub file: PathBuf,
    pub error: std::io::Error,
}

impl fmt::Display for FileIoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "for file `{}`, {}", self.file.display(), self.error)
    }
}

impl error::Error for FileIoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.error.source()
    }
}

pub fn for_file<S, T, F>(file: S, f: F) -> Result<T, FileIoError>
where
    S: Into<PathBuf> + AsRef<Path>,
    F: FnOnce(&Path) -> Result<T, std::io::Error>,
{
    match f(file.as_ref()) {
        Ok(t) => Ok(t),
        Err(error) => Err(FileIoError {
            file: file.into(),
            error,
        }),
    }
}

pub fn create_dir<S>(s: S) -> Result<(), FileIoError>
where
    S: Into<PathBuf> + AsRef<Path>,
{
    for_file(s, |file| ignore_already_exists(std::fs::create_dir(file)))
}

pub fn remove_file<S>(s: S) -> Result<(), FileIoError>
where
    S: Into<PathBuf> + AsRef<Path>,
{
    for_file(s, |file| ignore_not_found(std::fs::remove_file(file)))
}

pub fn read<S>(s: S) -> Result<Vec<u8>, FileIoError>
where
    S: Into<PathBuf> + AsRef<Path>,
{
    #[allow(clippy::redundant_closure)]
    for_file(s, |file| std::fs::read(file))
}

pub fn write<S, C>(file: S, content: C) -> Result<(), FileIoError>
where
    S: Into<PathBuf> + AsRef<Path>,
    C: AsRef<[u8]>,
{
    for_file(file, |file| std::fs::write(file, content))
}

pub fn init_common() -> Result<(), FileIoError> {
    let rustfmt_toml = include_str!("../template/rustfmt.toml");
    let cargo_toml = include_str!("../template/Cargo.toml");
    let gitignore = include_str!("../template/.gitignore");
    let src_lib = include_str!("../template/src/lib.rs");

    create_dir("src")?;
    create_dir("src/bin")?;
    create_dir("tests")?;
    create_dir("generated")?;

    write("rustfmt.toml", rustfmt_toml)?;
    write("Cargo.toml", cargo_toml)?;
    write(".gitignore", gitignore)?;
    write("src/lib.rs", src_lib)?;

    Ok(())
}

pub fn format_path_buf<S1: AsRef<Path>, S2: AsRef<OsStr>>(
    folders: &[S1],
    extension: S2,
) -> PathBuf {
    let mut result = folders.iter().collect::<PathBuf>();
    result.set_extension(extension);
    result
}

pub fn create_executable_general(
    name: &str,
    template_relative_path: &Path,
    create_empty_test: bool,
    main_str: &str,
    bin_str: &str,
) -> Result<(), FileIoError> {
    // TODO считывать это из байтов, и тоже заменять всё на байты
    write(
        format_path_buf(&["src", &name], "rs"),
        &main_str.replace("%", template_relative_path.to_str().unwrap()),
    )?;
    write(
        format_path_buf(&["src", "bin", &name], "rs"),
        &bin_str.replace("~", name),
    )?;
    if create_empty_test {
        write(format_path_buf(&["tests", &name], "test"), "")?;
    }

    for_file("src/lib.rs", |file| {
        writeln!(
            OpenOptions::new().write(true).append(true).open(file)?,
            "pub mod {};",
            name
        )
    })
}

pub fn make_template_relavite_path(template_absolute_path: &Path) -> PathBuf {
    // TODO сделать тут нормальную обработку ошибок
    let current_dir = std::env::current_dir()
        .expect("can't get current dir")
        .join("src");
    pathdiff::diff_paths(template_absolute_path, current_dir)
        .expect("can't find diff between paths")
}

pub fn create_executable(name: &str, template_relative_path: &Path) -> Result<(), FileIoError> {
    create_executable_general(
        name,
        template_relative_path,
        true,
        include_str!("../template/src/name.rs"),
        include_str!("../template/src/bin/exe.rs"),
    )
}

pub fn create_checker(name: &str, template_relative_path: &Path) -> Result<(), FileIoError> {
    create_executable_general(
        &format!("{}_checker", name),
        template_relative_path,
        false,
        include_str!("../template/src/name_checker.rs"),
        include_str!("../template/src/bin/exe.rs"),
    )
}

pub fn remove_executable_general(name: &str, remove_test: bool) -> Result<(), FileIoError> {
    remove_file(format_path_buf(&["src", &name], "rs"))?;
    remove_file(format_path_buf(&["src", "bin", &name], "rs"))?;
    if remove_test {
        remove_file(format_path_buf(&["tests", &name], "test"))?;
    }

    let readed = read("src/lib.rs")?;
    let re = Regex::new(&format!("pub mod {};", name)).unwrap();
    let lib_rs = re.replace_all(&readed, b"".as_ref());
    write("src/lib.rs", lib_rs)
}

pub fn remove_executable(name: &str) -> Result<(), FileIoError> {
    remove_executable_general(name, true)
}

pub fn remove_checker(name: &str) -> Result<(), FileIoError> {
    remove_executable_general(&format!("{}_checker", name), false)
}

pub fn write_test<T: AsRef<[u8]>>(name: &str, content: T) -> Result<(), FileIoError> {
    write(format_path_buf(&["tests", &name], "test"), content)
}

pub fn get_current_executables() -> Result<Vec<String>, FileIoError> {
    let iter = for_file("src/bin", |file| Path::new(file).read_dir())?;
    let mut result = Vec::new();
    for entry in iter {
        let entry = for_file("src/bin", |_| Ok(entry?.path()))?;
        let s = entry.file_stem().unwrap().to_str().unwrap();
        if !s.ends_with("_checker") {
            result.push(s.to_string());
        }
    }
    result.sort_unstable();
    Ok(result)
}

pub enum GenerateError {
    OtherFiles(FileIoError),
    IncludeError(FileIoError),
}

struct GeneratedFileHeader {
    width: usize,
    lines: Vec<HeaderLine>,
}

struct HeaderLine {
    text: String,
    info_left: String,
    info_right: String,
}

fn write_to_vec(header: GeneratedFileHeader) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend(b"/".iter());
    result.extend(std::iter::repeat(b'*').take(header.width - 3));
    result.push(b'\n');
    for HeaderLine {
        text,
        info_left,
        info_right,
    } in header.lines
    {
        result.extend(b"* ".iter());
        result.extend(text.as_bytes().iter());
        result.extend(b": ".iter());
        result.extend(info_left.as_bytes().iter());
        result.extend(
            std::iter::repeat(b' ')
                .take(header.width - 8 - text.len() - info_left.len() - info_right.len()),
        );
        result.extend(info_right.as_bytes().iter());
        result.extend(b" *\n".iter());
    }
    result.extend(std::iter::repeat(b'*').take(header.width - 3));
    result.extend(b"/\n\n".iter());
    result
}

fn trim<T, P>(slice: &[T], mut predicate: P) -> &[T]
where
    P: FnMut(&T) -> bool,
{
    let mut left = 0;
    let mut right = slice.len();

    let mut iter = slice.iter();

    while let Some(e) = iter.next() {
        if predicate(e) {
            left += 1
        } else {
            break;
        }
    }

    while let Some(e) = iter.next_back() {
        if predicate(e) {
            right -= 1
        } else {
            break;
        }
    }

    &slice[left..right]
}

#[allow(clippy::redundant_closure)]
pub fn generate_without_include(path: &Path) -> Result<(), GenerateError> {
    let file = read(path).map_err(GenerateError::OtherFiles)?;
    let (parent, name) = for_file(path, |_| {
        let parent = path
            .parent()
            .ok_or_else(|| std::io::Error::new(ErrorKind::Other, "can't get parent"))?;
        let name = path.file_stem().ok_or_else(|| {
            std::io::Error::new(ErrorKind::Other, "can't get file name without extension")
        })?;
        Ok((parent, name))
    })
    .map_err(GenerateError::OtherFiles)?;

    let re = Regex::new(r#"include!\("([^"]+)"\);"#).unwrap();
    let mut files = re
        .captures_iter(&file)
        .map(|x| {
            (
                x.get(0).unwrap().range(),
                Path::new(std::str::from_utf8(x.get(1).unwrap().as_bytes()).unwrap()),
            )
        })
        .collect::<Vec<_>>();
    files.sort_unstable_by_key(|x| x.0.start);

    let new_file = {
        use chrono::prelude::*;
        let local: DateTime<Local> = Local::now();
        // TODO считывать это из настроек
        let header = GeneratedFileHeader {
            width: 80,
            lines: vec![
                HeaderLine {
                    text: "Generated and tested by".to_string(),
                    info_left: "olytest".to_string(),
                    info_right: "(https://github.com/optozorax/olytest)".to_string(),
                },
                HeaderLine {
                    text: "Author".to_string(),
                    info_left: "Ilya Sheprut".to_string(),
                    info_right: "a.k.a. optozorax".to_string(),
                },
                HeaderLine {
                    text: "Generated at".to_string(),
                    info_left: "".to_string(),
                    info_right: local.to_rfc2822(),
                },
                HeaderLine {
                    text: "License".to_string(),
                    info_left: "MIT/Apache 2.0".to_string(),
                    info_right: "".to_string(),
                },
            ],
        };
        let separator = b"//----------------------------------------------------------------------------\n\n";
        let mut new_file = write_to_vec(header);
        let mut start = 0;
        let mut first = true;
        for (range, include_file_path) in files {
            let include_file_content =
                read(parent.join(include_file_path)).map_err(GenerateError::IncludeError)?;
            new_file.extend(&file[start..range.start]);
            if !first {
                new_file.extend(separator);
            } else {
                first = false;
            }
            new_file.extend(trim(&include_file_content, |x| x.is_ascii_whitespace()));
            new_file.push(b'\n');
            start = range.end;
        }
        new_file.extend(&file[start..]);
        new_file
    };

    create_dir("generated").map_err(GenerateError::OtherFiles)?;
    write(
        format_path_buf(&[&"generated".as_ref(), &name], "rs"),
        new_file,
    )
    .map_err(GenerateError::OtherFiles)
}
