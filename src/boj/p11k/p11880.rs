use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11880(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let mut output = String::new();

    let t = input.trim().parse::<usize>().unwrap();
    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();
        let (a, b, c) = (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        );

        let sum = a + b + c;
        let max = a.max(b).max(c);
        let other = sum - max;

        let ans = max * max + other * other;

        output.push_str(&format!("{}\n", ans));
    }

    write!(writer, "{}", output).unwrap();
}

// https://www.acmicpc.net/problem/11880
// 개미
#[test]
fn test_solve11880() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
1 2 3"
                .to_string(),
            want: "18
"
            .to_string(),
        },
        TestData {
            s: "2
1 1 1
1 2 3"
                .to_string(),
            want: "5
18
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11880(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}
