use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5217(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let mut answers = Vec::with_capacity(num_cases);
    for _ in 0..num_cases {
        let n: i32 = read_value(read_line(reader));
        let pairs = (1..n / 2 + 1)
            .filter_map(|i| {
                let j = n - i;
                if i < j {
                    Some(format!("{} {}", i, j))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        answers.push(format!(
            "Pairs for {}:{}{}",
            n,
            if pairs.is_empty() { "" } else { " " },
            pairs.join(", ")
        ));
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/5217
// 쌍의 합
#[test]
fn test_solve5217() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
2
3
4
5"
            .to_string(),
            want: "Pairs for 2:
Pairs for 3: 1 2
Pairs for 4: 1 3
Pairs for 5: 1 4, 2 3"
                .to_string(),
        },
        TestData {
            s: "3
1
6
10"
            .to_string(),
            want: "Pairs for 1:
Pairs for 6: 1 5, 2 4
Pairs for 10: 1 9, 2 8, 3 7, 4 6"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve5217(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
