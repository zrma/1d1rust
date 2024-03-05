use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2204(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let n = read_line(reader).parse::<usize>().unwrap();
        if n == 0 {
            break;
        }

        let mut words = vec![];
        for _ in 0..n {
            let word = read_line(reader);
            words.push(word);
        }

        words.sort_by_key(|a| a.to_lowercase());

        writeln!(writer, "{}", &words[0]).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/2204
// 도비의 난독증 테스트
#[test]
fn test_solve2204() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "3
Cat
fat
bAt
4
call
ball
All
Hall
0"
        .to_string(),
        want: "bAt
All
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2204(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
