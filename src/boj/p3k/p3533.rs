use crate::utils::io::read_n_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3533(reader: &mut impl BufRead, writer: &mut impl Write) {
    let arr: Vec<u8> = read_n_values(reader, 10);
    let sum = arr.iter().sum::<u8>();

    //  s | n | nC2 | nC3 | nC2 + nC3 | 홀짝 | f = 1 - ((nC2 + nC3) % 2)
    //  0  10   45   120       165         1     0
    //  1   9   36    84       120         0     1
    //  2   8   28    56        84         0     1
    //  3   7   21    35        56         0     1
    //  4   6   15    20        35         1     0
    //  5   5   10    10        20         0     1
    //  6   4    6     4        10         0     1
    //  7   3    3     1         4         0     1
    //  8   2    1     0         1         1     0
    //  9   1    0     0         0         0     1
    // 10   0    0     0         0         0     1
    if sum % 4 == 0 {
        writeln!(writer, "0").unwrap();
    } else {
        writeln!(writer, "1").unwrap();
    }
}

#[allow(dead_code)]
fn solve3533_explicit_formula(reader: &mut impl BufRead, writer: &mut impl Write) {
    let arr: Vec<u8> = read_n_values(reader, 10);
    let sum = arr.iter().sum::<u8>();

    let n = 10 - sum;

    // NOTE
    // 수 10개 주어짐
    // 각 수는 0 | 1
    //
    // 두 수 쌍의 OR 1인 경우 = 두 수 중 하나라도 1인 경우
    // 두 수 쌍의 OR 0인 경우 = 두 수 모두 0
    //
    // 세 수 쌍의 OR 1인 경우 = 세 수 중 하나라도 1인 경우
    // 세 수 쌍의 OR 0인 경우 = 세 수 모두 0
    //
    // 모든 두 수 쌍과 모든 세 수쌍의 결과 XOR 0이다? = 1이 짝수번 등장 = 합이 짝수
    // 모든 두 수 쌍과 모든 세 수쌍의 결과 XOR 1이다? = 1이 홀수번 등장 = 합이 홀수
    //
    // 두 수 쌍 경우의 수 = 10C2 = 45
    // 세 수 쌍 경우의 수 = 10C3 = 120
    //
    // 1인 변수의 개수 = 변수 합(각 변수가 0 | 1 이므로) = sum
    // 0인 변수의 개수 = 10 - 1인 변수의 개수 = 10 - sum
    //
    // 두 수 쌍 결과 OR 0인 경우 = 0인 변수 중 2개 뽑기 = (10 - sum)C2 = nC2
    // 세 수 쌍 결과 OR 0인 경우 = 0인 변수 중 3개 뽑기 = (10 - sum)C3 = nC3
    //
    // 두 수 쌍 결과 OR 1인 경우 = 적어도 하나는 1 = 두 수 쌍 경우의 수 - 두 수 쌍 OR 0인 경우 = 45 - nC2
    // 세 수 쌍 결과 OR 1인 경우 = 적어도 하나는 1 = 세 수 쌍 경우의 수 - 세 수 쌍 OR 0인 경우 = 120 - nC3
    //
    // 둘을 합치면 45 + 120 - nC2 - nC3
    // (각 쌍의 결과가 0인 경우는 합쳐도 영향 안 끼치므로(0) 고려 안 함)
    //
    // (10C2 + 10C3 - nC2 - nC3) % 2 == 0 이면 0, 아니면 1
    // 45 + 120 - nC2 - nC3 = 165 - nC2 - nC3
    // (165 - nC2 - nC3) % 2 == 0
    // 1 - (nC2 + nC3) % 2 == 0 이면 0, 아니면 1
    // (nC2 + nC3) % 2 == 1 이면 0, 아니면 1

    if (combination(n, 2) + combination(n, 3)) % 2 == 1 {
        writeln!(writer, "0").unwrap();
    } else {
        writeln!(writer, "1").unwrap();
    }
}

fn combination(n: u8, k: u8) -> u8 {
    if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        let k = k.min(n - k);
        (1..=k).fold(1, |result, i| result * (n - k + i) / i)
    }
}

// https://www.acmicpc.net/problem/3533
// Explicit Formula
#[test]
fn test_solve3533() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 0 0 1 0 0 1 0 0 1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 1 1 1 1 1 1 1 1 1".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve3533(&mut reader, &mut writer);

            let got = String::from_utf8(writer).unwrap();
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve3533_explicit_formula(&mut reader, &mut writer);

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
}
