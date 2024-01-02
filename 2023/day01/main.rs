use std::fs;

fn main() {
    println!("{}", literal_check());
}

fn literal_check() -> u32 {
    let mut result: u32 = 0;
    let contents = fs::read_to_string("calibration")
        .expect("Unable to read file.");

    let lines = contents.lines();

    for _line in lines {
        let characters: Vec<_> = _line.split(&String::new()).collect::<Vec<_>>();
        let size: usize = characters.len();
        let r: &usize = &size;

        let mut l_num: u32 = 0;
        let mut r_num: u32 = 0;

        for i in 0..size {
            l_num = characters[i].parse::<u32>().unwrap_or(0);
            r_num = characters[r-i-1].parse::<u32>().unwrap_or(0);

            println!("{}:{}", characters[i], characters[r-i-1])
        }

        result += l_num + r_num
    }

    return result;
}
