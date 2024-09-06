use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25024(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans: String = (0..num_cases)
        .map(|_| {
            let (a, b): (u32, u32) = read_values_as!(read_line(reader), u32, u32);

            let is_valid_time = (0..24).contains(&a) && (0..60).contains(&b);
            let is_valid_date = match a {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => b > 0 && b < 32,
                4 | 6 | 9 | 11 => b > 0 && b < 31,
                2 => b > 0 && b <= 29,
                _ => false,
            };

            let time_check = if is_valid_time { "Yes" } else { "No" };
            let date_check = if is_valid_date { "Yes" } else { "No" };

            format!("{} {}", time_check, date_check)
        })
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25024
// 시간과 날짜
#[test]
fn test_solve25024() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "3
8 14
23 59
98 76"
                .to_string(),
            want: "Yes Yes
Yes No
No No"
                .to_string(),
        },
        TestData {
            s: "5
12 31
23 59
0 0
0 1
1 0"
            .to_string(),
            want: "Yes Yes
Yes No
Yes No
Yes No
Yes No"
                .to_string(),
        },
        TestData {
            s: "2
1 29
2 29"
                .to_string(),
            want: "Yes Yes
Yes Yes"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25024(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
