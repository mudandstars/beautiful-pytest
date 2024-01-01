use ansi_term::Color::{Green, Red, Black};
use ansi_term::Style;


use crate::pytest::*;


pub fn handle_line(line: &str, files: &mut Vec<File>, test_name_of_error_being_read: &mut Option<String>) {
    if line.to_lowercase().contains("python") && line.to_lowercase().contains("pytest") {
        println!("{}", Green.dimmed().paint(line));
    }

    if line_contains_test(&line) {
        // if let Some(file) = files.last() {
        if files.last().is_none() || files.last().unwrap().get_name() != extract_file_name(&line) {
            files.push(File::new(extract_file_name(line)));

            println!("");
            println!("{} {}", Style::new().on(Green).fg(Black).bold().paint(" TESTING "), files.last().unwrap().get_name());
        }

        let test_name = extract_test_name(&line);
        files.last_mut().unwrap().tests.push(Test::new(test_name.clone()));

        if test_passed(&line) {
            let check_mark = '\u{2714}';
            println!(" {} {}", Green.paint(check_mark.to_string()), test_name);
        } else {
            let x_mark = '\u{2718}';
            files.last_mut().unwrap().tests.last_mut().unwrap().error_type = Some(String::from("something"));

            println!(" {} {}", Red.paint(x_mark.to_string()), test_name);
        }
    }

    // set current file being read
    let test_name_of_the_error = is_error_header_for_test(line);
    if test_name_of_the_error.is_some() {
        *test_name_of_error_being_read = Some(test_name_of_the_error.unwrap().to_string());
    }

    if test_name_of_error_being_read.is_some() {
        if line.contains("short test summary info") {
            *test_name_of_error_being_read = None;
        } else if ! line.contains(FAILED_TEST_HEAD_LINE_SEPARATOR) && line.len() > 0 {
            for file in files.iter_mut() {
                for test in file.tests.iter_mut() {
                    if test.name == *test_name_of_error_being_read.as_ref().unwrap() {
                        if line.contains(".py:") {
                            test.error_type = Some(line.split(" ").last().unwrap().to_string());
                        } else {
                            let existing_error = if test.full_error.clone().is_some() { test.full_error.clone().unwrap()} else {String::new()};

                            test.full_error = Some(existing_error + "\n " + line);

                            test.short_error_description = Some(line.to_string());
                        }
                    }
                }
            }
        }
    }

    // println!("Pytest output: {}", line);
}
