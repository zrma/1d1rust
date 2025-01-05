use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20006(reader: &mut impl BufRead, writer: &mut impl Write) {
    let first_line = crate::utils::io::read_line(reader);
    let (p, m) = read_values_as!(first_line, usize, usize);

    #[derive(Clone)]
    struct Player {
        level: i32,
        nickname: String,
    }

    struct Room {
        min_level: i32,
        max_level: i32,
        capacity: usize,
        players: Vec<Player>,
    }

    let mut rooms: Vec<Room> = vec![];

    for _ in 0..p {
        let (level, nickname) = read_values_as!(read_line(reader), i32, String);

        // 기존 방에 들어갈 수 있는지 확인
        let mut entered = false;
        for room in rooms.iter_mut() {
            if (room.min_level <= level)
                && (level <= room.max_level)
                && (room.players.len() < room.capacity)
            {
                room.players.push(Player {
                    level,
                    nickname: nickname.clone(),
                });
                entered = true;
                break;
            }
        }

        // 들어갈 방이 없으면 새 방을 생성
        if !entered {
            rooms.push(Room {
                min_level: level - 10,
                max_level: level + 10,
                capacity: m,
                players: vec![Player { level, nickname }],
            });
        }
    }

    // 출력
    for room in &mut rooms {
        if room.players.len() == room.capacity {
            writeln!(writer, "Started!").unwrap();
        } else {
            writeln!(writer, "Waiting!").unwrap();
        }

        // 닉네임 사전순으로 정렬
        room.players.sort_by(|a, b| a.nickname.cmp(&b.nickname));

        for player in &room.players {
            writeln!(writer, "{} {}", player.level, player.nickname).unwrap();
        }
    }
}

// https://www.acmicpc.net/problem/20006
// 랭킹전 대기열
#[test]
fn test_solve20006() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10 5
10 a
15 b
20 c
25 d
30 e
17 f
18 g
26 h
24 i
28 j"
                .to_string(),
            want: "Started!
10 a
15 b
20 c
17 f
18 g
Started!
25 d
30 e
26 h
24 i
28 j"
                .to_string(),
        },
        TestData {
            s: "2 3
10 az
11 bc"
                .to_string(),
            want: "Waiting!
10 az
11 bc"
                .to_string(),
        },
        TestData {
            s: "1 1
10 a"
                .to_string(),
            want: "Started!
10 a"
                .to_string(),
        },
        TestData {
            s: "3 2
15 a
3 b
1 c"
            .to_string(),
            want: "Waiting!
15 a
Started!
3 b
1 c"
            .to_string(),
        },
        TestData {
            s: "11 5
10 a
15 b
20 c
25 d
30 e
17 f
18 g
26 h
24 i
28 j
10 k"
                .to_string(),
            want: "Started!
10 a
15 b
20 c
17 f
18 g
Started!
25 d
30 e
26 h
24 i
28 j
Waiting!
10 k"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20006(&mut reader, &mut writer);

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
