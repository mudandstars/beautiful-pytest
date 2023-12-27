use std::process::{Command, Stdio};
use ansi_term::Color::{Green, Red, Yellow};
use ansi_term::Style;
use std::io::{BufRead, BufReader};


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
    let mut current_file = "none".to_string();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Process the line here (print or do something else)
                if line.to_lowercase().contains("python") && line.to_lowercase().contains("pytest") {
                    println!("{}", Green.dimmed().paint(&line));
                }
                // filename with color background, then for each test within a file:
                if line_contains_test(&line) {
                    if current_file != extract_file_name(&line) {
                            current_file = extract_file_name(&line);
                            println!("file: {}", Green.paint(&current_file));
                    }

                    if test_passed(&line) {
                        println!("{} {}", Green.paint("check: "), extract_test_name(&line));
                    } else {
                        println!("{} {}", Red.paint("fail: "), extract_test_name(&line));
                    }
                }

                println!("Pytest output: {}", line);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }

        // Execute `ls` in the current directory of the program.
    match pytest_command.status() {
            Ok(status) => println!("Command executed with status: {}", status),
            Err(e) => eprintln!("process failed to execute: {:?}", e),
        }
    }

fn line_contains_test(line: &str) -> bool {
    return line.contains(".py::") && (line.contains(" PASSED") || line.contains(" FAILED"));
}

fn test_passed(line: &str) -> bool {
    return line.contains("PASSED");
}

fn extract_test_name(line: &str) -> String {
    let start = line.find(".py::").unwrap();

    let end: usize;
    if line.contains("PASSED") {
        end = line.rfind(" PASSED").unwrap();
    } else {
        end = line.rfind(" FAILED").unwrap();
    }

    let start_idx = start + ".py::".len();
    return line[start_idx..end].to_string();
}

fn extract_file_name(line: &str) -> String {
    let end = line.rfind(".py").unwrap();

    return line[0..end].to_string();
}
