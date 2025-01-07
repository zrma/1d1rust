use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1406(reader: &mut impl BufRead, writer: &mut impl Write) {
    // 초기 문자열과 명령어 수 읽기
    let s = read_line(reader);
    let n: usize = read_value(read_line(reader));

    // 커서 왼쪽과 오른쪽을 담당할 스택(벡터)
    let mut left: Vec<char> = s.chars().collect();
    let mut right: Vec<char> = Vec::new();

    for _ in 0..n {
        let line = read_line(reader);
        let parts = line.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "L" => {
                if !left.is_empty() {
                    right.push(left.pop().unwrap());
                }
            }
            "D" => {
                if !right.is_empty() {
                    left.push(right.pop().unwrap());
                }
            }
            "B" => {
                if !left.is_empty() {
                    left.pop();
                }
            }
            "P" => {
                // P x 형태에서 x를 꺼내 왼쪽 스택에 추가
                let ch = parts[1].chars().next().unwrap();
                left.push(ch);
            }
            _ => {}
        }
    }

    // right 스택에 있는 문자들을 역순으로 왼쪽에 붙여 최종 문자열 완성
    while let Some(ch) = right.pop() {
        left.push(ch);
    }

    // 출력
    let result: String = left.into_iter().collect();
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/1406
// 에디터
#[test]
fn test_solve1406() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "abcd
3
P x
L
P y"
            .to_string(),
            want: "abcdyx".to_string(),
        },
        TestData {
            s: "abc
9
L
L
L
L
L
P x
L
B
P y"
            .to_string(),
            want: "yxabc".to_string(),
        },
        TestData {
            s: "dmih
11
B
B
P x
L
B
B
B
P y
D
D
P z"
            .to_string(),
            want: "yxz".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1406(&mut reader, &mut writer);

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
