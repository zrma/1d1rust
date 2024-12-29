use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18119(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    // 단어들을 비트마스크로 변환하여 저장
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let word = read_line(reader);
        let mut mask = 0u32;
        for c in word.chars() {
            mask |= 1 << (c as u32 - 'a' as u32);
        }
        words.push(mask);
    }

    // 현재 알고 있는 알파벳들의 비트마스크 (처음에는 모든 알파벳을 알고 있음)
    let mut known = (1 << 26) - 1;

    // 쿼리 처리
    for _ in 0..m {
        let (o, x) = read_values_as!(read_line(reader), usize, char);
        let bit = 1 << (x as u32 - 'a' as u32);

        // 알파벳을 잊거나 기억
        if o == 1 {
            known &= !bit; // 해당 알파벳을 잊음
        } else {
            known |= bit; // 해당 알파벳을 기억
        }

        // 현재 기억할 수 있는 단어의 개수 계산
        // 단어의 모든 알파벳이 known에 포함되어 있어야 함
        let count = words.iter().filter(|&&word| (word & known) == word).count();

        writeln!(writer, "{}", count).unwrap();
    }
}

// https://www.acmicpc.net/problem/18119
// 단어 암기
#[test]
fn test_solve18119() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 10
apple
actual
banana
brick
courts
1 l
1 b
1 c
1 n
2 l
2 b
1 s
2 c
2 s
2 n"
            .to_string(),
            want: "3
1
0
0
1
1
1
3
4
5"
            .to_string(),
        },
        TestData {
            s: "1 3
apple
1 a
1 d
2 a"
            .to_string(),
            want: "0
0
1"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18119(&mut reader, &mut writer);

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
