use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5565(reader: &mut impl BufRead, writer: &mut impl Write) {
    let total: i32 = read_value(read_line(reader));
    let mut remaining = total;
    for _ in 0..9 {
        remaining -= read_value::<i32>(read_line(reader));
    }

    writeln!(writer, "{}", remaining).unwrap();
}

// https://www.acmicpc.net/problem/5565
// 영수증
#[test]
fn test_solve5565() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9850
1050
800
420
380
600
820
2400
1800
980"
            .to_string(),
            want: "600".to_string(),
        },
        TestData {
            s: "5500
100
200
300
500
600
700
800
900
1000"
                .to_string(),
            want: "400".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5565(&mut reader, &mut writer);

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
