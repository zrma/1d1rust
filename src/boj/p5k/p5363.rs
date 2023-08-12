use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5363(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..n {
        let s = read_line(reader);

        let mut iter = s.split_whitespace();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let mut ans = iter.collect::<Vec<&str>>();
        ans.push(a);
        ans.push(b);

        writeln!(writer, "{}", ans.join(" ")).unwrap();
    }
}

// https://www.acmicpc.net/problem/5363
// 요다
#[test]
fn test_solve5363() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "4
I will go now to find the Wookiee
Solo found the death star near planet Kessel
I'll fight Darth Maul here and now
Vader will find Luke before he can escape"
                .to_string(),
            want: "go now to find the Wookiee I will
the death star near planet Kessel Solo found
Darth Maul here and now I'll fight
find Luke before he can escape Vader will
"
            .to_string(),
        },
        TestData {
            s: "1
I will go now to find the Wookiee"
                .to_string(),
            want: "go now to find the Wookiee I will
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5363(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
