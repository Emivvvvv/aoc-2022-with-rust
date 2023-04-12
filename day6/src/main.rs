use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();

}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}