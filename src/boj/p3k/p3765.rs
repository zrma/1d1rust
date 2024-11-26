use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3765(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut s = String::new();

    while reader.read_line(&mut s).unwrap_or(0) > 0 {
        write!(writer, "{}", s).unwrap();
        s.clear();
    }
}

// https://www.acmicpc.net/problem/3765
// noinspection SpellCheckingInspection
// Celebrity jeopardy
#[test]
fn test_solve3765() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "Y = 3
X=9"
            .to_string(),
            want: "Y = 3
X=9"
            .to_string(),
        },
        TestData {
            s: "Y = 3
X=9
Y=3"
            .to_string(),
            want: "Y = 3
X=9
Y=3"
            .to_string(),
        },
        TestData {
            s: "Y = 3".to_string(),
            want: "Y = 3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3765(&mut reader, &mut writer);

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
