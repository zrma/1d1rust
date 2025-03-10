use std::convert::{TryFrom, TryInto};

pub fn check_palindrome_nth(s: &[u8], i: usize) -> bool {
    s[i] == s[s.len() - 1 - i]
}

pub fn try_next_pos(
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> Option<(usize, usize)> {
    let nx = x.checked_add_signed(dx)?;
    let ny = y.checked_add_signed(dy)?;
    if nx < w && ny < h {
        Some((nx, ny))
    } else {
        None
    }
}

pub fn char_to_index<T>(c: char) -> T
where
    T: TryFrom<u64> + Copy,
{
    let sub = match c {
        'a'..='z' => b'a',
        'A'..='Z' => b'A',
        '0'..='9' => b'0',
        _ => panic!("Character is not a valid ASCII letter"),
    };

    let value = u64::from(c)
        .checked_sub(u64::from(sub))
        .expect("Subtraction underflow");
    T::try_from(value).ok().expect("Conversion failed")
}

pub fn char_to_digit<T: TryFrom<u64> + Copy>(c: char) -> T {
    u64::from(c).try_into().ok().expect("Conversion failed")
}

/// 복소수 구조체 (Re, Im)
#[derive(Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

/// 분할 정복 방식 FFT
fn fft(a: &mut [Complex], invert: bool) {
    let n = a.len();
    // 비트 반전(bit reversal) 정렬
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    // Cooley–Tukey
    let mut length = 2;
    while length <= n {
        let angle = 2.0 * std::f64::consts::PI / (length as f64) * if invert { -1.0 } else { 1.0 };
        let wlen = Complex {
            re: angle.cos(),
            im: angle.sin(),
        };
        for i in (0..n).step_by(length) {
            let mut w = Complex { re: 1.0, im: 0.0 };
            for j in 0..(length / 2) {
                let u = a[i + j];
                let v = mul(a[i + j + length / 2], w);
                a[i + j] = add(u, v);
                a[i + j + length / 2] = sub(u, v);
                w = mul(w, wlen);
            }
        }
        length <<= 1;
    }
    // 역변환 시 스케일링
    if invert {
        for x in a {
            x.re /= n as f64;
            x.im /= n as f64;
        }
    }
}

/// 두 복소수 곱
fn mul(a: Complex, b: Complex) -> Complex {
    Complex {
        re: a.re * b.re - a.im * b.im,
        im: a.re * b.im + a.im * b.re,
    }
}

/// 두 복소수 합/차
fn add(a: Complex, b: Complex) -> Complex {
    Complex {
        re: a.re + b.re,
        im: a.im + b.im,
    }
}
fn sub(a: Complex, b: Complex) -> Complex {
    Complex {
        re: a.re - b.re,
        im: a.im - b.im,
    }
}

/// 큰 수 곱 (FFT 버전, 개념 스케치)
pub(crate) fn multiply_fft(a_str: &str, b_str: &str) -> String {
    // 1) 숫자 -> 배열
    let a: Vec<i32> = a_str.bytes().rev().map(|c| (c - b'0') as i32).collect();
    let b: Vec<i32> = b_str.bytes().rev().map(|c| (c - b'0') as i32).collect();
    let n = a.len() + b.len();

    // 2) FFT용 길이(2의 거듭제곱) 계산
    let mut size = 1;
    while size < n {
        size <<= 1;
    }

    // 3) 실수부에 자리수 배치, 허수부는 0
    let mut fa: Vec<Complex> = a
        .into_iter()
        .map(|v| Complex {
            re: v as f64,
            im: 0.0,
        })
        .collect();
    fa.resize(size, Complex { re: 0.0, im: 0.0 });

    let mut fb: Vec<Complex> = b
        .into_iter()
        .map(|v| Complex {
            re: v as f64,
            im: 0.0,
        })
        .collect();
    fb.resize(size, Complex { re: 0.0, im: 0.0 });

    // 4) FFT 변환
    fft(&mut fa, false);
    fft(&mut fb, false);

    // 5) 주파수영역에서 곱
    for i in 0..size {
        fa[i] = mul(fa[i], fb[i]);
    }

    // 6) 역 FFT
    fft(&mut fa, true);

    // 7) 반올림 + 캐리 처리
    //    fa[i]의 실수부를 반올림하면 10^i 자리의 값
    let mut result = vec![0i64; size];
    for i in 0..size {
        let val = fa[i].re.round() as i64; // 부동소수점 오차 주의
        result[i] = val;
    }
    // 캐리
    for i in 0..(size - 1) {
        let carry = result[i] / 10;
        result[i] %= 10;
        result[i + 1] += carry;
    }
    // 맨 앞자리 0 제거
    while result.len() > 1 && *result.last().unwrap() == 0 {
        result.pop();
    }
    // 뒤집어서 출력
    result
        .iter()
        .rev()
        .map(|d| (b'0' + *d as u8) as char)
        .collect()
}
