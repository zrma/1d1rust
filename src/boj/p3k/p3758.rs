use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3758(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    for _ in 0..num_cases {
        let (n, k, my_team, m) = read_values_as!(read_line(reader), usize, usize, usize, usize);

        // 팀 정보를 저장할 구조
        // scores[team_id][problem_id] = 해당 문제에 대한 최고 점수
        let mut scores = vec![vec![0; k + 1]; n + 1];
        // 각 팀의 제출 횟수
        let mut submissions_count = vec![0; n + 1];
        // 각 팀의 마지막 제출 시간(인덱스)
        let mut last_submission_time = vec![0; n + 1];

        for time in 1..=m {
            let line = read_line(reader);
            let mut iter = line.split_whitespace();
            let i = iter.next().unwrap().parse::<usize>().unwrap(); // 팀 ID
            let j = iter.next().unwrap().parse::<usize>().unwrap(); // 문제 번호
            let s = iter.next().unwrap().parse::<usize>().unwrap(); // 점수

            // 제출 정보 갱신
            submissions_count[i] += 1;
            if s > scores[i][j] {
                scores[i][j] = s;
            }
            last_submission_time[i] = time;
        }

        // 팀별 최종 점수 계산
        let mut teams = Vec::with_capacity(n);
        for team_id in 1..=n {
            let total_score: usize = scores[team_id].iter().sum();
            teams.push((
                total_score,
                submissions_count[team_id],
                last_submission_time[team_id],
                team_id,
            ));
        }

        // 정렬: 점수 내림차순, 제출 횟수 오름차순, 마지막 제출 시간 오름차순
        teams.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));

        // my_team의 순위 찾기
        let rank = teams
            .iter()
            .position(|&(_, _, _, tid)| tid == my_team)
            .map(|idx| idx + 1)
            .unwrap();

        writeln!(writer, "{}", rank).unwrap();
    }
}

// https://www.acmicpc.net/problem/3758
// KCPC
#[test]
fn test_solve3758() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
3 4 3 5
1 1 30
2 3 30
1 2 40
1 2 20
3 1 70
4 4 1 10
1 1 50
2 1 20
1 1 80
3 1 0
1 2 20
2 2 10
4 3 0
2 1 0
2 2 100
1 4 20"
                .to_string(),
            want: "1
2"
            .to_string(),
        },
        TestData {
            s: "1
5 4 3 10
1 1 30
2 3 30
1 2 40
1 2 20
3 1 70
5 1 70
4 1 70
4 1 70
4 1 70
3 1 70"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "1
6 4 3 20
1 1 30
2 3 30
1 2 40
1 2 20
3 1 70
5 1 120
4 1 70
4 2 50
3 1 50
3 2 50
5 1 110
6 1 122
2 3 15
4 1 15
1 4 22
5 1 12
2 3 11
2 4 12
3 2 15
2 4 16"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "1
4 2 3 5
1 1 30
2 1 50
3 2 60
1 2 20
1 1 100"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "1
2 1 1 2
1 1 1
2 1 2"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3758(&mut reader, &mut writer);

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
