use std::process::{Command, Stdio};
use ansi_term::Color::{Green, Red, Black};
use ansi_term::Style;
use std::io::{BufRead, BufReader, Error};

mod pytest;


fn main() {
    execute_pytest();
}

fn execute_pytest() {
    let mut pytest_command = Command::new("pytest");
    pytest_command.arg(".");
    pytest_command.arg("-v");
    pytest_command.stdout(Stdio::piped());

    let mut child = match pytest_command.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to start pytest: {}", e);
            return;
        }
    };

    let stdout = child.stdout.take().expect("Failed to capture stdout");

    let reader = BufReader::new(stdout);
    let mut current_file = "".to_string();
    for line in reader.lines() {
        handle_line(line, &mut current_file);
    }

    println!("");
    match pytest_command.status() {
            Ok(status) => println!("Command executed with status: {}", status),
            Err(e) => eprintln!("process failed to execute: {:?}", e),
    }
}

fn handle_line(line: Result<String, Error>, current_file: &mut String) {
    match line {
        Ok(line) => {
            // Process the line here (print or do something else)
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

            // println!("Pytest output: {}", line);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
