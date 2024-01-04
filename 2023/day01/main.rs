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

        let mut l_str: String = String::new();
        let mut r_str: String = String::new();
        let mut l_num: u32 = 0;
        let mut r_num: u32 = 0;
        
        for l in 0..size {
            l_str = characters[l].to_string();
            
            if _numbers.contains(&l_str) {
                println!("Found left number {}", &l_str);
                l_num = characters[l].parse::<u32>().unwrap();
                break;
            }
        }

        for r in size..1 {
            r_str = characters[r-1].to_string();
            
            if _numbers.contains(&r_str) {
                println!("Found right number {}", &r_str);
                r_num = characters[r-1].parse::<u32>().unwrap();
                break;
            }
        }

        println!("Found L: {}, R:{} in {}", &l_str, &r_str, &_line);
        
        result += l_num + r_num;
    }

    return result;
}
