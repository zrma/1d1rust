use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3048(reader: &mut impl BufRead, writer: &mut impl Write) {
    let _ = read_line(reader);

    struct Ant {
        name: char,
        to_right: bool,
        moved: bool,
    }

    let ants0 = read_line(reader)
        .chars()
        .rev()
        .map(|c| Ant {
            name: c,
            to_right: true,
            moved: false,
        })
        .collect::<Vec<_>>();

    let ants1 = read_line(reader)
        .chars()
        .map(|c| Ant {
            name: c,
            to_right: false,
            moved: false,
        })
        .collect::<Vec<_>>();

    let mut ants = ants0;
    ants.extend(ants1);

    let t = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..t {
        for i in 0..ants.len() - 1 {
            if ants[i].to_right && !ants[i].moved && !ants[i + 1].to_right && !ants[i + 1].moved {
                ants.swap(i, i + 1);
                ants[i].moved = true;
                ants[i + 1].moved = true;
            }
        }

        ants.iter_mut().for_each(|a| a.moved = false);
    }

    for ant in ants {
        write!(writer, "{}", ant.name).unwrap();
    }
}

// https://www.acmicpc.net/problem/3048
// 개미
// noinspection SpellCheckingInspection
#[test]
fn test_solve3048() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 3
ABC
DEF
0"
            .to_string(),
            want: "CBADEF".to_string(),
        },
        TestData {
            s: "3 3
ABC
DEF
2"
            .to_string(),
            want: "CDBEAF".to_string(),
        },
        TestData {
            s: "3 4
JLA
CRUO
3"
            .to_string(),
            want: "CARLUJO".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3048(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
