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
    about = "Program to run tests as easy and as fast as possible during competitive programming.\nWrite tests for program in file `tests/<name>.test`, when each test separated by `\\` symbol. Then, inside test, input and output may be separated by '~' or '%'.\n * '~' is used, when you need simple testing.\n * '%' is used, when you need checker.\n * This symbols can be escaped by writing twice: \"~~\", \"%%\", \"\\\\\".\n * If none of this symbols is on the test, then output for program just printed on screen for visual testing.",
    author = "Ilya Sheprut a.k.a. optozorax"
)]
struct Opt {
    /// Run executable in debug mode, and with env `RUST_BACKTRACE=full`.
    #[structopt(short, long)]
    debug: bool,

    /// Show output of program even if test passed.
    #[structopt(short = "o", long)]
    show_output: bool,

    /// Also show run time.
    #[structopt(short, long)]
    time: bool,

    /// Don't format code before compiling.
    #[structopt(short = "f", long)]
    not_format: bool,

    /// Show program output in real time.
    #[structopt(short = "r", long)]
    real_time: bool,

    /// With this option, output of program and checker will be escaped: line endings become '\n' and etc.
    #[structopt(short = "s", long)]
    escape_output: bool,

    /// Name of binary to run. Examples `a`, `b`, `e1`.
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

    // TODO не ложить всю программу, а возвращать в виде ошибки, на случай если есть несколько программ для тестирования.
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

    if !opt.not_format {
        format(&mut stdout, &opt);
    }

    let program_file_name = compile(&mut stdout, &opt.name, &opt);
    let checker_file_name = if has_checker {
        compile(&mut stdout, format!("{}_checker", opt.name).as_ref(), &opt)
    } else {
        "internal error".to_string()
    };

    println!();

    let mut ok_count = 0;
    let mut err_count = 0;
    for (no, test) in tests.into_iter().enumerate().map(|(i, x)| (i + 1, x)) {
        let mut collector = run::Collector::new(opt.real_time, true, opt.escape_output, false);
        let result = match test {
            test::TokenizedTest::Simple { input, output } => {
                let run_result = run::run_program(
                    &program_file_name,
                    input,
                    opt.debug,
                    &mut stdout,
                    &mut collector,
                );
                run_result.map(|duration| {
                    (
                        input,
                        duration,
                        output,
                        collector.get_result_output(),
                        collector.is_program_writes_stderr(),
                    )
                })
            }
            test::TokenizedTest::WithChecker {
                checker_input,
                checker_output,
            } => {
                let run_result = run::run_with_checker(
                    &program_file_name,
                    &checker_file_name,
                    checker_input,
                    opt.debug,
                    &mut stdout,
                    &mut collector,
                );
                run_result.map(|duration| {
                    (
                        checker_input,
                        duration,
                        checker_output,
                        collector.get_result_output(),
                        collector.is_program_writes_stderr(),
                    )
                })
            }
        };
        use run::RunErr::*;
        match result {
            Ok((input, duration, should_be_output, program_output, is_program_writes_stderr)) => {
                let tokenized_program_output = test::Lines::from(program_output);
                let (verdict_color, verdict, is_print_input_output, should_be_output_diff) = match should_be_output {
                    Some(should_be_output) => {
                        if tokenized_program_output == should_be_output {
                            ok_count += 1;
                            (Color::Green, "OK", opt.show_output, None)
                        } else {
                            err_count += 1;
                            (Color::Red, "ERR", true, Some(should_be_output))
                        }
                    }
                    None => {
                        (Color::Yellow, "CHECK", true, None)
                    }
                };
                clr!(stdout, b (verdict_color) "Test {} {}", no, verdict);
                if is_program_writes_stderr {
                    clr!(stdout, ", but with "; (Color::Red) "stderr");
                }
                if opt.time {
                    clr!(stdout, ", "; (Color::Blue) "run time: "; "{:.1?}", duration);
                }
                writeln!(stdout).unwrap();
                if is_print_input_output {
                    clrln!(stdout, n (Color::Blue) "input:");
                    stdout.write_all(input).unwrap();
                    writeln!(stdout).unwrap();
                    clrln!(stdout, n (Color::Blue) "output:");
                    collector.print(&mut stdout);
                    writeln!(stdout).unwrap();    
                }
                if let Some(should_be_output) = should_be_output_diff {
                    clrln!(stdout, n (Color::Blue) "diff with test:");
                    for line in
                        diff::slice(&tokenized_program_output.0, &should_be_output.0)
                    {
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
                    writeln!(stdout).unwrap();
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
