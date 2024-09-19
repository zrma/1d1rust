use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13416(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|_| {
            let num_days: usize = read_value(read_line(reader));
            let profits: i32 = (0..num_days)
                .map(|_| read_n_values(reader, 3))
                .map(|v| v.iter().copied().filter(|&x| x > 0).max().unwrap_or(0))
                .sum();
            profits.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/13416
// 주식 투자
#[test]
fn test_solve13416() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "2
4
500 800 200
300 0 300
-100 -200 -400
600 200 300
3
451 234 309
224 334 467
143 246 245"
                .to_string(),
            want: "1700
1164"
                .to_string(),
        },
        TestData {
            s: "2
3
100 100 100
100 100 100
100 100 100
2
100 50 -1
-50 -30 -20"
                .to_string(),
            want: "300
100"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13416(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
