use ansi_term::Color::{Green, Red, Black};
use crate::pytest::File;


pub fn print_testing_results(files: Vec<File>) {
    let mut passed_tests_count: u32 = 0;
    let mut failed_tests_count: u32 = 0;

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

    if failed_tests_count > 0 {
        println!(" {}\t {}{} {}",
            Black.dimmed().paint("Tests"),
            Red.bold().paint(String::from(" ") + &failed_tests_count.to_string() + " failed"),
            Black.dimmed().paint(","),
            Green.bold().paint(passed_tests_count.to_string() + " passed")
        );
    } else {
        println!(" {}\t {}",
            Black.dimmed().paint("Tests"),
            Green.bold().paint(passed_tests_count.to_string() + " passed")
        );
    }
}
