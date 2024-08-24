use anyhow::Result;

pub fn read_example(path: &str) -> Result<Vec<String>> {
    let text = std::fs::read_to_string(path)?;

    let parts: Vec<&str> = text.split("---").collect();
    assert!(parts.len() == 3);
    Ok(parts.into_iter().map(|x| x.trim().to_string()).collect())
}
