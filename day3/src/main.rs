use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();

    let all_rucksacks = input.lines().collect::<Vec<_>>();
    let mut sum: i64 = 0;

    // Part one
    for rucksack in &all_rucksacks {
        let compartment_len = rucksack.len()/2;
        let (first, second) = rucksack.split_at(compartment_len);

        let mut max: u8 = 0;
        for i in 0..first.len() as usize {
            for j in 0..second.len() as usize {
                let selected =
                    if first.chars().nth(i).unwrap() as u8 >= 97 { first.chars().nth(i).unwrap() as u8 - 96 }
                    else { first.chars().nth(i).unwrap() as u8 - 38 };
                if first.chars().nth(i).unwrap() == second.chars().nth(j).unwrap() && selected > max {
                    max = selected;
                }
            }
        }
        sum += max as i64;
    }
    println!("Part 1 sum: {}", sum);

    // Part two
    sum = 0;
    for i in 0..all_rucksacks.len()/3 as usize {
        let first = all_rucksacks[0 + 3*i];
        let second = all_rucksacks[1 + 3*i];
        let third = all_rucksacks[2 + 3*i];

        let mut same_chars = vec![];
        for i in 0..first.len() as usize {
            for j in 0..second.len() as usize {
                if first.chars().nth(i).unwrap() == second.chars().nth(j).unwrap(){
                    same_chars.push(first.chars().nth(i).unwrap());
                }
            }
        }

        let mut max = 0;
        for k in same_chars {
            let selected =
                if k as u8 >= 97 { k as u8 - 96 }
                else { k as u8 - 38 };
            if third.contains(k) && selected > max {
                max = selected;
            }
        }
        sum += max as i64;
    }
    println!("Part 2 sum: {}", sum)
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}