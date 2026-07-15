use std::io::Write;

pub fn get_number(max: u32) -> u32 {
    loop {
        std::print!(">");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(num) => {
                if num <= max {
                    return num;
                }
            }
            Err(_) => {
            }
        }
    }
}

pub fn get_hex_number(max: u32) -> u32 {
    loop {
        std::print!(">");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input
            .trim()
            .trim_start_matches("0x")
            .trim_start_matches("0X");

        match u32::from_str_radix(input, 16) {
            Ok(num) => {
                if num <= max {
                    return num;
                }
            }
            Err(_) => {
            }
        }
    }
}
