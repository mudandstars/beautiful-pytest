pub const FAILED_TEST_HEAD_LINE_SEPARATOR: &str = "_______________________________";
const FAILED_TEST_HEAD_LINE_SEPARATOR_START: &str = "_______________________________ ";


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
