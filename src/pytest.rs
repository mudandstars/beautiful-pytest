use ansi_term::Color::{Red, Black};
use ansi_term::Style;

const FAILED_TEST_HEAD_LINE_SEPARATOR: &str = "________________________________";
const FAILED_TEST_SECTION_LINE_SEPARATOR: &str = "===================================";

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
    name: String,
    pub error_type: Option<String>,
    pub short_error_description: Option<String>,
    pub full_error: Option<String>,
}

impl Test {
    pub fn new(name: String) -> Test {
        Test {
            name,
            error_type: None,
            short_error_description: None,
            full_error: None,
        }
    }

    pub fn print_error(&self, file_name: String) {
        if self.error_type.is_some() && self.short_error_description.is_some() && self.full_error.is_some() {
            println!("");
            println!("{}\t{}::{}\t\t\t\t{}", Style::new().on(Red).fg(Black).bold().paint(" FAILED "), file_name, self.name, Style::new().on(Red).fg(Black).bold().paint(String::from(" ") + self.error_type.as_ref().unwrap() + " "));
            println!(" {}", Red.paint(self.short_error_description.as_ref().unwrap()));
            println!("");
            println!(" {}", self.full_error.as_ref().unwrap());
            println!("");
        }
    }
}

pub fn line_contains_test(line: &str) -> bool {
    return line.contains(".py::") && (line.contains(" PASSED") || line.contains(" FAILED"));
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
    return line[start_idx..end].to_string();
}

pub fn extract_file_name(line: &str) -> String {
    let end = line.rfind(".py").unwrap();

    return line[0..end].to_string() + ".py";
}
