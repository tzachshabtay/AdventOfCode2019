pub fn run() {
    let (from, to) = (356261, 846303);
    let x = (from..=to).filter(|n| is_valid(n.to_string())).count();
    println!("{}", x);
}

enum SeqStatus {
    Candidate,
    Match,
    Disqualified,
}

fn is_valid(pwd: String) -> bool {
    let mut prev_char: char = ' ';
    let mut found_two_identicals = false;
    let mut seq_status = SeqStatus::Candidate;
    for c in pwd.chars() {
        if prev_char == ' ' {
            prev_char = c;
            seq_status = SeqStatus::Candidate;
            continue;
        }
        if prev_char > c {
            return false;
        }
        if prev_char == c {
            match seq_status {
                SeqStatus::Candidate => seq_status = SeqStatus::Match,
                SeqStatus::Match => seq_status = SeqStatus::Disqualified,
                _ => {}
            }
        } else {
            match seq_status {
                SeqStatus::Match => found_two_identicals = true,
                _ => {}
            }
            seq_status = SeqStatus::Candidate;
        }
        prev_char = c;
    }
    match seq_status {
        SeqStatus::Match => found_two_identicals = true,
        _ => {}
    }
    return found_two_identicals;
}