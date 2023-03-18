use std::fs;

#[derive(PartialEq)]
enum Choice {
    R,
    P,
    S,
}

fn main() {
    let input =  read_file_string("src/input.txt").unwrap();
    let all_games = input.lines().collect::<Vec<_>>();
    let mut total_score = 0;

    // Part one
    for game in &all_games {
        let hands = game.split(' ').collect::<Vec<_>>();

        let elve = match hands[0] {
            "A" => Choice::R,
            "B" => Choice::P,
            "C" => Choice::S,
            _ => panic!("Something went wrong while trying to match elve"),
        };

        let my_hand = match hands[1] {
            "X" => {total_score += 1; Choice::R},
            "Y" => {total_score += 2; Choice::P},
            "Z" => {total_score += 3; Choice::S},
            _ => panic!("Something went wrong while trying to match my_hand"),
        };

        if elve == my_hand {total_score += 3; continue}

        total_score += match my_hand {
            Choice::R => { if elve == Choice::S { 6 } else { 0 } },
            Choice::P => { if elve == Choice::R { 6 } else { 0 } },
            Choice::S => { if elve == Choice::P { 6 } else { 0 } },
        }
    }
    println!("Total score in part 1 is: {}", total_score);


    // Part two
    total_score = 0;
    for game in &all_games {
        let hands = game.split(' ').collect::<Vec<_>>();

        let elve = match hands[0] {
            "A" => Choice::R,
            "B" => Choice::P,
            "C" => Choice::S,
            _ => panic!("Something went wrong while trying to match elve"),
        };

        total_score += match hands[1] {
            "X" => {if elve == Choice::R { 3 } else if elve == Choice::P { 1 } else { 2 }},
            "Y" => {if elve == Choice::R { 1 + 3 } else if elve == Choice::P { 2 + 3 } else { 3 + 3 }},
            "Z" => {if elve == Choice::R { 2 + 6 } else if elve == Choice::P { 3 + 6} else { 1 + 6}},
            _ => panic!("Something went wrong while trying to match my_hand"),
        };

    }
    println!("Total score in part 2 is: {}", total_score);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}