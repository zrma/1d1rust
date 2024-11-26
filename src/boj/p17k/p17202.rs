use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17202(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let t = read_line(reader);

    let arr_s = s.as_bytes();
    let arr_t = t.as_bytes();

    let mut ans = String::new();
    for i in 0..8 {
        ans.push(arr_s[i] as char);
        ans.push(arr_t[i] as char);
    }

    while ans.len() > 2 {
        let arr_ans = ans.as_bytes();
        let mut tmp = String::new();
        for i in 0..ans.len() - 1 {
            let a = arr_ans[i];
            let b = arr_ans[i + 1];
            tmp.push_str(&((a + b - 2 * b'0') % 10).to_string());
        }
        ans = tmp;
    }

    if ans.len() == 1 {
        ans.insert(0, '0');
    }

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/17202
// 핸드폰 번호 궁합
#[test]
fn test_solve17202() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
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
