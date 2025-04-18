use crate::utils::io::read_line;
use std::convert::TryInto;
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
        _ => unreachable!("invalid char"),
    };

    let mut answers = vec![];
    loop {
        let s = read_line(reader);
        if s == "#" {
            break;
        }

        let ans: isize = s
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let exponent: u32 = i.try_into().unwrap();
                char_to_num(c) * 8_isize.pow(exponent)
            })
            .sum();

        answers.push(ans.to_string());
    }

    writeln!(writer, "{}", answers.join("\n")).unwrap();
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
