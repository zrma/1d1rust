use std::any::type_name;
use std::io::{self, BufWriter};
use std::io::{BufRead, Write};

// ===== 필수 섹션: 수정하지 마세요 =====
// NOTE - function order
// 1. main
// 2. solve####
// 3. other boilerplate code
fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let writer = stdout.lock();
    let mut writer = BufWriter::new(writer);

    solve(&mut reader, &mut writer);

    writer.flush().unwrap();
}
// ===== 필수 섹션 끝 =====

// ===== 구현 섹션: 이 부분을 수정하세요 =====
// TODO: Implement solution function
// keep the suffix of the function name as the problem number
fn solve(reader: &mut impl BufRead, writer: &mut impl Write) {
    // 아래 예제 중 문제에 필요한 입력 방식만 선택하세요

    // [예제 1] 단일 값 읽기 (정수)
    // let n: i32 = read_value(read_line(reader));

    // [예제 2] 한 줄에서 여러 값 읽기 (정수)
    // let (a, b): (i32, i32) = read_values_as!(read_line(reader), i32, i32);

    // [예제 3] 공백으로 구분된 여러 값을 벡터로 읽기
    // let values: Vec<i32> = read_values(reader);

    // [예제 4] N개의 줄을 각각 읽기
    // let n: usize = read_value(read_line(reader));
    // for _ in 0..n {
    //     let line = read_line(reader);
    //     // 각 줄 처리...
    // }

    // TODO: 문제 로직 구현
    let ans = 0; // 임시 답변

    // 답 출력
    writeln!(writer, "{}", ans).unwrap();
}

// 필요한 경우 추가 함수를 이곳에 구현하세요
// fn sub_function() {
// }
// ===== 구현 섹션 끝 =====

// ===== 유틸리티 섹션: 필요한 함수만 포함하세요 =====
// 다음 함수들은 src/utils/io.rs에서 가져온 것입니다.
// 필요한 함수만 복사하여 사용하고, 원본 구현과 동기화를 유지하세요.

// [유틸리티 1] read_values_as! 매크로 사용 시 필요
// 한 줄에서 여러 타입의 값을 읽을 때 사용합니다.
#[macro_export]
macro_rules! read_values_as {
    ($line:expr, $( $t:ty ),+ ) => {{
        let ln = $line;
        let mut iter = ln.split_whitespace();
        (
            $(
                {
                    let token = iter
                        .next()
                        .expect("expected a token, but found none");
                    token.parse::<$t>()
                        .unwrap_or_else(|_| panic!(
                            "expected a value of type `{}` from '{}', but parsing failed",
                            stringify!($t),
                            token
                        ))
                },
            )+
        )
    }};
}

// [유틸리티 2] 기본 입력 함수 - 공백으로 구분된 값들을 벡터로 읽기
// 사용 예: let values: Vec<i32> = read_values(reader);
pub fn read_values<T: std::str::FromStr>(reader: &mut impl BufRead) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let line = read_line(reader);
    line.split_whitespace()
        .map(|s| {
            s.parse::<T>().unwrap_or_else(|_| {
                panic!(
                    "expected a value of type `{}` from '{}', but parsing failed",
                    type_name::<T>(),
                    s
                )
            })
        })
        .collect()
}

// [유틸리티 3] 한 줄 읽기 - 거의 모든 입력에 필요한 기본 함수
// 사용 예: let line = read_line(reader);
pub fn read_line(reader: &mut impl BufRead) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("expected to read a line from input, but encountered an error");
    line.trim().to_string()
}

// 추가 유틸리티 함수 - 필요한 경우만 복사하세요:
//
// [유틸리티 4] 공백으로 구분된 값들을 벡터로 읽기 (위의 read_values와 동일)
// pub fn read_values<T: std::str::FromStr>(reader: &mut impl BufRead) -> Vec<T>
//
// [유틸리티 5] 정확히 N개의 값을 벡터로 읽기
// pub fn read_n_values<T: std::str::FromStr>(reader: &mut impl BufRead, n: usize) -> Vec<T>
//
// [유틸리티 6] 2차원 맵/행렬 읽기
// pub fn read_map<T: std::str::FromStr>(reader: &mut impl BufRead, rows: usize, cols: usize) -> Vec<Vec<T>>
//
// [유틸리티 7] 행렬을 문자열로 변환
// pub fn matrix_to_str<T: ToString + std::fmt::Display>(mat: &[Vec<T>]) -> String
// ===== 유틸리티 섹션 끝 =====
