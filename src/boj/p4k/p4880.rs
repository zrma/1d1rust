use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4880(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let (a1, a2, a3) = read_values_as!(read_line(reader), i32, i32, i32);
        if (a1, a2, a3) == (0, 0, 0) {
            break;
        }

        let ans = if a2 - a1 == a3 - a2 {
            format!("AP {}", a3 + (a2 - a1)) // 등차수열
        } else if a1 != 0 && a2 % a1 == 0 && a3 % a2 == 0 {
            format!("GP {}", a3 * (a2 / a1)) // 등비수열
        } else {
            panic!("Invalid sequence: {} {} {}", a1, a2, a3); // 잘못된 입력 시 패닉 발생
        };

        writeln!(writer, "{}", ans).expect("Failed to write result");
    }
}

// https://www.acmicpc.net/problem/4880
// 다음수
#[test]
fn test_solve4880() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 7 10
2 6 18
0 0 0"
                .to_string(),
            want: "AP 13
GP 54
"
            .to_string(),
        },
        TestData {
            s: "1 2 4
1 2 3
0 1 2
-1 0 1
-2 -1 0
0 0 0"
                .to_string(),
            want: "GP 8
AP 4
AP 3
AP 2
AP 1
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4880(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
