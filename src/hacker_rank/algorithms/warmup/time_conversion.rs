#[allow(dead_code)]
fn time_conversion(mut s: String) -> String {
    let tail = s.split_off(s.len() - 2);
    let offset = if tail.as_str() == "PM" { 12 } else { 0 };

    let token: Vec<&str> = s.split(':').collect();
    let (hour, minute, second) = (token[0], token[1], token[2]);

    let hour_num = hour.parse::<i32>();
    match hour_num {
        Ok(n) => format!("{:02}:{}:{}", n % 12 + offset, minute, second),
        _ => format!("00:{}:{}", minute, second),
    }
}

// https://www.hackerrank.com/challenges/time-conversion/problem
#[test]
fn test_time_conversion() {
    let s = std::string::String::from("07:05:45PM");
    let actual = time_conversion(s);
    let expected = "19:05:45";
    assert_eq!(actual, expected);

    let s = std::string::String::from("AB:12:34AM");
    let actual = time_conversion(s);
    let expected = "00:12:34";
    assert_eq!(actual, expected);
}
