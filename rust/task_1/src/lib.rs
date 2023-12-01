use std::collections::HashMap;


fn replace_text_to_digit(line: &str, mapping: HashMap<&str, &str>) -> String {
    let copied = line.clone();
    let mut start = 0;
    let mut end = 2;
    while line.len() > 3 && end < line.len() && start <= line.len() - 3 {
        let part = &copied[start..=end];
        if mapping.contains_key(part) {
            return copied.replace(part, mapping.get(part).unwrap()).to_string()
        }
        if end - start < 4 {
            end += 1;
        } else {
            start += 1;
            end = start + 2;
        }
    }

    copied.to_string()
}

fn code_for_line(line: &str) -> i32 {
    let digits_normal = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let line_normal_replaced = replace_text_to_digit(line, digits_normal);

    let digits_reversed = HashMap::from([
        ("eno", "1"),
        ("owt", "2"),
        ("eerht", "3"),
        ("ruof", "4"),
        ("evif", "5"),
        ("xis", "6"),
        ("neves", "7"),
        ("thgie", "8"),
        ("enin", "9"),
    ]);
    let line_reversed = line.chars().rev().collect::<String>();
    let line_reversed_replaced = replace_text_to_digit(&line_reversed, digits_reversed);

    let first_digit = line_normal_replaced.chars().find(|c| c.is_digit(10)).expect("Could not find digit");
    let last_digit = line_reversed_replaced.chars().find(|c| c.is_digit(10)).expect("Could not find digit");
    format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap()
}

pub fn trebuchet(calibration_text: &str) -> i32 {
    calibration_text.lines().map(|l| code_for_line(l)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_calibration_for_line_when_two_digits_present() {
        let calibration_line = "1abc2";
        let code = code_for_line(calibration_line);
        assert_eq!(code, 12);
    }


    #[test]
    fn gets_calibration_for_line_when_many_present() {
        let calibration_line = "6a31bc7";
        let code = code_for_line(calibration_line);
        assert_eq!(code, 67);
    }


    #[test]
    fn gets_calibration_for_line_when_single_digit_present() {
        let calibration_line = "a6bc";
        let code = code_for_line(calibration_line);
        assert_eq!(code, 66);
    }


    #[test]
    fn gets_calibration_for_line_when_text_as_digits() {
        let calibration_line = "twone";
        let code = code_for_line(calibration_line);
        assert_eq!(code, 21);
    }

    #[test]
    fn gets_calibration_value() {
        let calibration_text = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let calibration = trebuchet(calibration_text);
        assert_eq!(calibration, 142);
    }
}
