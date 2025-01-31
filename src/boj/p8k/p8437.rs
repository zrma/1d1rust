use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8437(reader: &mut impl BufRead, writer: &mut impl Write) {
    let total = read_line(reader);
    let diff = read_line(reader);

    // 문자열을 숫자 벡터로 변환
    let total: Vec<u8> = total.bytes().map(|b| b - b'0').collect();
    let diff: Vec<u8> = diff.bytes().map(|b| b - b'0').collect();

    // 큰 수의 덧셈과 나눗셈을 수행
    let mut x = add_big_numbers(&total, &diff);
    for i in 0..x.len() {
        if x[i] % 2 == 1 && i + 1 < x.len() {
            x[i + 1] += 10;
        }
        x[i] /= 2;
    }
    while x.len() > 1 && x[0] == 0 {
        x.remove(0);
    }

    let y = subtract_big_numbers(&total, &x);

    // 결과 출력
    for digit in x {
        write!(writer, "{}", digit).unwrap();
    }
    writeln!(writer).unwrap();

    for digit in y {
        write!(writer, "{}", digit).unwrap();
    }
    writeln!(writer).unwrap();
}

// 큰 수의 덧셈
fn add_big_numbers(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut carry = 0;
    let mut i = a.len() as i32 - 1;
    let mut j = b.len() as i32 - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let sum =
            carry + if i >= 0 { a[i as usize] } else { 0 } + if j >= 0 { b[j as usize] } else { 0 };
        result.push(sum % 10);
        carry = sum / 10;
        i -= 1;
        j -= 1;
    }

    result.reverse();
    result
}

// 큰 수의 뺄셈
fn subtract_big_numbers(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut borrow = 0;
    let mut i = a.len() as i32 - 1;
    let mut j = b.len() as i32 - 1;

    while i >= 0 {
        let mut diff = a[i as usize] as i32 - borrow;
        if j >= 0 {
            diff -= b[j as usize] as i32;
        }

        if diff < 0 {
            diff += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        result.push(diff as u8);
        i -= 1;
        j -= 1;
    }

    result.reverse();
    while result.len() > 1 && result[0] == 0 {
        result.remove(0);
    }
    result
}

// https://www.acmicpc.net/problem/8437
// Julka
#[test]
fn test_solve8437() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "10
2"
            .to_string(),
            want: "6
4"
            .to_string(),
        },
        TestCase {
            s: "18
2"
            .to_string(),
            want: "10
8"
            .to_string(),
        },
        TestCase {
            s: "1000000000000000000000000000000
2"
            .to_string(),
            want: "500000000000000000000000000001
499999999999999999999999999999"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8437(&mut reader, &mut writer);

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
