use sky::readfile;

type Number = u32;

pub fn a() {
    let ctxt = readfile("01");
    let mut sum = 0;
    for line in ctxt.lines() {
        let digits: Vec<&str> = line.matches(char::is_numeric).collect();
        let Some(first) = digits
            .first()
            .and_then(|s| s.chars().next())
            .and_then(|c| c.to_digit(10))
        else {
            panic!("No digits in {line}");
        };
        let Some(last) = digits
            .last()
            .and_then(|s| s.chars().next())
            .and_then(|c| c.to_digit(10))
        else {
            panic!("No digits in {line} even though there were last time?");
        };
        let value = first * 10 + last;
        sum += value;
    }
    println!("Sum of calibration values is {sum}");
}

fn elf_first(s: &str) -> Number {
    let mut left = s;
    loop {
        if left.is_empty() {
            panic!("No digits found in {s}");
        }
        if left.starts_with(char::is_numeric) {
            let Some(answer) = left.chars().next() else {
                panic!(
                    "Despite starting with a digit, '{left}' does not in fact start with anything!"
                );
            };
            let Some(answer) = answer.to_digit(10) else {
                panic!("Despite being a digit, {answer} is not an ASCII digit, probably Unicode trouble");
            };
            return answer;
        } else if left.starts_with("one") {
            return 1;
        } else if left.starts_with("two") {
            return 2;
        } else if left.starts_with("three") {
            return 3;
        } else if left.starts_with("four") {
            return 4;
        } else if left.starts_with("five") {
            return 5;
        } else if left.starts_with("six") {
            return 6;
        } else if left.starts_with("seven") {
            return 7;
        } else if left.starts_with("eight") {
            return 8;
        } else if left.starts_with("nine") {
            return 9;
        } else {
            // at least one byte, but a whole code point - can use ceil_char_boundary if that's
            // stabilized
            let boundary = 1;
            left = &left[boundary..];
        }
    }
}

fn elf_last(s: &str) -> Number {
    let mut right = s;
    loop {
        if right.is_empty() {
            panic!("No digits found in {s}");
        }
        if right.ends_with(char::is_numeric) {
            let Some(answer) = right.chars().last() else {
                panic!(
                    "Despite ending with a digit, '{right}' does not in fact end with anything!"
                );
            };
            let Some(answer) = answer.to_digit(10) else {
                panic!("Despite being a digit, {answer} is not an ASCII digit, probably Unicode trouble");
            };
            return answer;
        } else if right.ends_with("one") {
            return 1;
        } else if right.ends_with("two") {
            return 2;
        } else if right.ends_with("three") {
            return 3;
        } else if right.ends_with("four") {
            return 4;
        } else if right.ends_with("five") {
            return 5;
        } else if right.ends_with("six") {
            return 6;
        } else if right.ends_with("seven") {
            return 7;
        } else if right.ends_with("eight") {
            return 8;
        } else if right.ends_with("nine") {
            return 9;
        } else {
            // at least one byte, but a whole code point - can use floor_char_boundary if that's
            // stabilized
            let boundary = right.len() - 1;
            right = &right[..boundary];
        }
    }
}

fn elf_digits(s: &str) -> Number {
    elf_first(s) * 10 + elf_last(s)
}

pub fn b() {
    let ctxt = readfile("01");
    let mut sum = 0;
    for line in ctxt.lines() {
        let value = elf_digits(line);
        sum += value;
    }
    println!("Actually sum of calibration values is {sum}");
}
