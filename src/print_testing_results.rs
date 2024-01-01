pub fn print_testing_results(files: Vec<pytest::File>) {
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

    println!(" {}\t {}{} {}",
        Black.dimmed().paint("Tests"),
        Red.bold().paint(failed_tests_count.to_string() + " failed"),
        Black.dimmed().paint(","),
        Green.bold().paint(passed_tests_count.to_string() + " passed")
    );
}
