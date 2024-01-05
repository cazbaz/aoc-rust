use std::fs;

fn main() {
    println!("{}", literal_check());
}

fn literal_check() -> u32 {
    // Create a vector of number strings from 0-9 to compare to (the literal part)
    let num_strings: Vec<String> = (0..=9).map(|n| n.to_string()).collect::<Vec<String>>();
    let mut result: u32 = 0;

    let contents = fs::read_to_string("calibration")
        .expect("Unable to read file.");

    let lines = contents.lines();

    for line in lines {
        // We don't care about empty lines
        if line.is_empty() {
            break;
        }

        // Get the individual characters of the line
        let line_chars: Vec<_> = line.split(&String::new()).collect::<Vec<_>>();
        let size: usize = line_chars.len();

        let mut l_str: String = String::new();
        let mut r_str: String = String::new();

        for l in 0..size {
            l_str = line_chars[l].to_string();

            if num_strings.contains(&l_str) {
                break;
            }
        }

        for r in (0..size).rev() {
            r_str = line_chars[r].to_string();
            
            if num_strings.contains(&r_str) {
                break;
            }
        }

        // Format the L and R result into LR, convert to u32, and add it into the total.
        // We already know the value will be 0-9, so no error handling is necessary.
        result += format!("{}{}", l_str, r_str).to_string().parse::<u32>().unwrap();

        println!("Found L: {}, R:{}, T:{} in {}", l_str, r_str, result, line);
    }

    return result;
}
