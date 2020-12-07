use anyhow::{Context, Result as AhResult};
use std::io::Write;
use std::process::{exit, Command, Stdio};
use termcolor::{Color, ColorChoice, StandardStream, WriteColor};
use termcolor_macro::*;
use toml::Value as Toml;

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut stderr = StandardStream::stdout(ColorChoice::Auto);

    let inputs = {
        clrln!(stdout: n(Color::Black)"Reading `in.txt`...");

        std::fs::read_to_string("in.txt").unwrap_or_else(|_| {
            clrln!(stdout: b (Color::Red) "Input error:"; " file in.txt was not found");
            exit(1)
        })
    };

    let outputs = {
        clrln!(stdout: n (Color::Black)"Reading `out.txt`...");

        std::fs::read_to_string("out.txt").unwrap_or_else(|_| {
            clrln!(stdout: b (Color::Red) "Input error:"; " file `out.txt` was not found");
            exit(1)
        })
    };

    let input_output = inputs
        .split('\\')
        .map(|s| s.trim())
        .zip(outputs.split('\\').map(|s| s.trim()))
        .enumerate()
        .map(|(index, v)| (index+1, v))
        .filter(|(_, (_, out))| !out.is_empty())
        .collect::<Vec<_>>();
    let input_test = inputs
        .split('\\')
        .map(|s| s.trim())
        .enumerate()
        .map(|(index, v)| (index+1, v))
        .skip(input_output.len())
        .collect::<Vec<_>>();

    let program_name = {
        clrln!(stdout: n (Color::Black)"Formatting...");
        Command::new("cargo")
            .arg("fmt")
            .output()
            .map(drop)
            .unwrap_or_else(|err| {
                clrln!(stdout: b (Color::Red) "Format error:"; "{}", err);
            });

        clrln!(stdout: n (Color::Black)"Building release...");

        let output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .output()
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
            let cargo_toml = std::fs::read_to_string("Cargo.toml")?.parse::<Toml>()?;
            Ok("target/release/".to_string()
                + cargo_toml
                    .get("package")
                    .context("no field `package`")?
                    .get("name")
                    .context("no field `package.name`")?
                    .as_str()
                    .context("`package.name` isn't a string")?)
        }()
        .unwrap_or_else(|err| {
            clrln!(stdout: b (Color::Red) "Reading `Cargo.toml` error:"; " {}.", err);
            exit(1)
        });

        clrln!(stdout: n (Color::Black)"Found executable file `{}`.", program_name);

        program_name
    };

    println!();

    let run_for_input = |input: &str| -> (String, String) {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);

        let mut process = Command::new(&program_name).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap_or_else(|err| {
            clrln!(stdout: b (Color::Red) "Internal error:"; " can't run process `{}`, with error: {}.", program_name, err);
            exit(1)
        });
        let mut stdin = process.stdin.take().unwrap_or_else(|| {
            clrln!(stdout: b (Color::Red) "Internal error:"; " can't get stdin of `{}` to write.", program_name);
            exit(1)
        });
        drop(stdin.write_all(input.as_bytes()));
        drop(stdin);
        let output = process.wait_with_output().unwrap_or_else(|err| {
            clrln!(stdout: b (Color::Red) "Internal error:"; " error waiting for process `{}`, with error: {}.", program_name, err);
            exit(1)
        });

        if !output.status.success() {
            clrln!(stdout: b (Color::Red) "Running test error:");
            drop(std::io::stdout().write_all(&output.stdout));
            drop(std::io::stderr().write_all(&output.stderr));
            exit(output.status.code().unwrap_or(1));
        }

        (String::from_utf8(output.stdout).unwrap_or_else(|err| {
            clrln!(stdout: b (Color::Red) "Internal error:"; " stdout for process `{}` was not UTF8 encoded: {}.", program_name, err);
            exit(1)
        }),
        String::from_utf8(output.stderr).unwrap_or_else(|err| {
            clrln!(stdout: b (Color::Red) "Internal error:"; " stderr for process `{}` was not UTF8 encoded: {}.", program_name, err);
            exit(1)
        }))
    };

    let mut ok_count = 0;
    let mut err_count = 0;
    for (no, (i, o)) in input_output {
        let (resulted, err) = run_for_input(i);
        if resulted.trim() == o.trim() {
            ok_count += 1;
            clrln!(stdout: b (Color::Green) "Test {} OK", no);
        } else {
            err_count += 1;
            clrln!(stdout: b (Color::Red) "Test {} ERR", no);
            clrln!(stdout: n (Color::Blue) "input:");
            println!("{}\n", i);
            clrln!(stdout: n (Color::Blue) "output:");
            for line in diff::lines(resulted.trim(), o.trim()) {
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
            println!();
            if !err.trim().is_empty() {
                clrln!(stdout: n (Color::Blue) "stderr:");
                println!("{}\n", err.trim());
            }
        }
    }

    if !input_test.is_empty() {
        println!();
    }

    for (no, i) in input_test {
        let (resulted, err) = run_for_input(i);
        clrln!(stdout: b (Color::Yellow) "Test {} testing", no);
        clrln!(stdout: n (Color::Blue) "input:");
        println!("{}\n", i);
        clrln!(stdout: n (Color::Blue) "output:");
        println!("{}\n", resulted.trim());
        if !err.trim().is_empty() {
            clrln!(stdout: n (Color::Blue) "stderr:");
            println!("{}\n", err.trim());
        }
    }

    if err_count == 0 {
        clrln!(stdout: b u (Color::Green) "OK all {} tests.", ok_count);
    } else {
        clrln!(stdout: b u (Color::Green) "OK"; b (Color::Green) ":"; " {} tests, ", ok_count; b u (Color::Red) "ERR";  b (Color::Red) ":"; " {} tests.", err_count);
    }
}
