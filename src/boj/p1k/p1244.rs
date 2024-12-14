use crate::{
    read_values_as,
    utils::io::{read_line, read_n_values, read_value},
};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1244(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_switches: usize = read_value(read_line(reader));
    let mut switches: Vec<usize> = read_n_values(reader, num_switches);
    let num_students: usize = read_value(read_line(reader));

    for _ in 0..num_students {
        let (gender, switch_num) = read_values_as!(read_line(reader), usize, usize);
        match gender {
            1 => {
                // 남학생: 받은 수의 배수 인덱스를 토글
                for i in (switch_num - 1..num_switches).step_by(switch_num) {
                    switches[i] = 1 - switches[i];
                }
            }
            2 => {
                // 여학생: 중심 인덱스에서 좌우 대칭 구간 찾기
                let idx = switch_num - 1;
                let mut left = idx;
                let mut right = idx;

                // 좌우로 확장하며 대칭 상태 확인
                while left > 0
                    && right < num_switches - 1
                    && switches[left - 1] == switches[right + 1]
                {
                    left -= 1;
                    right += 1;
                }

                // 확정된 대칭 구간 토글
                for switch in switches.iter_mut().skip(left).take(right - left + 1) {
                    *switch = 1 - *switch;
                }
            }
            _ => unreachable!(),
        }
    }

    // 20개씩 출력
    for (i, s) in switches.iter().enumerate() {
        write!(
            writer,
            "{}{}",
            s,
            if (i + 1) % 20 == 0 { "\n" } else { " " }
        )
        .unwrap();
    }
    if num_switches % 20 != 0 {
        writeln!(writer).unwrap();
    }
}

// https://www.acmicpc.net/problem/1244
// 스위치 켜고 끄기
#[test]
fn test_solve1244() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8
0 1 0 1 0 0 0 1
2
1 3
2 3"
            .to_string(),
            want: "1 0 0 0 1 1 0 1".to_string(),
        },
        TestData {
            s: "4
0 0 0 0
4
1 1
2 1
1 2
2 2"
            .to_string(),
            want: "0 1 1 0".to_string(),
        },
        TestData {
            s: "8
0 0 0 0 0 0 0 0
8
1 2
1 3
1 4
1 5
2 2
2 3
2 4
2 5"
            .to_string(),
            want: "0 1 1 1 0 1 1 0".to_string(),
        },
        TestData {
            s: "1
1
2
1 1
2 1"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "5
1 0 1 0 1
1
2 5"
            .to_string(),
            want: "1 0 1 0 0".to_string(),
        },
        TestData {
            s: "8
0 1 0 1 0 0 0 1
5
1 3
2 3
2 5
1 6
1 7"
            .to_string(),
            want: "1 0 0 0 0 0 1 1".to_string(),
        },
        TestData {
            s: "5
1 1 0 1 1
1
2 1"
            .to_string(),
            want: "0 1 0 1 1".to_string(),
        },
        TestData {
            s: "3
1 0 1
1
2 1"
            .to_string(),
            want: "0 0 1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1244(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("valid utf8 string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
