use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2386(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let line = read_line(reader).to_ascii_lowercase();
        if &line == "#" {
            break;
        }

        let mut parts = line.splitn(2, ' ');
        let c = parts.next().unwrap().chars().next().unwrap();
        let s = parts.next().unwrap_or("");

        let count = s.chars().filter(|&ch| ch == c).count();
        writeln!(writer, "{} {}", c, count).expect("writeln! should work");
    }
}

// https://www.acmicpc.net/problem/2386
// 도비의 영어 공부
#[test]
fn test_solve2386() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "g Programming Contest
n New Zealand
x This is quite a simple problem.
#"
        .to_string(),
        want: "g 2
n 2
x 0
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2386(&mut reader, &mut writer);

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
