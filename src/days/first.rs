pub fn first(inputs: Vec<&str>) -> () {
    let mut sum: i32 = 0;
    for input in inputs {
        sum += determine_number_1(input);
    }
    println!("{sum}");
}

fn determine_number_1(string: &str) -> i32 {
    let mut numbers: (String, String) = (String::new(), String::new());
    for ch in string.chars() {
        match ch {
            '0'..='9' => {
                if numbers.0 == "" {
                    numbers.0 = ch.to_string();
                } else {
                    numbers.1 = ch.to_string();
                }
            }
            _ => ()
        }    
    }
    if String::is_empty(&numbers.1) {
        format!("{}{}", numbers.0, numbers.0).parse::<i32>().unwrap()
    }else {
        format!("{}{}", numbers.0, numbers.1).parse::<i32>().unwrap()
    }
}

