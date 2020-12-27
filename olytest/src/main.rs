use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{exit, Command};

use structopt::StructOpt;
use termcolor::{Color, ColorChoice, StandardStream, WriteColor};
use termcolor_macro::*;

use olytest::codeforces;
use olytest::display::JoinedByTrait;
use olytest::files;
use olytest::run;
use olytest::test;

/// Program to run tests as easy and as fast as possible during competitive programming.
///
/// Write tests for program in file `tests/<name>.test`, when each test separated by `\` symbol. Then, inside test, input and output may be separated by '~' or '%'.
/// * '~' is used, when you need simple testing.
/// * '%' is used, when you need checker.
/// * This symbols can be escaped by writing twice: "~~", "%%", "\\".
/// * If none of this symbols is on the test, then output for program just printed on screen for visual testing.
#[derive(Debug, StructOpt)]
#[structopt(name = "olytest", author = "Ilya Sheprut a.k.a. optozorax")]
struct Opt {
    /// Absolute path to template folder, used in generation of executables.
    #[structopt(
        short,
        long,
        env = "OLYTEST_TEMPLATE_ABSOLUTE_PATH",
        parse(from_os_str)
    )]
    template_path: PathBuf,

    #[structopt(subcommand)]
    subcommand: OptEnum,
}

#[derive(Debug, StructOpt)]
enum OptEnum {
    /// Init this folder with provided executables names.
    #[structopt(visible_alias = "i")]
    Init { files: Vec<String> },

    /// Init this folder from codeforces.com contest. Names and tests will be parsed from contest page.
    #[structopt(visible_alias = "cf")]
    Codeforces { number: u64 },

    /// Run tests for provided executable.
    #[structopt(visible_alias = "t")]
    Test {
        /// Name of binary to run. Examples `a`, `b`, `e1`.
        name: String,

        #[structopt(flatten)]
        settings: TestSettings,
    },

    /// Run tests for all executables.
    #[structopt(visible_alias = "ta")]
    TestAll {
        #[structopt(flatten)]
        settings: TestSettings,
    },

    /// Generate executable for given name.
    #[structopt(visible_alias = "ge")]
    GenerateExecutable {
        /// Name of binary to generate. Examples `a`, `b`, `e1`.
        name: String,
    },

    /// Generate checker for executable with given name.
    #[structopt(visible_alias = "gc")]
    GenerateChecker {
        /// Name of binary to generate checker for. Example: `gc a` will generate `a_checker`.
        name: String,
    },

    /// Remove existing binary from project.
    #[structopt(visible_alias = "re")]
    RemoveExecutable {
        /// Name of binary to remove.        
        name: String,
    },

    /// Remove existing checker from project. Example: `rc a` will remove `a_checker`.
    #[structopt(visible_alias = "rc")]
    RemoveChecker {
        /// Name of binary to remove checker for.
        name: String,
    },
}

#[derive(Debug, StructOpt)]
struct TestSettings {
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

fn compile<T: WriteColor>(stdout: &mut T, name: &str, opt: &TestSettings) -> String {
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

fn format<T: WriteColor>(stdout: &mut T) {
    clrln!(stdout, n (Color::Black) "Formatting...");
    Command::new("cargo")
        .arg("fmt")
        .output()
        .map(drop)
        .unwrap_or_else(|err| {
            clrln!(stdout, b (Color::Red) "Format error:"; "{}", err);
        });
}

fn test<Out: WriteColor>(mut stdout: &mut Out, name: &str, opt: &TestSettings) {
    let test_file_name = format!("tests/{}.test", name);
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
        format(&mut stdout);
    }

    files::generate_without_include(&files::format_path_buf(&["src", &name], "rs")).unwrap_or_else(|err| {
        match err {
            files::GenerateError::OtherFiles(err) => clrln!(stdout, b (Color::Red) "During generation of file without include's."; " {}", err),
            files::GenerateError::IncludeError(err) => clrln!(stdout, b (Color::Red) "File for include not found: "; " {}", err.file.display()),
        }
        exit(1)
    });

    let program_file_name = compile(&mut stdout, &name, &opt);
    let checker_file_name = if has_checker {
        compile(&mut stdout, format!("{}_checker", name).as_ref(), &opt)
    } else {
        "internal error".to_string()
    };

    println!();

    let mut ok_count = 0;
    let mut err_count = 0;
    for (no, test) in tests.into_iter().enumerate().map(|(i, x)| (i + 1, x)) {
        let mut collector = run::Collector::new(opt.real_time, true, opt.escape_output, false);
        let (input, result) = match test {
            test::TokenizedTest::Simple { input, output } => {
                let run_result = run::run_program(
                    &program_file_name,
                    input,
                    opt.debug,
                    &mut stdout,
                    &mut collector,
                );
                (
                    input,
                    run_result.map(|duration| {
                        (
                            duration,
                            output,
                            collector.get_result_output(),
                            collector.is_program_writes_stderr(),
                        )
                    }),
                )
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
                (
                    checker_input,
                    run_result.map(|duration| {
                        (
                            duration,
                            checker_output,
                            collector.get_result_output(),
                            collector.is_program_writes_stderr(),
                        )
                    }),
                )
            }
        };
        use run::RunErr::*;
        struct TestResult<'a> {
            verdict_color: Color,
            verdict: Cow<'static, str>,
            is_program_writes_stderr: bool,
            is_print_input_output: bool,
            should_be_output_diff: Option<(test::Lines<'a>, test::Lines<'a>)>,
            time: Option<std::time::Duration>,
        }
        let TestResult {
            verdict_color,
            verdict,
            is_program_writes_stderr,
            is_print_input_output,
            should_be_output_diff,
            time,
        } = match result {
            Ok((duration, should_be_output, program_output, is_program_writes_stderr)) => {
                let tokenized_program_output = test::Lines::from(program_output);
                match should_be_output {
                    Some(should_be_output) => {
                        if tokenized_program_output == should_be_output {
                            ok_count += 1;
                            TestResult {
                                verdict_color: Color::Green,
                                verdict: Cow::Borrowed("OK"),
                                is_program_writes_stderr,
                                is_print_input_output: opt.show_output,
                                should_be_output_diff: None,
                                time: if opt.time { Some(duration) } else { None },
                            }
                        } else {
                            err_count += 1;
                            TestResult {
                                verdict_color: Color::Red,
                                verdict: Cow::Borrowed("ERR"),
                                is_program_writes_stderr,
                                is_print_input_output: true,
                                should_be_output_diff: Some((
                                    tokenized_program_output,
                                    should_be_output,
                                )),
                                time: if opt.time { Some(duration) } else { None },
                            }
                        }
                    }
                    None => TestResult {
                        verdict_color: Color::Yellow,
                        verdict: Cow::Borrowed("CHECK"),
                        is_program_writes_stderr,
                        is_print_input_output: true,
                        should_be_output_diff: None,
                        time: if opt.time { Some(duration) } else { None },
                    },
                }
            }
            Err(Crash) => {
                err_count += 1;
                TestResult {
                    verdict_color: Color::Red,
                    verdict: Cow::Borrowed("crashed"),
                    is_program_writes_stderr: false,
                    is_print_input_output: true,
                    should_be_output_diff: None,
                    time: None,
                }
            }
            Err(NotRun(error)) => {
                err_count += 1;
                TestResult {
                    verdict_color: Color::Red,
                    verdict: Cow::Owned(format!("won't run: {:?}", error)),
                    is_program_writes_stderr: false,
                    is_print_input_output: true,
                    should_be_output_diff: None,
                    time: None,
                }
            }
            Err(DuringRun(error)) => {
                err_count += 1;
                TestResult {
                    verdict_color: Color::Red,
                    verdict: Cow::Owned(format!("unexpected error during run: {:?}", error)),
                    is_program_writes_stderr: false,
                    is_print_input_output: true,
                    should_be_output_diff: None,
                    time: None,
                }
            }
            Err(IoError(error)) => {
                err_count += 1;
                TestResult {
                    verdict_color: Color::Red,
                    verdict: Cow::Owned(format!("I/O error: {:?}", error)),
                    is_program_writes_stderr: false,
                    is_print_input_output: true,
                    should_be_output_diff: None,
                    time: None,
                }
            }
            Err(InternalError) => {
                err_count += 1;
                TestResult {
                    verdict_color: Color::Red,
                    verdict: Cow::Borrowed("internal error"),
                    is_program_writes_stderr: false,
                    is_print_input_output: true,
                    should_be_output_diff: None,
                    time: None,
                }
            }
        };
        clr!(stdout, b (verdict_color) "Test {} {}", no, verdict);
        if is_program_writes_stderr {
            clr!(stdout, ", but with "; (Color::Red) "stderr");
        }
        if let Some(time) = time {
            clr!(stdout, ", "; (Color::Blue) "run time: "; "{:.1?}", time);
        }
        writeln!(stdout).unwrap();
        if is_print_input_output && !opt.real_time {
            clrln!(stdout, n (Color::Blue) "input:");
            stdout.write_all(input).unwrap();
            writeln!(stdout).unwrap();
            clrln!(stdout, n (Color::Blue) "output:");
            collector.print(&mut stdout);
            writeln!(stdout).unwrap();
        }
        if let Some((tokenized_program_output, should_be_output)) = should_be_output_diff {
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
            writeln!(stdout).unwrap();
        }
    }

    writeln!(stdout).unwrap();

    if err_count == 0 {
        clrln!(stdout, b u (Color::Green) "OK all {} tests.", ok_count);
    } else {
        clrln!(stdout, b u (Color::Green) "OK"; b (Color::Green) ":"; " {} tests, ", ok_count; b u (Color::Red) "ERR";  b (Color::Red) ":"; " {} tests.", err_count);
    }
}

fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let opt = Opt::from_args();

    let relative_path_to_template = files::make_template_relavite_path(&opt.template_path);

    use OptEnum::*;

    match opt.subcommand {
        Init { files } => {
            clrln!(stdout, n (Color::Black) "Initialize files...");
            files::init_common().context("Error in common files init")?;
            for name in files {
                clrln!(stdout, n (Color::Black) "Create executable `{}`...", name);
                files::create_executable(&name, &relative_path_to_template)
                    .with_context(|| format!("Error during creation of executable `{}`", name))?;
            }
        }
        Codeforces { number } => {
            clrln!(stdout, n (Color::Black) "Initialize files...");
            files::init_common().context("common files init")?;
            let problems =
                codeforces::get_problems(number).context("Can't get codeforces problems")?;
            clrln!(stdout, b (Color::Green) "Found problems: "; " {}", problems.iter().joined_by(" "));
            for name in &problems {
                clrln!(stdout, n (Color::Black) "Create executable `{}`...", name);
                files::create_executable(&name, &relative_path_to_template)
                    .with_context(|| format!("Error during creation of executable `{}`", name))?;
            }
            for name in &problems {
                match codeforces::get_tests(number, &name) {
                    Ok(tests) => {
                        if tests.0.is_empty() {
                            clrln!(stdout, (Color::Red) "Tests not found for `{}`.", name);
                        } else {
                            clrln!(stdout, n (Color::Black) "Write found tests for `{}`...", name);
                            files::write_test(name, tests.to_string()).unwrap();
                        }
                    }
                    Err(err) => {
                        clrln!(stdout, b (Color::Red) "Error during getting test for `{}`:", name; "{:?}", err);
                    }
                }
            }
        }
        Test { name, settings } => {
            test(&mut stdout, &name, &settings);
        }
        TestAll { settings } => {
            let all_executables = files::get_current_executables()
                .context("Error during get list of all executables")?;
            for name in all_executables {
                clrln!(stdout, b u (Color::Cyan) "Testing executable `{}`.", name);
                test(&mut stdout, &name, &settings);
                writeln!(stdout).unwrap();
            }
        }
        GenerateExecutable { name } => {
            files::create_executable(&name, &relative_path_to_template)
                .with_context(|| format!("Error during creation of executable `{}`", name))?;
        }
        GenerateChecker { name } => {
            files::create_checker(&name, &relative_path_to_template)
                .with_context(|| format!("Error during creation of checker `{}_checker`", name))?;
        }
        RemoveExecutable { name } => {
            files::remove_executable(&name)
                .with_context(|| format!("Error during removing executable `{}`", name))?;
        }
        RemoveChecker { name } => {
            files::remove_checker(&name)
                .with_context(|| format!("Error during removing checker `{}_checker`", name))?;
        }
    }
    Ok(())
}
