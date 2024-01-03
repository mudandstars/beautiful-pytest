use ansi_term::Color::{Green, Red, Black};
use crate::pytest::File;


pub fn print_testing_results(files: Vec<File>) {
    let mut passed_tests_count: u32 = 0;
    let mut failed_tests_count: u32 = 0;

    for file in files.iter() {
        for test in file.tests.iter() {
            if test.failed {
                failed_tests_count +=1;
                test.print_error(String::from(&file.name));
            } else {
                passed_tests_count +=1;
            }
        }
    }

    if failed_tests_count > 0 {
        println!("\n {}\t {}{} {}",
            Black.dimmed().paint("Tests"),
            Red.bold().paint(String::from(" ") + &failed_tests_count.to_string() + " failed"),
            Black.dimmed().paint(","),
            Green.bold().paint(passed_tests_count.to_string() + " passed")
        );
    } else {
        println!("\n {}\t {}",
            Black.dimmed().paint("Tests"),
            Green.bold().paint(passed_tests_count.to_string() + " passed")
        );
    }
}
