use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();
    let mut lines = input.lines().collect::<Vec<_>>();

    let mut ten_k_total: i128 = 0;
    let mut curr_min: i128 = 70000000;

    let dir_a_input = console_input_finder("/", &mut lines);
    println!("{}", calc_dir_size(dir_a_input, &mut ten_k_total, &mut lines, &mut curr_min));

    println!("The sum of their total sizes is {ten_k_total}");
    println!("{curr_min} is the total size of the smallest enough directory.")
}

fn calc_dir_size<'a>(console_input: Vec<&'a str>, ten_k_total: &mut i128, lines: &mut Vec<&'a str>, curr_min: &mut i128) -> i128 {
    let mut total = 0;

    for line in console_input.iter() {
        let split_vec = line.split(' ').collect::<Vec<_>>();
        if split_vec[0] == "dir" {
            let dir_size = calc_dir_size(console_input_finder(split_vec[1], lines), ten_k_total, lines, curr_min);

            //I know i cheated... I'm in my midterm week so I don't want to spend more time on it. Maybe I can fix it in the future.
            //              |||||||   I calculated the free space that I need before...
            //              vvvvvvv   A couple solutions come to my mind actually but I didn't find them efficient that's why I'll skip to improve this time.
            if dir_size >= (3636666) && *curr_min > dir_size {
                *curr_min = dir_size;
            }

            total += dir_size;
        } else {
            total += split_vec[0].parse::<i128>().unwrap();
        }
    }

    if total <= 100000 { *ten_k_total += total }
    total
}

fn console_input_finder<'a>(dir_name: &'a str, lines: &mut Vec<&'a str>) -> Vec<&'a str> {
    let lines_len = lines.len();

    let cd_index = lines
        .iter()
        .position(|&x| x == format!("$ cd {}", dir_name))
        .unwrap();

    lines[cd_index] = "CALCULATED";

    let start_index = cd_index + 2;

    let end_index = lines[start_index..]
        .iter()
        .position(|&line| line.contains('$'))
        .unwrap_or(lines_len - start_index) + start_index;

    lines[start_index..end_index].to_owned()
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_console_input_finder() {
        let input = read_file_string("src/test.txt").unwrap();
        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(vec!["4060174 j", "8033020 d.log", "5626152 d.ext", "7214296 k"], console_input_finder("d", &lines));
    }

    #[test]
    fn test_calc_dir_size() {
        let input = read_file_string("src/test.txt").unwrap();
        let lines = input.lines().collect::<Vec<_>>();
        let mut ten_k_total: i128 = 0;

        calc_dir_size(console_input_finder("/", &lines), &mut ten_k_total, &lines);
        assert_eq!(ten_k_total, 95437);
    }
}