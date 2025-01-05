use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13311(_: &mut impl BufRead, writer: &mut impl Write) {
    writeln!(writer, "-1").unwrap();
}

// https://www.acmicpc.net/problem/13311
// 행운의 편지
#[test]
fn test_solve13311() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [TestData {
        s: "".to_string(),
        want: "-1".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13311(&mut reader, &mut writer);

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
