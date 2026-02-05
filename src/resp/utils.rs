pub fn wrong_args(cmd: &str) -> String {
    format!("-ERR wrong number of arguments for '{}'\r\n", cmd)
}

pub fn bulk_string(s: &str) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}

pub fn format_array(values: Vec<String>) -> String {
    let mut resp = format!("*{}\r\n", values.len());
    for v in values {
        resp.push_str(&bulk_string(&v));
    }
    resp
}


pub fn parse_resp(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    // RESP protocol: starts with *N indicating array of N elements
    if lines[0].starts_with('*') {
        let mut result = Vec::new();
        let mut i = 1;
        while i < lines.len() {
            if lines[i].starts_with('$') {
                // Next line is the actual value
                if i + 1 < lines.len() {
                    result.push(lines[i + 1].to_string());
                    i += 2;
                } else {
                    break;
                }
            } else {
                i += 1;
            }
        }
        result
    } else {
        // Inline command: split by whitespace
        input.split_whitespace().map(|s| s.to_string()).collect()
    }
}
