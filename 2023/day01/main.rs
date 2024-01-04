use std::fs;

fn main() {
    println!("{}", literal_check());
}

fn literal_check() -> u32 {
    // Create a vector of number strings from 0-9 to compare to (the literal part)
    let _numbers: Vec<String> = (0..10).map(|x| x.to_string()).collect::<Vec<String>>();
    let mut result: u32 = 0;

    let contents = fs::read_to_string("calibration")
        .expect("Unable to read file.");

    let lines = contents.lines();

    for _line in lines {
        // We don't care about empty lines
        if _line.is_empty() {
            break;
        }

        // Get the individual characters of the line
        let characters: Vec<_> = _line.split(&String::new()).collect::<Vec<_>>();
        let size: usize = characters.len();

        let mut l_str: String = String::new();
        let mut r_str: String = String::new();
        
        for l in 0..size {
            l_str = characters[l].to_string();
            
            if _numbers.contains(&l_str) {
                break;
            }
        }

        for r in (1..size).rev() {
            r_str = characters[r-1].to_string();
            
            if _numbers.contains(&r_str) {
                break;
            }
        }

        // Format the L and R result into LR, convert to u32, and add it into the total.
        // We already know the value will be 0-9, so no error handling is necessary.
        result += format!("{}{}", &l_str, &r_str).to_string().parse::<u32>().unwrap();

        println!("Found L: {}, R:{}, T:{} in {}", &l_str, &r_str, &result, &_line);
    }

    return result;
}
