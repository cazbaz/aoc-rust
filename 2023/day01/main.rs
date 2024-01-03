use std::fs;

fn main() {
    println!("{}", literal_check());
}

fn literal_check() -> u32 {
    let _numbers: Vec<String> = (0..9).map(|x| x.to_string()).collect::<Vec<String>>();
    let mut result: u32 = 0;
    let contents = fs::read_to_string("calibration")
        .expect("Unable to read file.");

    let lines = contents.lines();

    for _line in lines {
        let characters: Vec<_> = _line.split(&String::new()).collect::<Vec<_>>();
        let size: usize = characters.len();
        let r: &usize = &size;

        let mut l_str: String = String::new();
        let mut r_str: String = String::new();
        let mut l_num: u32 = 0;
        let mut r_num: u32 = 0;

        for l in 0..size {
            //l_num = characters[l].parse::<u32>().unwrap_or(0);
            l_str = characters[l].to_string();
            r_str = characters[r-l-1].to_string();
            
            if _numbers.contains(&l_str) {
                l_num = characters[l].parse::<u32>().unwrap();
            }

            if _numbers.contains(&r_str) {
                r_num = characters[r-l-1].parse::<u32>().unwrap();
            }

            if l_num > 0 && r_num > 0 {
                result += l_num + r_num;
                break;
            }
        }
    }

    return result;
}
