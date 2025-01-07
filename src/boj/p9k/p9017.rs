use crate::utils::io::{read_line, read_n_values, read_value};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9017(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    for _ in 0..num_cases {
        let num_runners: usize = read_value(read_line(reader));
        let ranks: Vec<usize> = read_n_values(reader, num_runners);

        // 각 팀의 주자 수를 세어 6명 미만인 팀 찾기
        let mut team_counts = HashMap::new();
        for &team in ranks.iter() {
            *team_counts.entry(team).or_insert(0) += 1;
        }
        let valid_teams: Vec<_> = team_counts
            .iter()
            .filter(|&(_, &count)| count >= 6)
            .map(|(&team, _)| team)
            .collect();

        // 유효한 팀만 고려하여 등수 계산
        let mut team_runners: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut rank = 1;
        for &team in ranks.iter() {
            if valid_teams.contains(&team) {
                team_runners.entry(team).or_default().push(rank);
                rank += 1;
            }
        }

        // 각 팀의 점수 계산 (상위 4명)
        let mut team_scores: Vec<_> = team_runners
            .iter()
            .map(|(&team, runners)| {
                let score: usize = runners.iter().take(4).sum();
                let fifth = runners[4];
                (team, score, fifth)
            })
            .collect();

        // 점수 순으로 정렬 (동점시 5번째 주자 등수로 비교)
        team_scores.sort_by(|a, b| {
            if a.1 == b.1 {
                a.2.cmp(&b.2)
            } else {
                a.1.cmp(&b.1)
            }
        });

        writeln!(writer, "{}", team_scores[0].0).unwrap();
    }
}

// https://www.acmicpc.net/problem/9017
// 크로스 컨트리
#[test]
fn test_solve9017() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
15
1 2 3 3 1 3 2 4 1 1 3 1 3 3 1
18
1 2 3 1 2 3 1 2 3 3 3 3 2 2 2 1 1 1"
                .to_string(),
            want: "1
3"
            .to_string(),
        },
        TestData {
            s: "1
21
1 2 3 3 1 3 2 4 1 1 3 5 5 5 5 5 5 1 3 3 1"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1
12
1 2 1 1 2 2 2 2 1 1 2 1"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9017(&mut reader, &mut writer);

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
