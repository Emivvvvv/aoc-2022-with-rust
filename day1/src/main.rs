use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();

    let mut all_calories_per_elf: Vec<i64> = Vec::new();
    let all_calories = input.lines().collect::<Vec<_>>();
    let mut total = 0;

    for line in all_calories {
        if line == "" {
            all_calories_per_elf.push(total);
            total = 0;
        } else {
            match line.trim().parse::<i64>()  {
                Ok(cal) => total += cal,
                Err(e) => panic!("{}",e),
            };
        }
    }

    all_calories_per_elf.sort_by(|a, b| b.cmp(a));
    println!("max {} cals", all_calories_per_elf[0]);
    println!("sum of biggest three is {} cals", all_calories_per_elf[0] + all_calories_per_elf[1] + all_calories_per_elf[2])
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}