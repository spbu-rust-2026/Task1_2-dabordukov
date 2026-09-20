use std::io;

fn main() {
    let mut input = String::new();
    let mut sum: u128 = 0;
    loop {
        input.clear();
        if io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line")
            == 0
        {
            break;
        }

        let input = input.trim();

        if input == "-1" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        match input.parse::<u128>() {
            Ok(number) if number > 0 => sum += number,
            _ => {
                println!("NaN");
                return;
            }
        }
    }

    println!("{sum}")
}
