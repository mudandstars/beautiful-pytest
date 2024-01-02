use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader};
use std::env;
use std::path::Path;

mod pytest;
mod print_testing_results;
mod handle_line;


fn main() {
    let mut child = run_pytest();

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut files: Vec<pytest::File> = vec![];

    let mut test_name_of_error_being_read: Option<String> = None;

    for line in reader.lines() {
        match line {
            Ok(line) => handle_line::handle_line(&line, &mut files, &mut test_name_of_error_being_read),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    print_testing_results::print_testing_results(files);
}


fn run_pytest() -> Child {
    let mut pytest_command = Command::new("pytest");
    let args: Vec<String> = env::args().collect();

    let config_file_exists = Path::new("./pytest.ini").exists() || Path::new("./.pytest.ini").exists();

    if args.len() < 2 && ! config_file_exists {
        pytest_command.arg(".");
    } else {
        let filter = &args[1];
        pytest_command.arg(format!("-k {}", filter));
    }
    pytest_command.arg("-v");
    pytest_command.stdout(Stdio::piped());

    return pytest_command.spawn().expect("Failed to start pytest");
}
