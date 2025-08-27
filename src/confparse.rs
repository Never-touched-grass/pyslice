pub struct TokDef {
    pub name: String,
    pub pattern: String,
    pub ignore: bool,
}

pub fn parse(path: &str) -> Result<Vec<TokDef>, std::io::Error> {
    let contents = std::fs::read_to_string(path)?;
    let tokens = contents
        .lines()
        .filter(|line| {
            let t = line.trim();
            !t.starts_with(';') && !t.is_empty()
        })
        .map(|line| {
            let prs: Vec<&str> = line.split_whitespace().collect();
            let name = prs[0];
            let mut regex_parts = prs[1..].to_vec();
            let mut ignore = false;
            if let Some(&"IGNORE") = regex_parts.last() {
                ignore = true;
                regex_parts.pop();
            }
            let pattern = regex_parts.join(" ");
            TokDef {
                name: name.to_string(),
                pattern,
                ignore,
            }
        })
        .collect();
    Ok(tokens)
}
