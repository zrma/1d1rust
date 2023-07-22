use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1235(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let mut a = vec![];
    for _ in 0..n {
        a.push(read_line(reader));
    }

    let mut ans = 1;
    for i in 1..=a[0].len() {
        let mut set = std::collections::HashSet::new();
        for s in &a {
            set.insert(&s[s.len() - i..]);
        }
        if set.len() == n {
            break;
        }
        ans += 1;
    }
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/1235
// 학생 번호
#[test]
fn test_solve1235() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "3
1212345
1212356
0033445"
            .to_string(),
        want: "3".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1235(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}