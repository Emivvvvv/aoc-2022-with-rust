use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();

    let creates_input = input.lines().filter(|line| line.contains("[")).collect::<Vec<_>>();
    let instructions = input.lines().filter(|line| line.contains("move")).collect::<Vec<_>>();

    let mut all_stacks: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    let mut part_two = all_stacks.clone();

    for line in creates_input {
        for stack in 0..line.len()/4 + 1 as usize {
            let create = &line[stack*3 + stack..(stack + 1)*3 + stack];
            let vec_len = all_stacks[stack].len();
            let letter = String::from(&create[1..2]);
            if letter != " " {all_stacks[stack].insert(vec_len , letter.clone()); part_two[stack].insert(vec_len, letter)}
        }
    }

    for instruction in &instructions {
        let instruction_vec = instruction.split(' ').filter(|word| !word.contains("o")).map(|num| num.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mv = instruction_vec[0];
        let frm = instruction_vec[1] - 1;
        let to = instruction_vec[2] - 1;

        for i in 0..mv{
            //Part one
            let create = all_stacks[frm].remove(0);
            all_stacks[to].insert(0, create);

            //Part two
            let create = part_two[frm].remove(mv - i - 1);
            part_two[to].insert(0, create);
        }
    }

    print!("Part one: ");
    for stack in &all_stacks {
        if stack.len() != 0 {print!("{}", stack[0])}
        else {print!("#")};
    }
    println!();


    print!("Part two: ");
    for stack in &part_two {
        if stack.len() != 0 {print!("{}", stack[0])}
        else {print!("#")};
    }
    println!();
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
