pub fn run() {
    let (from, to) = (356261, 846303);
    let x = (from..=to).filter(|n| is_valid(n.to_string())).count();
    println!("{}", x);
}

fn is_valid(pwd: String) -> bool {
    let mut prev_char: char = ' ';
    let mut found_two_identicals = false;
    for c in pwd.chars() {
        if prev_char == ' ' {
            prev_char = c;
            continue;
        }
        if prev_char > c {
            return false;
        }
        if prev_char == c {
            found_two_identicals = true;
        }
        prev_char = c;
    }
    return found_two_identicals;
}