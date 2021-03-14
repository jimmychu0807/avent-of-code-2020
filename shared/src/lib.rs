use std::fs;

pub fn read_file(path: &str) -> Result<Vec<String>, &'static str> {
	let contents = fs::read_to_string(path)
		.map_err(|_| "File cannot be read")?;

	let lines: Vec<String> = contents.split('\n')
		.map(|line| line.to_string())
		.collect();

	Ok(lines)
}
