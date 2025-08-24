pub struct TokDef {
    pub name: String,
    pub pattern: String,
    pub ignore: bool
}

pub fn parse(path: &str) -> Result<Vec<TokDef>, std::io::Error> {
    let contents = std::fs::read_to_string(path).unwrap();
    let tokens = contents.lines()
        .filter(|line| !line.trim().starts_with(";") && !line.trim().is_empty())
        .map(|line| {
            let prs: Vec<&str> = line.split_whitespace().collect();
            TokDef {
                name: prs[0].to_string(),
                pattern: prs[1].to_string(),
                ignore: prs.get(2).map_or(false, |opt| *opt == "IGNORE"),
            }
        })
        .collect();
    Ok(tokens)
}
