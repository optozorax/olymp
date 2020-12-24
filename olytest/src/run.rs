use std::ops::Range;
use std::process::Stdio;
use std::time::{Duration, Instant};

use termcolor::{Color, WriteColor};
use termcolor_macro::*;

fn write_escaped<T: WriteColor>(
    output: &mut T,
    data: &[u8],
    escape_output: bool,
    buffer: &mut Vec<u8>,
) -> bool {
    if escape_output {
        buffer.clear();
        for s in data {
            for c in std::ascii::escape_default(*s) {
                buffer.push(c);
            }
        }
        output.write_all(&buffer).unwrap();
        buffer.last().map(|x| *x == b'\n').unwrap_or(false)
    } else {
        output.write_all(data).unwrap();
        data.last().map(|x| *x == b'\n').unwrap_or(false)
    }
}

fn print_checker_stdout<T: WriteColor>(output: &mut T) {
    clr!(output, b (Color::Yellow) "[CHECKER] ");
}
fn print_checker_stderr<T: WriteColor>(output: &mut T) {
    clr!(output, b (Color::Red) "(CHECKER) ");
}
fn print_program_stdout<T: WriteColor>(output: &mut T) {
    clr!(output, b (Color::Blue) "[PROGRAM] ");
}
fn print_program_stderr<T: WriteColor>(output: &mut T) {
    clr!(output, b n (Color::Red) "(PROGRAM) ");
}

fn print_out_with_checker<T: WriteColor>(
    output: &mut T,
    data: &[u8],
    escape_stdout: bool,
    escape_stderr: bool,
    buffer: &mut Vec<u8>,
    kind: PrintTypeWithChecker,
    previous: &mut PrintTypeWithChecker,
    previous_was_ln: bool,
) -> bool {
    use PrintTypeWithChecker::*;
    if kind != *previous && !data.is_empty() {
        if !(*previous == Nothing || previous_was_ln) {
            buffer.clear();
            buffer.push(b'\n');
            output.write_all(&buffer).unwrap();
        }
        *previous = kind;
        match kind {
            Nothing => {}
            ProgramStdout => print_program_stdout(output),
            ProgramStderr => print_program_stderr(output),
            CheckerStdout => print_checker_stdout(output),
            CheckerStderr => print_checker_stderr(output),
        }
    }
    write_escaped(
        output,
        data,
        match kind {
            Nothing => escape_stdout,
            ProgramStdout => escape_stdout,
            ProgramStderr => escape_stderr,
            CheckerStdout => escape_stdout,
            CheckerStderr => escape_stderr,
        },
        buffer,
    )
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum PrintTypeWithChecker {
    Nothing,
    ProgramStdout,
    ProgramStderr,
    CheckerStdout,
    CheckerStderr,
}

#[derive(Debug)]
pub enum RunErr {
    Crash,
    NotRun(std::io::Error),
    DuringRun(std::io::Error),
    IoError(std::io::Error),
    InternalError,
}

pub struct Collector {
    previous: PrintTypeWithChecker,
    buffer: Vec<u8>,
    real_time_print: bool,
    collect_to_print: bool,
    escape_stdout: bool,
    escape_stderr: bool,
    result: Vec<u8>,
    to_print: Vec<u8>,
    to_print_types: Vec<(PrintTypeWithChecker, Range<usize>)>,
    is_program_writes_stderr: bool,
    previous_was_ln: bool,
}

impl Collector {
    pub fn new(
        real_time_print: bool,
        collect_to_print: bool,
        escape_stdout: bool,
        escape_stderr: bool,
    ) -> Self {
        Self {
            previous: PrintTypeWithChecker::Nothing,
            buffer: Vec::with_capacity(100),
            real_time_print,
            collect_to_print,
            escape_stdout,
            escape_stderr,
            result: Vec::new(),
            to_print: Vec::new(),
            to_print_types: Vec::new(),
            is_program_writes_stderr: false,
            previous_was_ln: true,
        }
    }

    fn collect<T: WriteColor>(&mut self, input: &[u8], kind: PrintTypeWithChecker, output: &mut T) {
        if self.real_time_print {
            self.previous_was_ln = print_out_with_checker(
                output,
                input,
                self.escape_stdout,
                self.escape_stderr,
                &mut self.buffer,
                kind,
                &mut self.previous,
                self.previous_was_ln,
            );
        }
        if self.collect_to_print {
            let start = self.to_print.len();
            let end = start + input.len();
            self.to_print.extend(input);
            self.to_print_types.push((kind, start..end));
        }
    }

    fn collect_checker_stdout<T: WriteColor>(&mut self, input: &[u8], output: &mut T) {
        self.collect(input, PrintTypeWithChecker::CheckerStdout, output);
    }
    fn collect_checker_stderr<T: WriteColor>(&mut self, input: &[u8], output: &mut T) {
        self.collect(input, PrintTypeWithChecker::CheckerStderr, output);
    }
    fn collect_program_stdout<T: WriteColor>(&mut self, input: &[u8], output: &mut T) {
        self.collect(input, PrintTypeWithChecker::ProgramStdout, output);
    }
    fn collect_program_stderr<T: WriteColor>(&mut self, input: &[u8], output: &mut T) {
        self.is_program_writes_stderr = true;
        self.collect(input, PrintTypeWithChecker::ProgramStderr, output);
    }

    fn collect_result(&mut self, input: &[u8]) {
        self.result.extend(input);
    }

    pub fn print<T: WriteColor>(&self, output: &mut T) {
        let mut previous = PrintTypeWithChecker::Nothing;
        let mut buffer = Vec::with_capacity(100);
        let mut previous_was_ln = true;
        for (kind, range) in &self.to_print_types {
            previous_was_ln = print_out_with_checker(
                output,
                &self.to_print[range.clone()],
                self.escape_stdout,
                self.escape_stderr,
                &mut buffer,
                *kind,
                &mut previous,
                previous_was_ln,
            );
        }
    }

    pub fn get_result_output(&self) -> &[u8] {
        &self.result[..]
    }

    pub fn is_program_writes_stderr(&self) -> bool {
        self.is_program_writes_stderr
    }
}

#[tokio::main]
pub async fn run_with_checker<T: WriteColor>(
    path_to_program: &str,
    path_to_checker: &str,
    input: &[u8],
    debug_flags: bool,
    output: &mut T,
    collector: &mut Collector,
) -> Result<Duration, RunErr> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::process::Command;

    let mut program = {
        let mut process = Command::new(&path_to_program);
        if debug_flags {
            process.env("RUST_BACKTRACE", "full");
        }

        process.stdout(Stdio::piped()).stderr(Stdio::piped());
        process
    }
    .stdin(Stdio::piped())
    .spawn()
    .map_err(RunErr::NotRun)?;

    let mut checker = {
        let mut process = Command::new(&path_to_checker);
        if debug_flags {
            process.env("RUST_BACKTRACE", "full");
        }

        process.stdout(Stdio::piped()).stderr(Stdio::piped());
        process
    }
    .stdin(Stdio::piped())
    .spawn()
    .map_err(RunErr::NotRun)?;

    let mut program_stdin = program.stdin.take().ok_or(RunErr::InternalError)?;
    let mut program_stdout = program.stdout.take().ok_or(RunErr::InternalError)?;
    let mut program_stderr = program.stderr.take().ok_or(RunErr::InternalError)?;
    let mut checker_stdin = checker.stdin.take().ok_or(RunErr::InternalError)?;
    let mut checker_stdout = checker.stdout.take().ok_or(RunErr::InternalError)?;
    let mut checker_stderr = checker.stderr.take().ok_or(RunErr::InternalError)?;

    let time = Instant::now();
    match checker_stdin
        .write_all(input)
        .await
        .map_err(|x| (x.kind(), x))
    {
        Ok(_) => {}
        Err((std::io::ErrorKind::BrokenPipe, _)) => {} // Ignore broken pipe because we write to closed channel, and we don't care about it
        Err((_, err)) => return Err(RunErr::IoError(err)),
    }

    let mut buffer1 = Vec::with_capacity(1000);
    let mut buffer2 = Vec::with_capacity(1000);
    let mut buffer3 = Vec::with_capacity(1000);
    let mut buffer4 = Vec::with_capacity(1000);

    let mut checker_exited = None;
    let mut program_exited = None;

    let mut checker_stdout_closed = false;
    let mut checker_stderr_closed = false;
    let mut program_stdout_closed = false;
    let mut program_stderr_closed = false;

    loop {
        buffer1.clear();
        buffer2.clear();
        buffer3.clear();
        buffer4.clear();
        tokio::select! {
            readed = checker_stdout.read_buf(&mut buffer1) => {
                if readed.unwrap() != 0 {
                    collector.collect_checker_stdout(&buffer1, output);
                    match program_stdin
                        .write_all(&buffer1)
                        .await
                        .map_err(|x| (x.kind(), x))
                    {
                        Ok(_) => {}
                        // Ignore broken pipe because we can write to closed channel, and we don't care about it
                        Err((std::io::ErrorKind::BrokenPipe, _)) => {}
                        Err((_, err)) => return Err(RunErr::IoError(err)),
                    }
                } else {
                    checker_stdout_closed = true;
                }
            },
            readed = checker_stderr.read_buf(&mut buffer2) => {
                if readed.unwrap() != 0 {
                    collector.collect_checker_stderr(&buffer2, output);
                    collector.collect_result(&buffer2);
                } else {
                    checker_stderr_closed = true;
                }
            },
            readed = program_stdout.read_buf(&mut buffer3) => {
                if readed.unwrap() != 0 {
                    collector.collect_program_stdout(&buffer3, output);
                    match checker_stdin
                        .write_all(&buffer3)
                        .await
                        .map_err(|x| (x.kind(), x))
                    {
                        Ok(_) => {}
                        // Ignore broken pipe because we write to closed channel, and we don't care about it
                        Err((std::io::ErrorKind::BrokenPipe, _)) => {}
                        Err((_, err)) => return Err(RunErr::IoError(err)),
                    }
                } else {
                    program_stdout_closed = true;
                }
            },
            readed = program_stderr.read_buf(&mut buffer4) => {
                if readed.unwrap() != 0 {
                    collector.collect_program_stderr(&buffer4, output);
                } else {
                    program_stderr_closed = true;
                }
            },
        }
        if let Ok(Some(status)) = program.try_wait() {
            program_exited = Some(status);
        }
        if let Ok(Some(status)) = checker.try_wait() {
            checker_exited = Some(status);
        }
        let program_crashed = program_exited.map(|x| !x.success()).unwrap_or(false)
            && program_stdout_closed
            && program_stderr_closed;
        let checker_crashed = checker_exited.map(|x| !x.success()).unwrap_or(false)
            && checker_stderr_closed
            && checker_stdout_closed;
        let checker_done = checker_exited.map(|x| x.success()).unwrap_or(false)
            && checker_stderr_closed
            && checker_stdout_closed;
        let program_done = program_exited.map(|x| x.success()).unwrap_or(false)
            && program_stdout_closed
            && program_stderr_closed;
        let everything_done_correctly = checker_done && program_done;
        if program_crashed || checker_crashed || everything_done_correctly {
            break;
        }
    }
    let duration = time.elapsed();

    if let Some(status) = checker_exited {
        if !status.success() {
            return Err(RunErr::Crash);
        }
    }
    if let Some(status) = program_exited {
        if !status.success() {
            return Err(RunErr::Crash);
        }
    }

    Ok(duration)
}

#[tokio::main]
pub async fn run_program<T: WriteColor>(
    path: &str,
    input: &[u8],
    debug_flags: bool,
    output: &mut T,
    collector: &mut Collector,
) -> Result<Duration, RunErr> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::process::Command;

    let mut program = {
        let mut process = Command::new(&path);

        if debug_flags {
            process.env("RUST_BACKTRACE", "full");
        }

        process.stdout(Stdio::piped()).stderr(Stdio::piped());
        process
    }
    .stdin(Stdio::piped())
    .spawn()
    .map_err(RunErr::NotRun)?;

    let mut stdin = program.stdin.take().ok_or(RunErr::InternalError)?;
    let mut stdout = program.stdout.take().ok_or(RunErr::InternalError)?;
    let mut stderr = program.stderr.take().ok_or(RunErr::InternalError)?;

    match stdin.write_all(input).await.map_err(|x| (x.kind(), x)) {
        Ok(_) => {}
        Err((std::io::ErrorKind::BrokenPipe, _)) => {} // Ignore broken pipe because we write to closed channel, and we don't care about it
        Err((_, err)) => return Err(RunErr::IoError(err)),
    }

    let time = Instant::now();

    let mut buffer1 = Vec::with_capacity(1000);
    let mut buffer2 = Vec::with_capacity(1000);

    let mut stdout_closed = false;
    let mut stderr_closed = false;

    let exit_status = loop {
        buffer1.clear();
        buffer2.clear();
        tokio::select! {
            readed = stdout.read_buf(&mut buffer1) => {
                if readed.unwrap() != 0 {
                    collector.collect_program_stdout(&buffer1, output);
                    collector.collect_result(&buffer1);
                } else {
                    stdout_closed = true;
                }
            },
            readed = stderr.read_buf(&mut buffer2) => {
                if readed.unwrap() != 0 {
                    collector.collect_program_stderr(&buffer2, output);
                } else {
                    stderr_closed = true;
                }
            },
        };
        if let Ok(Some(status)) = program.try_wait() {
            if status.success() && stdout_closed && stderr_closed {
                break status;
            }
        }
    };
    let duration = time.elapsed();

    if exit_status.success() {
        Ok(duration)
    } else {
        Err(RunErr::Crash)
    }
}
