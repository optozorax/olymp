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
    about = "Program to run tests as easy and as fast as possible during competitive programming.\nWrite inputs for program in file `tests/<name>.in`, when each input separated by `\\` symbol. Each input will be run for program independently. Also, you can write outputs to file `tests/<name>.out`, separated by `\\` symbol. Output from program will be compared to those outputs which are written in `tests/<name>.out`. For other inputs, outputs of program just printed on screen.",
    author = "Ilya Sheprut a.k.a. optozorax"
)]
struct Opt {
    /// Run executable in debug mode with `RUST_BACKTRACE=full`.
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

    /// One-letter name of binary to run. May be not one-lettered: `e1`, `e2`.
    name: String,
}

enum RunOk {
    Success(std::process::Output, Duration),
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
        // TODO, this isn't working :(
        if let Ok(key) = std::env::var("COLORTERM") {
            process.env("COLORTERM", &key);
        }
        if let Ok(key) = std::env::var("TERM") {
            process.env("TERM", &key);
        }
        if opt.debug {
            process.env("RUST_BACKTRACE", "full");
        }

        process.stdout(Stdio::piped()).stderr(Stdio::piped());
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
    } else {
        Ok(RunOk::Success(output, duration))
    }
}

struct ReadFileError<'a> {
    file: &'a str,
    error: std::io::Error,
}

fn read_file(file: &str) -> Result<Vec<u8>, ReadFileError> {
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
                .filter(|x| !x.is_empty())
                .eq(bline
                    .split(u8::is_ascii_whitespace)
                    .filter(|x| !x.is_empty()))
        })
}

fn main() {
    let opt = Opt::from_args();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut stderr = StandardStream::stdout(ColorChoice::Auto);

    let inputs = read_file(format!("tests/{}.in", opt.name).as_str()).unwrap_or_else(|err| {
        match err.error.kind() {
            std::io::ErrorKind::NotFound => {
                clrln!(stdout: b (Color::Red) "File "; "`{}`", err.file; b (Color::Red) " not found."; " You should create this file and write input tests to your program in it.");
            },
            _ => {
                clrln!(stdout: b (Color::Red) "Unknown input error:"; " while reading `{}`: {}", err.file, err.error);
            },
        }
        exit(1)
    });
    let outputs = read_file(format!("tests/{}.out", opt.name).as_str()).unwrap_or_else(|_| Vec::new());

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
                .args(&["--bin", &opt.name])
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
                .args(&["--bin", &opt.name])
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
            let postfix = &opt.name;
            let _postfix_old = cargo_toml
                .get("package")
                .context("no field `package`")?
                .get("name")
                .context("no field `package.name`")?
                .as_str()
                .context("`package.name` isn't a string")?;
            Ok(prefix.to_string() + postfix)
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
            Ok(Success(output, duration)) => {
                let mut print_stderr = opt.stderr;
                match o {
                    Some(o) => {
                        if output_equals(o, &output.stdout) {
                            ok_count += 1;
                            clr!(stdout: b (Color::Green) "Test {} OK", no);
                            if !output.stderr.is_empty() {
                                clr!(stdout: ", but with "; (Color::Red) "stderr");
                            }
                            if opt.time {
                                clr!(stdout: ", "; (Color::Blue) "run time: "; "{:.1?}", duration);
                            }
                            writeln!(stdout).unwrap();
                        } else {
                            err_count += 1;
                            clrln!(stdout: b (Color::Red) "Test {} ERR", no);
                            clrln!(stdout: n (Color::Blue) "input:");
                            stdout.write_all(i).unwrap();
                            writeln!(stdout).unwrap();
                            clrln!(stdout: n (Color::Blue) "output:");

                            // TODO improve this thing, remove strings, &str, and do diff on raw [u8]!
                            let output_stdout = String::from_utf8(output.stdout).unwrap();
                            let o = String::from_utf8(o.to_vec()).unwrap();
                            for line in diff::lines(&output_stdout.trim(), &o.trim()) {
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
                        writeln!(stdout).unwrap();
                        clrln!(stdout: n (Color::Blue) "output:");
                        stdout.write_all(&output.stdout).unwrap();
                        writeln!(stdout).unwrap();
                        print_stderr = true;
                    }
                }
                if print_stderr && output.stderr.iter().any(|x| !x.is_ascii_whitespace()) {
                    writeln!(stdout).unwrap();
                    clrln!(stdout: n (Color::Blue) "stderr:");
                    stdout.write_all(&output.stderr).unwrap();
                    writeln!(stdout).unwrap();
                }
                if opt.time {
                    clrln!(stdout: (Color::Blue) "Run time: "; "{:.1?}", duration);
                }
            }
            Err(Crash(output)) => {
                err_count += 1;
                clrln!(stdout: b (Color::Red) "Test {} crashed", no);
                clrln!(stdout: n (Color::Blue) "input:");
                stdout.write_all(i).unwrap();
                writeln!(stdout).unwrap();
                clrln!(stdout: n (Color::Blue) "output:");
                stdout.write_all(&output.stdout).unwrap();
                writeln!(stdout).unwrap();
                clrln!(stdout: n (Color::Blue) "stderr:");
                stdout.write_all(&output.stderr).unwrap();
                writeln!(stdout).unwrap();
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

    writeln!(stdout).unwrap();

    if err_count == 0 {
        clrln!(stdout: b u (Color::Green) "OK all {} tests.", ok_count);
    } else {
        clrln!(stdout: b u (Color::Green) "OK"; b (Color::Green) ":"; " {} tests, ", ok_count; b u (Color::Red) "ERR";  b (Color::Red) ":"; " {} tests.", err_count);
    }
}
