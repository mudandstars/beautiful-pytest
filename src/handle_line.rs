use ansi_term::Color::Green;


use crate::pytest::{File, Test, line_manipulation};


pub fn handle_line(line: &str, files: &mut Vec<File>, test_name_of_error_being_read: &mut Option<String>, file_name_of_test_being_read: &mut Option<String>) {
    let file_name = line_manipulation::extract_file_name(&line);

    if line.to_lowercase().contains("python") && line.to_lowercase().contains("pytest") {
        println!("{}", Green.dimmed().paint(line));
    }

    if file_name_of_test_being_read.is_some() && *file_name_of_test_being_read.as_ref().unwrap() != file_name {
        files.last_mut().unwrap().print_tests_results();
        *file_name_of_test_being_read = None;
    }

    if line_manipulation::line_contains_test(&line) {
        let test_name = line_manipulation::extract_test_name(&line);

        if files.last().is_none() || files.last().unwrap().name != String::from(&file_name) {
            files.push(File::new(String::from(&file_name)));
            *file_name_of_test_being_read = Some(String::from(&file_name));
        }

        if line_manipulation::test_passed(&line) {
            files.last_mut().unwrap().tests.push(Test::new(String::from(&test_name), false));
        } else {
            files.last_mut().unwrap().tests.push(Test::new(String::from(&test_name), true));
        }
    }

    set_test_name_of_error_being_read(line, test_name_of_error_being_read);

    if test_name_of_error_being_read.is_some() {
        if line.contains("short test summary info") {
            *test_name_of_error_being_read = None;
        } else if ! line.contains(line_manipulation::FAILED_TEST_HEAD_LINE_SEPARATOR) && line.len() > 0 {
            for file in files.iter_mut() {
                for test in file.tests.iter_mut() {
                    if test.name == *test_name_of_error_being_read.as_ref().unwrap() {
                        if line.contains(".py:") {
                            test.error_type = Some(line.split(" ").last().unwrap().to_string());

                            test.error_line_number = Some(line.split(".py:").last().unwrap().split(":").next().unwrap().parse::<u32>().expect("Failed to read error line number"));
                        } else {
                            let existing_error = if test.full_error.clone().is_some() { test.full_error.clone().unwrap()} else {String::new()};

                            test.full_error = Some(existing_error + "\n " + line);

                            test.short_error_description = Some(line.split("E      ").last().unwrap().to_string());
                        }
                    }
                }
            }
        }
    }
}

fn set_test_name_of_error_being_read(line: &str, test_name_of_error_being_read: &mut Option<String>) {
    let test_name_of_the_error = line_manipulation::is_error_header_for_test(line);

    if test_name_of_the_error.is_some() {
        *test_name_of_error_being_read = Some(test_name_of_the_error.unwrap().to_string());
    }
}
