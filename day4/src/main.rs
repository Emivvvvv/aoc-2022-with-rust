use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();

    let all_pairs = input.lines().collect::<Vec<_>>();
    let mut fully_contain_count = 0;
    let mut partly_contain_count = 0;

    for pairs in &all_pairs {
        let pair = pairs.split(&[',', '-']).collect::<Vec<&str>>();

        let values = pair.iter().map(|x|u32::from_str_radix(x, 10).unwrap()).collect::<Vec<u32>>();

        if ( values[0] <= values[2] && values[1] >= values[3] ) || ( values[0] >= values[2] && values[1] <= values[3] ) {
            fully_contain_count += 1;
        }

        if values[1] >= values[2] && values[3] >= values[0] {
            partly_contain_count += 1;
        }

    }

    println!("Fully contain count: {}", fully_contain_count);
    println!("Partly contain count: {}", partly_contain_count);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}