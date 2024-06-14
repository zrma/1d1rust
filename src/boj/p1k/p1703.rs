use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1703(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = vec![];
    let mut input = String::new();

    while reader.read_line(&mut input).expect("Failed to read") != 0 {
        let trimmed_input = input.trim();
        if trimmed_input == "0" {
            break;
        }

        let mut tokens = trimmed_input.split_whitespace();
        let number_of_pairs = tokens
            .next()
            .expect("Failed to get the number of pairs")
            .parse::<usize>()
            .expect("Failed to parse number of pairs");

        let calculation_result = tokens
            .take(number_of_pairs * 2)
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|pair| {
                let a = pair[0].parse::<i32>().expect("Failed to parse 'a'");
                let b = pair[1].parse::<i32>().expect("Failed to parse 'b'");
                (a, b)
            })
            .fold(1, |acc, (a, b)| acc * a - b);

        answers.push(calculation_result.to_string());
        input.clear();
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/1703
// noinspection SpellCheckingInspection
// 생장점
#[test]
fn test_solve1703() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 3 0
2 3 0 2 0
3 3 0 2 0 2 0
3 3 0 2 1 2 3
2 4 1 3 4
4 5 0 5 1 5 2 5 101
0"
            .to_string(),
            want: "3
6
12
7
5
489"
            .to_string(),
        },
        TestData {
            s: "2 1 0 1 0
0"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1703(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
