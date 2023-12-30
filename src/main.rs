use std::process::{Command, Stdio, Child};
use ansi_term::Color::{Green, Red, Black};
use ansi_term::Style;
use std::io::{BufRead, BufReader};
use std::env;

mod pytest;


fn main() {
    let mut child = run_pytest();

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut files: Vec<pytest::File> = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                handle_line(line, &mut files);
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let mut passed_tests_count: u32 = 0;
    let mut failed_tests_count: u32 = 0;

    files.last_mut().unwrap().tests.last_mut().unwrap().error_type = Some(String::from("NameError"));
    files.last_mut().unwrap().tests.last_mut().unwrap().short_error_description = Some(String::from("NameError: name 'asdf' is not defined"));
    // files.last_mut().unwrap().tests.last_mut().unwrap().short_error_description = Some(String::from("FAILED test_test2.py::test_passed_test - NameError: name 'asdf' is not defined"));
    files.last_mut().unwrap().tests.last_mut().unwrap().full_error = Some(String::from(" def test_passed_test():\n\t>       assert 1 == asdf\n\tE       NameError: name 'asdf' is not defined\n\n\ttest_test2.py:6: NameError"));

    for file in files.iter() {
        for test in file.tests.iter() {
            if test.error_type.is_none() {
                passed_tests_count +=1;
            } else {
                test.print_error(String::from(file.get_name()));
                failed_tests_count +=1;
            }
        }
    }

    println!(" {}\t {}{} {}",
        Black.dimmed().paint("Tests"),
        Red.bold().paint(failed_tests_count.to_string() + " failed"),
        Black.dimmed().paint(","),
        Green.bold().paint(passed_tests_count.to_string() + " passed")
    );
}


fn handle_line(line: String, files: &mut Vec<pytest::File>) {
    if line.to_lowercase().contains("python") && line.to_lowercase().contains("pytest") {
        println!("{}", Green.dimmed().paint(&line));
    }

    if pytest::line_contains_test(&line) {
        // if let Some(file) = files.last() {
        if files.last().is_none() || files.last().unwrap().get_name() != pytest::extract_file_name(&line) {
            files.push(pytest::File::new(pytest::extract_file_name(&line)));

            println!("");
            println!("{} {}", Style::new().on(Green).fg(Black).bold().paint(" TESTING "), files.last().unwrap().get_name());
        }

        let test_name = pytest::extract_test_name(&line).replace("test_", "").replace("_", " ");
        files.last_mut().unwrap().tests.push(pytest::Test::new(test_name.clone()));

        if pytest::test_passed(&line) {
            let check_mark = '\u{2714}';
            println!(" {} {}", Green.paint(check_mark.to_string()), test_name);
        } else {
            let x_mark = '\u{2718}';
            files.last_mut().unwrap().tests.last_mut().unwrap().error_type = Some(String::from("something"));

            println!(" {} {}", Red.paint(x_mark.to_string()), test_name);
        }
    }

    // println!("Pytest output: {}", line);
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
