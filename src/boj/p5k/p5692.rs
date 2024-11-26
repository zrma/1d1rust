use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5692(reader: &mut impl BufRead, writer: &mut impl Write) {
    let factorials: Vec<u32> = (1..=5)
        .scan(1, |state, i| {
            *state *= i;
            Some(*state)
        })
        .collect();

    let answers: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let n: u32 = match line.parse() {
                Ok(value) => value,
                Err(_) => return None,
            };

            if n == 0 {
                return None;
            }

            let ans = line.chars().rev().enumerate().fold(0, |acc, (pos, c)| {
                acc + c.to_digit(10).expect("should be a digit") * factorials[pos]
            });

            Some(ans.to_string())
        })
        .collect();

    write!(writer, "{}", answers.join("\n")).expect("write! should work");
}

// https://www.acmicpc.net/problem/5692
// 팩토리얼 진법
#[test]
fn test_solve5692() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "719
1
15
110
102
0"
            .to_string(),
            want: "53
1
7
8
8"
            .to_string(),
        },
        TestData {
            s: "11111
0"
            .to_string(),
            want: "153".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5692(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
