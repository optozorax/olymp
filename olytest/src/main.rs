use std::fs::File;
use std::io::{Read, Write};
use std::process::{exit, Command};

use structopt::StructOpt;
use termcolor::{Color, ColorChoice, StandardStream, WriteColor};
use termcolor_macro::*;

use olytest::run;
use olytest::test;

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

    /// Show program output in real time.
    #[structopt(short = "r", long = "real-time")]
    real_time: bool,

    /// One-letter name of binary to run. May be not one-lettered: `e1`, `e2`.
    name: String,
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

fn compile<T: WriteColor>(stdout: &mut T, name: &str, opt: &Opt) -> String {
    let output = if opt.debug {
        clrln!(stdout, n (Color::Black)"Building `{}` in debug...", name);
        Command::new("cargo")
            .arg("build")
            .args(&["--bin", name])
            .output()
    } else {
        clrln!(stdout, n (Color::Black)"Building `{}` in release...", name);
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .args(&["--bin", name])
            .output()
    }
    .unwrap_or_else(|err| {
        clrln!(stdout, b (Color::Red) "Building error:"; "{:?}", err);
        exit(1)
    });

    // TODO не ложить всю программу, а возвращать в виде ошибки.
    if !output.status.success() {
        clrln!(stdout, b (Color::Red) "Building error.");
        clrln!(stdout, n (Color::Black) "With stdin from cargo:");
        stdout.write_all(&output.stdout).unwrap();
        println!();

        clrln!(stdout, n (Color::Black) "With stderr from cargo:");
        stdout.write_all(&output.stderr).unwrap();
        exit(output.status.code().unwrap_or(1));
    }

    let prefix = if opt.debug {
        "target/debug/"
    } else {
        "target/release/"
    };
    let postfix = &name;

    prefix.to_string() + postfix
}

fn format<T: WriteColor>(stdout: &mut T, opt: &Opt) {
    if !opt.not_format {
        clrln!(stdout, n (Color::Black) "Formatting...");
        Command::new("cargo")
            .arg("fmt")
            .output()
            .map(drop)
            .unwrap_or_else(|err| {
                clrln!(stdout, b (Color::Red) "Format error:"; "{}", err);
            });
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    let test_file_name = format!("tests/{}.test", opt.name);
    let test_file_content = read_file(test_file_name.as_str()).unwrap_or_else(|err| {
        match err.error.kind() {
            std::io::ErrorKind::NotFound => {
                clrln!(stdout, b (Color::Red) "File "; "`{}`", err.file; b (Color::Red) " not found."; " You should create this file and write input tests to your program in it.");
            },
            _ => {
                clrln!(stdout, b (Color::Red) "Unknown input error:"; " while reading `{}`: {}", err.file, err.error);
            },
        }
        exit(1)
    });

    let tests = test::read_tests(&test_file_content).unwrap_or_else(|test_no| {
        clrln!(stdout, b (Color::Red) "Wrong test format."; " In file `{}` on test #{} you have wrong format. Maybe not escaped `~`, `%`, `\\` or more than one separator of the same symbols.", test_file_name, test_no);
        exit(1)
    });
    let tests = test::tokenize_tests(&tests);

    let has_checker = test::has_checker(&tests);

    format(&mut stdout, &opt);

    let program_file_name = compile(&mut stdout, &opt.name, &opt);
    let checker_file_name = if has_checker {
        compile(&mut stdout, format!("{}_checker", opt.name).as_ref(), &opt)
    } else {
        "internal error".to_string()
    };

    println!();

    let mut ok_count = 0;
    let mut err_count = 0;
    for (no, test) in tests.into_iter().enumerate().map(|(i, x)| (i+1, x)) {
        let mut collector = run::Collector::new(opt.real_time, true, true, false);
        let result = match test {
            test::TokenizedTest::Simple { input, output } => {
                let run_result = run::run_program(&program_file_name, input, opt.debug, &mut stdout, &mut collector);
                run_result.map(|duration| (input, duration, output, collector.get_result_output(), collector.is_program_writes_stderr()))
            },
            test::TokenizedTest::WithChecker { checker_input, checker_output } => {
                let run_result = run::run_with_checker(&program_file_name, &checker_file_name, checker_input, opt.debug, &mut stdout, &mut collector);
                run_result.map(|duration| (checker_input, duration, checker_output, collector.get_result_output(), collector.is_program_writes_stderr()))
            },
        };
        use run::RunErr::*;
        match result {
            Ok((input, duration, should_be_output, program_output, is_program_writes_stderr)) => {
                let mut print_runtime = false;
                match should_be_output {
                    Some(should_be_output) => {
                        let tokenized_program_output = test::Lines::from(program_output);
                        if tokenized_program_output == should_be_output {
                            ok_count += 1;
                            clr!(stdout, b (Color::Green) "Test {} OK", no);
                            if is_program_writes_stderr {
                                clr!(stdout, ", but with "; (Color::Red) "stderr");
                            }
                            if opt.time {
                                clr!(stdout, ", "; (Color::Blue) "run time: "; "{:.1?}", duration);
                            }
                            writeln!(stdout).unwrap();
                        } else {
                            err_count += 1;
                            clrln!(stdout, b (Color::Red) "Test {} ERR", no);
                            collector.print(&mut stdout);

                            clrln!(stdout, n (Color::Blue) "diff with test:");
                            for line in diff::slice(&tokenized_program_output.0, &should_be_output.0) {
                                match line {
                                    diff::Result::Left(a) => {
                                        clrln!(stdout, b n (Color::Green) "+"; (Color::Green) " {}", a)
                                    }
                                    diff::Result::Both(a, _) => println!("  {}", a),
                                    diff::Result::Right(a) => {
                                        clrln!(stdout, b n (Color::Red) "-"; (Color::Red) " {}", a)
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        clrln!(stdout, b (Color::Yellow) "Test {} testing", no);
                        clrln!(stdout, n (Color::Blue) "input:");
                        stdout.write_all(input).unwrap();
                        writeln!(stdout).unwrap();
                        clrln!(stdout, n (Color::Blue) "output:");
                        collector.print(&mut stdout);
                        print_runtime = opt.time;
                    }
                }
                if print_runtime {
                    clrln!(stdout, (Color::Blue) "Run time: "; "{:.1?}", duration);
                }
            }
            Err(Crash) => {
                err_count += 1;
                clrln!(stdout, b (Color::Red) "Test {} crashed", no);

                if !opt.real_time {
                    collector.print(&mut stdout);
                }
            }
            Err(NotRun(error)) => {
                err_count += 1;
                clrln!(stdout, b (Color::Red) "Test {} won't run:", no; " {}", error);
            }
            Err(DuringRun(error)) => {
                err_count += 1;
                clrln!(stdout, b (Color::Red) "Test {} unexpected error during run:", no; " {}", error);
            }
            Err(IoError(error)) => {
                err_count += 1;
                clrln!(stdout, b (Color::Red) "Test {} i/o error:", no; " {}", error);
            }
            Err(InternalError) => {
                err_count += 1;
                clrln!(stdout, b (Color::Red) "Test {} internal error.", no);
            }
        }
    }

    writeln!(stdout).unwrap();

    if err_count == 0 {
        clrln!(stdout, b u (Color::Green) "OK all {} tests.", ok_count);
    } else {
        clrln!(stdout, b u (Color::Green) "OK"; b (Color::Green) ":"; " {} tests, ", ok_count; b u (Color::Red) "ERR";  b (Color::Red) ":"; " {} tests.", err_count);
    }
}
