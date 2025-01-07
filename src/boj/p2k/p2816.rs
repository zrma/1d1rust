use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2816(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut channels = Vec::with_capacity(n);
    for _ in 0..n {
        channels.push(read_line(reader));
    }

    let mut cmds = String::new();
    let mut curr = 0;

    // 1단계: KBS1을 맨 위로 이동
    let i1 = channels.iter().position(|ch| ch == "KBS1").unwrap();
    for _ in 0..i1 {
        cmds.push('1');
        curr += 1;
    }
    for _ in 0..i1 {
        cmds.push('4');
        channels.swap(curr, curr - 1);
        curr -= 1;
    }

    // 2단계: KBS2를 두 번째 위치로 이동
    let i2 = channels.iter().position(|ch| ch == "KBS2").unwrap();
    for _ in 0..i2 {
        cmds.push('1');
        curr += 1;
    }
    for _ in 1..i2 {
        cmds.push('4');
        channels.swap(curr, curr - 1);
        curr -= 1;
    }

    writeln!(writer, "{}", cmds).unwrap();
}

// https://www.acmicpc.net/problem/2816
// 디지털 티비
#[test]
fn test_solve2816() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
MBC
KBS1
KBS2"
                .to_string(),
            want: "14114".to_string(),
        },
        TestData {
            s: "4
ABC1
ABC02
KBS2
KBS1"
                .to_string(),
            want: "11144411144".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2816(&mut reader, &mut writer);

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
