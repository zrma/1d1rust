use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1864(reader: &mut impl BufRead, writer: &mut impl Write) {
    let char_to_num = |c| match c {
        '-' => 0,
        '\\' => 1,
        '(' => 2,
        '@' => 3,
        '?' => 4,
        '>' => 5,
        '&' => 6,
        '%' => 7,
        '/' => -1,
        _ => unreachable!(),
    };

    let mut answers = vec![];
    loop {
        let s = read_line(reader);
        if s == "#" {
            break;
        }

        let ans = s.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            acc + char_to_num(c) * 8_isize.pow(i as u32)
        });
        answers.push(ans.to_string());
    }

    writeln!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/1864
// 문어 숫자
#[test]
fn test_solve1864() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "(@&
?/--
/(\\
?
#"
        .to_string(),
        want: "158
1984
-47
4
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1864(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
