use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15312(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s1 = read_line(reader);
    let s2 = read_line(reader);

    let scores = [
        3, 2, 1, 2, 3, 3, 2, 3, 3, 2, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 2, 2, 1,
    ];

    let arr1 = s1.as_bytes();
    let arr2 = s2.as_bytes();

    let mut ans = vec![];
    for i in 0..s1.len() {
        ans.push(scores[(arr1[i] - b'A') as usize]);
        ans.push(scores[(arr2[i] - b'A') as usize]);
    }

    while ans.len() > 2 {
        let mut tmp = vec![];
        for i in 0..ans.len() - 1 {
            tmp.push((ans[i] + ans[i + 1]) % 10);
        }
        ans = tmp;
    }

    write!(writer, "{}{}", ans[0], ans[1]).expect("Failed to write");
}

// https://www.acmicpc.net/problem/15312
// 이름 궁합
#[test]
fn test_solve15312() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "CJM
HER"
        .to_string(),
        want: "99".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15312(&mut reader, &mut writer);

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
