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
        writeln!(writer, "{} {}", c, count).unwrap();
    }
}

// https://www.acmicpc.net/problem/2386
// 도비의 영어 공부
#[test]
fn test_2386() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
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
        use std::io::Cursor;
        let mut reader = Cursor::new(&data.s);
        let mut writer = Cursor::new(Vec::new());
        solve2386(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}
