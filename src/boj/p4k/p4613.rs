use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4613(reader: &mut impl BufRead, writer: &mut impl Write) {
    let reader = reader.lines();
    let mut results = vec![];

    for line in reader.map_while(Result::ok) {
        if line == "#" {
            break;
        }

        let sum = line.trim_end().chars().enumerate().fold(0, |acc, (i, c)| {
            if c == ' ' {
                acc
            } else {
                acc + (i as u32 + 1) * (c as u32 - 'A' as u32 + 1)
            }
        });
        results.push(sum.to_string());
    }

    write!(writer, "{}", results.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/4613
// noinspection SpellCheckingInspection
// Quicksum
#[test]
fn test_solve4613() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ACM
MID CENTRAL
REGIONAL PROGRAMMING CONTEST
ACN
A C M
ABC
BBC
#"
            .to_string(),
            want: "46
650
4690
49
75
14
15"
            .to_string(),
        },
        TestData {
            s: "A
AB
ABC
#"
            .to_string(),
            want: "1
5
14"
            .to_string(),
        },
        TestData {
            s: "A
AA
AAA
#"
            .to_string(),
            want: "1
3
6"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4613(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
