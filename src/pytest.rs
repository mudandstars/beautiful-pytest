use ansi_term::Color::{Red, Green, Black};
use ansi_term::Style;

pub const FAILED_TEST_HEAD_LINE_SEPARATOR: &str = "_______________________________";
const FAILED_TEST_HEAD_LINE_SEPARATOR_START: &str = "_______________________________ ";
const FAILED_TEST_SECTION_LINE_SEPARATOR: &str = "==================================";
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

pub fn line_contains_test(line: &str) -> bool {
    return line.contains(".py::") && (line.contains(" PASSED") || line.contains(" FAILED"));
}

pub fn is_error_header_for_test(line: &str) -> Option<String> {
    if line.contains(FAILED_TEST_HEAD_LINE_SEPARATOR) {
        let end_index = line.rfind(&(String::from(" ") + FAILED_TEST_HEAD_LINE_SEPARATOR)).unwrap();
        let start = line.find(FAILED_TEST_HEAD_LINE_SEPARATOR_START).unwrap();
        let start_index = start + FAILED_TEST_HEAD_LINE_SEPARATOR_START.len();

        let test_name = &line[start_index..end_index];
        let cleaned = test_name.replace("test_", "").replace("_", " ");

        Some(cleaned)
    } else {
        None
    }
}

pub fn test_passed(line: &str) -> bool {
    return line.contains("PASSED");
}

pub fn extract_test_name(line: &str) -> String {
    let start = line.find(".py::").unwrap();

    let end: usize;
    if line.contains("PASSED") {
        end = line.rfind(" PASSED").unwrap();
    } else {
        end = line.rfind(" FAILED").unwrap();
    }

    let start_idx = start + ".py::".len();
    return line[start_idx..end].to_string().replace("test_", "").replace("_", " ");
}

pub fn extract_file_name(line: &str) -> String {
    let end = line.rfind(".py");

    if end.is_none() {
        return String::new()
    }

    return line[0..end.unwrap()].to_string() + ".py";
}

 fn padded_text(text: String, width: usize) -> String {
    format!("{:^width$}", text, width = width - 2)
}
