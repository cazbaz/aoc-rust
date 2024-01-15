/*
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
*/

use std::fs;

fn main() {
    println!("{}", parse_games())
}

struct Game {
    id: u32,
    red: u16,
    green: u16,
    blue: u16
}

fn parse_games() -> u32 {
    let mut result: u32 = 0;

    let input = fs::read_to_string("input").expect("Unable to read input file.");

    for line in input.lines() {
        let game = parse_into_game(line.to_string());
        
        match game {
            Some(game) => {
                result += game.id;
            }
            None => {
                continue;
            }
        }
    }

    return result; 
}

fn parse_into_game(line: String) -> Option<Game> {
    if line.is_empty() {
        return None;
    }

    let name_result_split: Vec<&str> = line.split(":").collect();
    let game_name: Vec<&str> = name_result_split[0].split(" ").collect(); 
    let game_id: String = String::from(game_name[1]);
    let all_results: Vec<&str> = name_result_split[1].split(";").collect();

    print!("name_result_split: {:?}\ngame_name: {:?}\nall_results: {:?}\n", name_result_split, game_name, all_results);

    let mut red_result: &str = "";
    let mut green_result: &str = "";
    let mut blue_result: &str = "";

    for result in all_results {
        if result.contains("red") {
            red_result = result.split(" ").collect::<Vec<&str>>()[0];
            continue;
        }

        if result.contains("green") {
            green_result = result.split(" ").collect::<Vec<&str>>()[0];
            continue;
        }

        if result.contains("blue") {
            blue_result = result.split(" ").collect::<Vec<&str>>()[0];
            continue;
        }
    }

    //print!("r_result: {}, g_result: {}, b_result: {}\n", red_result, green_result, blue_result);

    let game = Game {
        id: game_id.parse::<u32>().unwrap_or(0),
        red: red_result.parse::<u16>().unwrap_or(0),
        green: green_result.parse::<u16>().unwrap_or(0),
        blue: blue_result.parse::<u16>().unwrap_or(0)
    };

    //print!("id: {}, red: {}, green: {}, blue: {}\n", game.id, game.red, game.green, game.blue);
    Some(game)
}
