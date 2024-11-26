use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1235(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(read_line(reader));
    }

    let mut ans = 1;
    for i in 1..=a[0].len() {
        let mut set = std::collections::HashSet::with_capacity(n);
        for s in &a {
            set.insert(&s[s.len() - i..]);
        }
        if set.len() == n {
            break;
        }
        ans += 1;
    }
    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/1235
// 학생 번호
#[test]
fn test_solve1235() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
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
