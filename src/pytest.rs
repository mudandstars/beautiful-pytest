use ansi_term::Color::{Red, Green, Black};
use ansi_term::Style;

pub mod line_manipulation;

const CHECK_MARK: char = '\u{2714}';
const X_MARK: char = '\u{2718}';

const FILE_NAME_WIDTH: usize  = 120;
const ERROR_TYPE_WIDTH: usize  = 20;
const TEST_STATE_WIDTH: usize  = 10;

pub struct File {
    pub name: String,
    pub tests: Vec<Test>,
}

impl File {
    pub fn new(name: String) -> Self {
        return File {
            name,
            tests: vec![],
        }
    }

    pub fn print_tests_results(&mut self) {
        println!("");

        if self.all_tests_passed() {
            println!("{} {}", Style::new().on(Green).fg(Black).bold().paint(padded_text("PASSED".to_string(), TEST_STATE_WIDTH)), self.name);
        } else {
            println!("{} {}", Style::new().on(Red).fg(Black).bold().paint(padded_text("FAILED".to_string(), TEST_STATE_WIDTH)), self.name);
        }

        for test in self.tests.iter() {
            test.print_result();
        }
    }

    fn all_tests_passed(&self) -> bool {
        for test in self.tests.iter() {
            if test.failed {
                return false;
            }
        }

        return true;
    }
}

pub struct Test {
    pub name: String,
    pub failed: bool,
    pub error_line_number: Option<u32>,
    pub error_type: Option<String>,
    pub short_error_description: Option<String>,
    pub full_error: Option<String>,
}

impl Test {
    pub fn new(name: String, failed: bool) -> Self {
        Test {
            name,
            failed,
            error_line_number: None,
            error_type: None,
            short_error_description: None,
            full_error: None,
        }
    }

    pub fn print_result(&self) {
        if self.failed {
            println!(" {} {}", Red.paint(X_MARK.to_string()), self.name);
        } else {
            println!(" {} {}", Green.paint(CHECK_MARK.to_string()), self.name);
        }
    }

    pub fn print_error(&self, file_name: String) {
        if self.failed {
            println!("\n");
            println!("{}{}{}", Style::new().on(Red).fg(Black).bold().paint(" FAILED "), padded_text(file_name + &self.name, FILE_NAME_WIDTH), Style::new().on(Red).fg(Black).bold().paint(String::from(" ") + &padded_text(self.error_type.as_ref().unwrap().to_string(), ERROR_TYPE_WIDTH) + " "));
            println!(" {} {}", Red.bold().underline().paint(String::from("Line ") + &self.error_line_number.unwrap().to_string() + ":"), Red.paint(self.short_error_description.as_ref().unwrap()));
            println!(" {}", self.full_error.as_ref().unwrap());
        }
    }
}


fn padded_text(text: String, width: usize) -> String {
    format!("{:^width$}", text, width = width - 2)
}
