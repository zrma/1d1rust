use crate::utils::functions::char_to_index;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11648(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut number_str = read_line(reader);

    let mut steps = 0;
    while number_str.len() > 1 {
        let product: u64 = number_str.chars().map(char_to_index::<u64>).product();
        number_str = product.to_string();
        steps += 1;
    }

    write!(writer, "{}", steps).expect("write! should work");
}

// https://www.acmicpc.net/problem/11648
// 지속
#[test]
fn test_solve11648() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "10".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "679".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "111111111".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11648(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
