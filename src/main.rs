use std::process::{Command, Stdio, Child};
use ansi_term::Color::{Green, Red, Black};
use ansi_term::Style;
use std::io::{BufRead, BufReader, Error};
use std::env;

mod pytest;


fn main() {
    execute_pytest();
}

fn execute_pytest() {
    let mut child = run_pytest();

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut current_file = "".to_string();

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                handle_line(line, &mut current_file);
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn handle_line(line: String, current_file: &mut String) {
    if line.to_lowercase().contains("python") && line.to_lowercase().contains("pytest") {
        println!("{}", Green.dimmed().paint(&line));
    }

    if pytest::line_contains_test(&line) {
        if *current_file != pytest::extract_file_name(&line) {
                *current_file = pytest::extract_file_name(&line);

                println!("");
                println!("{} {}.py", Style::new().on(Green).fg(Black).bold().paint(" TESTING "), *current_file);
        }

        let test_name = pytest::extract_test_name(&line).replace("test_", "").replace("_", " ");
        if pytest::test_passed(&line) {
            let check_mark = '\u{2714}';
            println!(" {} {}", Green.paint(check_mark.to_string()), test_name);
        } else {
            let x_mark = '\u{2718}';
            println!(" {} {}", Red.paint(x_mark.to_string()), test_name);
        }
    }

    println!("Pytest output: {}", line);
}

fn run_pytest() -> Child {
    let mut pytest_command = Command::new("pytest");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        pytest_command.arg(".");
    } else {
        let filter = &args[1];
        pytest_command.arg(format!("-k {}", filter));
    }
    pytest_command.arg("-v");
    // dbg!(args);
    pytest_command.stdout(Stdio::piped());

    return pytest_command.spawn().expect("Failed to start pytest");
}
