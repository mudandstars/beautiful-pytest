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

    return line[0..end].to_string();
}
