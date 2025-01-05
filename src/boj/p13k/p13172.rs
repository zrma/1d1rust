use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13172(reader: &mut impl BufRead, writer: &mut impl Write) {
    let m: u64 = read_value(read_line(reader)); // 주사위의 개수 M

    let mut ans = 0u64;
    for _ in 0..m {
        // 각 줄에서 N, S를 읽고, ans에 (S × N^{-1}) mod m을 누적
        let (n, s) = read_values_as!(read_line(reader), u64, u64);
        let inv_n = mod_inv(n, MOD);
        ans = (ans + (s % MOD) * inv_n) % MOD;
    }

    // 결과 출력
    writeln!(writer, "{}", ans).expect("Failed to write answer");
}

const MOD: u64 = 1_000_000_007;

/// 거듭제곱을 이용해 a^(b) mod m 을 구하는 함수 (빠른 거듭제곱)
fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }
    result
}

/// 모듈러 곱셈에 대한 역원 계산: a^{-1} ≡ a^{m-2} (mod m)  [m이 소수일 때]
fn mod_inv(a: u64, m: u64) -> u64 {
    mod_pow(a, m - 2, m)
}

// https://www.acmicpc.net/problem/13172
// Σ
#[test]
fn test_solve13172() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
3 7"
            .to_string(),
            want: "333333338".to_string(),
        },
        // 간단 테스트: M=2, (2,3), (4,10)
        // sum = 3/2 + 10/4 = 1.5 + 2.5 = 4
        TestData {
            s: "2
            2 3
            4 10"
                .to_string(),
            want: "4".to_string(),
        },
        // 간단 테스트: M=2, (3,6), (5,15)
        // sum = 6/3 + 15/5 = 2 + 3 = 5
        TestData {
            s: "2
            3 6
            5 15"
                .to_string(),
            want: "5".to_string(),
        },
        // 조금 복합: M=2, (6,21), (8,36)
        // sum = 21/6 + 36/8 = 3.5 + 4.5 = 8
        TestData {
            s: "2
            6 21
            8 36"
                .to_string(),
            want: "8".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13172(&mut reader, &mut writer);

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
