use std::fs::File;
use std::io::{Read, Write};
use std::process::{exit, Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result as AhResult};
use itertools::{EitherOrBoth, Itertools};
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, StandardStream, WriteColor};
use termcolor_macro::*;
use toml::Value as Toml;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "olytest",
    about = "Program to run tests as easy and as fast as possible during competitive programming.\nWrite inputs for program in file `in.txt`, when each input separated by `\\` symbol. Each input will be run for program independently. Also, you can write outputs to file `out.txt`, separated by `\\` symbol. Output from program will be compared to those outputs which are written in `out.txt`. For other inputs, outputs of program just printed on screen.",
    author = "Ilya Sheprut a.k.a. optozorax"
)]
struct Opt {
    /// Run executable in debug mode with `RUST_BACKTRACE=full` and `RUSTFLAGS='--cfg color_backtrace'`.
    /// With this option you can write: `#[cfg(color_backtrace)] color_backtrace::install();` in your `fn main` to see pretty backtrace when program panics.
    #[structopt(short, long)]
    debug: bool,

    /// Show stderr of program even if test passed.
    #[structopt(short = "e", long)]
    stderr: bool,

    /// Didn't collect stdout, stderr, just show run time.
    #[structopt(short, long)]
    time: bool,

    /// Don't format code before compiling.
    #[structopt(short = "f", long = "not-format")]
    not_format: bool,
}

enum RunOk {
    OnlyTime(Duration),
    Success(std::process::Output),
}

enum RunErr {
    Crash(std::process::Output),
    NotRun(std::io::Error),
    DuringRun(std::io::Error),
    IoError(std::io::Error),
    InternalError,
}

fn run_program(path: &str, input: &[u8], opt: &Opt) -> Result<RunOk, RunErr> {
    let mut process = {
        let mut process = Command::new(&path);

        // Print colors if this program run with color printing
        if let Ok(key) = std::env::var("COLORTERM") {
            process.env("COLORTERM", &key);
        }
        if let Ok(key) = std::env::var("TERM") {
            process.env("TERM", &key);
        }
        if opt.debug {
            process.env("RUST_BACKTRACE", "full");
        }

        if opt.time {
            process.stdout(Stdio::null()).stderr(Stdio::null());
        } else {
            process.stdout(Stdio::piped()).stderr(Stdio::piped());
        }
        process
    }
    .stdin(Stdio::piped())
    .spawn()
    .map_err(RunErr::NotRun)?;

    let time = Instant::now();
    let err = process
        .stdin
        .take()
        .ok_or(RunErr::InternalError)?
        .write_all(input);
    match err.map_err(|x| (x.kind(), x)) {
        Ok(_) => (),
        Err((std::io::ErrorKind::BrokenPipe, _)) => (), // Ignore broken pipe because we write to closed channel, and we don't care about it
        Err((_, err)) => return Err(RunErr::IoError(err)),
    }

    let output = process.wait_with_output().map_err(RunErr::DuringRun)?;

    let duration = time.elapsed();

    if !output.status.success() {
        Err(RunErr::Crash(output))
    } else if opt.time {
        Ok(RunOk::OnlyTime(duration))
    } else {
        Ok(RunOk::Success(output))
    }
}

struct ReadFileError {
    file: &'static str,
    error: std::io::Error,
}

fn read_file(file: &'static str) -> Result<Vec<u8>, ReadFileError> {
    let mut result = Vec::new();
    File::open(file)
        .map_err(|error| ReadFileError { file, error })?
        .read_to_end(&mut result)
        .map_err(|error| ReadFileError { file, error })?;
    Ok(result)
}

// Checks output of program, ignores spaces, but requires other things to lay on same lines
fn output_equals(a: &[u8], b: &[u8]) -> bool {
    let alines = a
        .split(|x| *x == b'\n')
        .filter(|x| !x.iter().all(|c| c.is_ascii_whitespace()))
        .collect::<Vec<&[u8]>>();
    let blines = b
        .split(|x| *x == b'\n')
        .filter(|x| !x.iter().all(|c| c.is_ascii_whitespace()))
        .collect::<Vec<&[u8]>>();
    if alines.len() != blines.len() {
        return false;
    }

    alines
        .into_iter()
        .zip(blines.into_iter())
        .all(|(aline, bline)| {
            aline
                .split(u8::is_ascii_whitespace)
                .filter(|x| x.is_empty())
                .eq(bline
                    .split(u8::is_ascii_whitespace)
                    .filter(|x| x.is_empty()))
        })
}

fn main() {
    let opt = Opt::from_args();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut stderr = StandardStream::stdout(ColorChoice::Auto);

    let inputs = read_file("in.txt").unwrap_or_else(|err| {
        clrln!(stdout: b (Color::Red) "Input error:"; " while reading `{}`: {}", err.file, err.error);
        exit(1)
    });
    let outputs = read_file("out.txt").unwrap_or_else(|_| Vec::new());

    let tests_iterator = inputs
        .split(|x| *x == b'\\')
        .map(|x| {
            &x[x.iter()
                .position(|x| !x.is_ascii_whitespace())
                .unwrap_or(x.len())
                ..x.len()
                    - x.iter()
                        .rev()
                        .position(|x| !x.is_ascii_whitespace())
                        .unwrap_or(0)]
        }) // trim
        .zip_longest(
            outputs.split(|x| *x == b'\\').map(|x| {
                &x[x.iter()
                    .position(|x| !x.is_ascii_whitespace())
                    .unwrap_or(x.len())
                    ..x.len()
                        - x.iter()
                            .rev()
                            .position(|x| !x.is_ascii_whitespace())
                            .unwrap_or(0)]
            }), // trim
        )
        .enumerate()
        .filter_map(|(index, v)| {
            Some((
                index + 1,
                match v {
                    EitherOrBoth::Both(i, o) => (i, Some(o)),
                    EitherOrBoth::Left(i) => (i, None),
                    EitherOrBoth::Right(_) => return None,
                },
            ))
        });

    let program_name = {
        if !opt.not_format {
            clrln!(stdout: n (Color::Black) "Formatting...");
            Command::new("cargo")
                .arg("fmt")
                .output()
                .map(drop)
                .unwrap_or_else(|err| {
                    clrln!(stdout: b (Color::Red) "Format error:"; "{}", err);
                });
        }

        let output = if opt.debug {
            clrln!(stdout: n (Color::Black)"Building debug...");
            Command::new("cargo")
                .arg("build")
                .env("RUSTFLAGS", "--cfg color_backtrace")
                .env(
                    "COLORTERM",
                    &std::env::var("COLORTERM").unwrap_or_else(|_| String::new()),
                )
                .env(
                    "TERM",
                    &std::env::var("TERM").unwrap_or_else(|_| String::new()),
                )
                .output()
        } else {
            clrln!(stdout: n (Color::Black)"Building release...");
            Command::new("cargo")
                .arg("build")
                .arg("--release")
                .env(
                    "COLORTERM",
                    &std::env::var("COLORTERM").unwrap_or_else(|_| String::new()),
                )
                .env(
                    "TERM",
                    &std::env::var("TERM").unwrap_or_else(|_| String::new()),
                )
                .output()
        }
        .unwrap_or_else(|err| {
            clrln!(stdout: b (Color::Red) "Building error:"; "{:?}", err);
            exit(1)
        });

        if !output.status.success() {
            clrln!(stdout: b (Color::Red) "Building error.");
            clrln!(stdout: n (Color::Black) "With stdin from cargo:");
            drop(std::io::stdout().write_all(&output.stdout));
            println!();

            clrln!(stderr: n (Color::Black) "With stderr from cargo:");
            drop(std::io::stderr().write_all(&output.stderr));
            exit(output.status.code().unwrap_or(1));
        }

        clrln!(stdout: n (Color::Black) "Done building.");

        let program_name = || -> AhResult<String> {
            let cargo_toml = std::fs::read_to_string("Cargo.toml")
                .context("Cargo.toml")?
                .parse::<Toml>()?;
            let prefix = if opt.debug {
                "target/debug/"
            } else {
                "target/release/"
            };
            Ok(prefix.to_string()
                + cargo_toml
                    .get("package")
                    .context("no field `package`")?
                    .get("name")
                    .context("no field `package.name`")?
                    .as_str()
                    .context("`package.name` isn't a string")?)
        }()
        .unwrap_or_else(|err| {
            clrln!(stdout: b (Color::Red) "Reading `Cargo.toml` error:"; " {}", err);
            exit(1)
        });

        clrln!(stdout: n (Color::Black) "Found executable file `{}`.", program_name);

        program_name
    };

    println!();

    let mut ok_count = 0;
    let mut err_count = 0;
    for (no, (i, o)) in tests_iterator {
        use RunErr::*;
        use RunOk::*;
        match run_program(&program_name, i, &opt) {
            Ok(OnlyTime(duration)) => {
                clrln!(stdout: b (Color::Blue) "Test {} run time:", no; " {:?}", duration);
            }
            Ok(Success(output)) => {
                let mut print_stderr = opt.stderr;
                match o {
                    Some(o) => {
                        if output_equals(o, &output.stdout) {
                            ok_count += 1;
                            clrln!(stdout: b (Color::Green) "Test {} OK", no);
                        } else {
                            err_count += 1;
                            clrln!(stdout: b (Color::Red) "Test {} ERR", no);
                            clrln!(stdout: n (Color::Blue) "input:");
                            stdout.write_all(i).unwrap();
                            println!();
                            clrln!(stdout: n (Color::Blue) "output:");

                            // TODO improve this thing, remove strings, &str, and do diff on raw [u8]!
                            let output_stdout = String::from_utf8(output.stdout).unwrap();
                            let o = String::from_utf8(o.to_vec()).unwrap();
                            for line in diff::lines(&output_stdout, &o) {
                                match line {
                                    diff::Result::Left(a) => {
                                        clrln!(stdout: b n (Color::Green) "+"; (Color::Green) " {}", a)
                                    }
                                    diff::Result::Both(a, _) => println!("  {}", a),
                                    diff::Result::Right(a) => {
                                        clrln!(stdout: b n (Color::Red) "-"; (Color::Red) " {}", a)
                                    }
                                }
                            }
                            print_stderr = true;
                        }
                    }
                    None => {
                        clrln!(stdout: b (Color::Yellow) "Test {} testing", no);
                        clrln!(stdout: n (Color::Blue) "input:");
                        stdout.write_all(i).unwrap();
                        println!();
                        clrln!(stdout: n (Color::Blue) "output:");
                        stdout.write_all(&output.stdout).unwrap();
                        println!();
                        print_stderr = true;
                    }
                }
                if print_stderr && output.stderr.iter().any(|x| !x.is_ascii_whitespace()) {
                    println!();
                    clrln!(stdout: n (Color::Blue) "stderr:");
                    stdout.write_all(&output.stderr).unwrap();
                    println!();
                }
            }
            Err(Crash(output)) => {
                clrln!(stdout: b (Color::Red) "Test {} crashed", no);
                clrln!(stdout: n (Color::Blue) "input:");
                stdout.write_all(i).unwrap();
                println!();
                clrln!(stdout: n (Color::Blue) "output:");
                stdout.write_all(&output.stdout).unwrap();
                println!();
                clrln!(stdout: n (Color::Blue) "stderr:");
                stdout.write_all(&output.stderr).unwrap();
                println!();
            }
            Err(NotRun(error)) => {
                err_count += 1;
                clrln!(stdout: b (Color::Red) "Test {} won't run:", no; " {}", error);
            }
            Err(DuringRun(error)) => {
                err_count += 1;
                clrln!(stdout: b (Color::Red) "Test {} unexpected error during run:", no; " {}", error);
            }
            Err(IoError(error)) => {
                err_count += 1;
                clrln!(stdout: b (Color::Red) "Test {} i/o error:", no; " {}", error);
            }
            Err(InternalError) => {
                err_count += 1;
                clrln!(stdout: b (Color::Red) "Test {} internal error.", no);
            }
        }
    }

    println!();

    if err_count == 0 {
        clrln!(stdout: b u (Color::Green) "OK all {} tests.", ok_count);
    } else {
        clrln!(stdout: b u (Color::Green) "OK"; b (Color::Green) ":"; " {} tests, ", ok_count; b u (Color::Red) "ERR";  b (Color::Red) ":"; " {} tests.", err_count);
    }
}
