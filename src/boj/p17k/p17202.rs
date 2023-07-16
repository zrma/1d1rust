use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17202(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let t = read_line(reader);

    let mut ans = String::new();
    for i in 0..8 {
        ans.push(s.chars().nth(i).unwrap());
        ans.push(t.chars().nth(i).unwrap());
    }

    while ans.len() > 2 {
        let mut tmp = String::new();
        for i in 0..ans.len() - 1 {
            let a = ans.chars().nth(i).unwrap();
            let b = ans.chars().nth(i + 1).unwrap();
            tmp.push_str(&((a as u8 + b as u8 - 2 * b'0') % 10).to_string());
        }
        ans = tmp;
    }

    if ans.len() == 1 {
        ans.insert(0, '0');
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/17202
// 핸드폰 번호 궁합
#[test]
fn test_solve17202() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "74759336
36195974"
                .to_string(),
            want: "26".to_string(),
        },
        TestData {
            s: "01234567
12345678"
                .to_string(),
            want: "02".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17202(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
