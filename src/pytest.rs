use ansi_term::Color::{Red, Black};
use ansi_term::Style;

pub const FAILED_TEST_HEAD_LINE_SEPARATOR: &str = "_______________________________";
const FAILED_TEST_HEAD_LINE_SEPARATOR_START: &str = "_______________________________ ";
const FAILED_TEST_SECTION_LINE_SEPARATOR: &str = "==================================";

pub struct File {
    name: String,
    pub tests: Vec<Test>
}

impl File {
    pub fn new(name: String) -> File {
        return File {
            name,
            tests: vec![],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct Test {
    pub name: String,
    pub error_line_number: Option<u32>,
    pub error_type: Option<String>,
    pub short_error_description: Option<String>,
    pub full_error: Option<String>,
}

impl Test {
    pub fn new(name: String) -> Test {
        Test {
            name,
            error_line_number: None,
            error_type: None,
            short_error_description: None,
            full_error: None,
        }
    }

    pub fn print_error(&self, file_name: String) {
        if self.error_type.is_some() && self.error_line_number.is_some() && self.short_error_description.is_some() && self.full_error.is_some() {
            println!("");
            println!("{} {}::{}\t\t\t\t{}", Style::new().on(Red).fg(Black).bold().paint(" FAILED "), file_name, self.name, Style::new().on(Red).fg(Black).bold().paint(String::from(" ") + self.error_type.as_ref().unwrap() + " "));
            println!(" {} {}", Red.bold().underline().paint(String::from("Line ") + &self.error_line_number.unwrap().to_string() + ":"), Red.paint(self.short_error_description.as_ref().unwrap()));
            println!(" {}", self.full_error.as_ref().unwrap());
            println!("");
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
    let end = line.rfind(".py").unwrap();

    return line[0..end].to_string() + ".py";
}
