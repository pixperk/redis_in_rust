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
    input
        .lines()
        .filter(|line| !line.starts_with('*') && !line.starts_with('$'))
        .map(|line| line.to_string())
        .collect()
}
