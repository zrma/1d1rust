use std::io::Write;

#[allow(dead_code)]
fn solve4690(writer: &mut impl Write) {
    for a in 2..=100 {
        let a_cubed = a * a * a;
        for b in 2..a {
            let b_cubed = b * b * b;
            for c in b..a {
                let c_cubed = c * c * c;
                for d in c..a {
                    if a_cubed == b_cubed + c_cubed + d * d * d {
                        writeln!(writer, "Cube = {}, Triple = ({},{},{})", a, b, c, d).unwrap();
                    }
                }
            }
        }
    }
}

// https://www.acmicpc.net/problem/4690
// 완전 세제곱
#[test]
fn test_solve4690() {
    struct TestData {
        want: String,
    }
    for (i, data) in [TestData {
        want: "Cube = 6, Triple = (3,4,5)
Cube = 12, Triple = (6,8,10)
Cube = 18, Triple = (2,12,16)
Cube = 18, Triple = (9,12,15)
Cube = 19, Triple = (3,10,18)
Cube = 20, Triple = (7,14,17)
Cube = 24, Triple = (12,16,20)
Cube = 25, Triple = (4,17,22)
Cube = 27, Triple = (3,18,24)
Cube = 28, Triple = (18,19,21)
Cube = 29, Triple = (11,15,27)
Cube = 30, Triple = (15,20,25)
Cube = 36, Triple = (4,24,32)
Cube = 36, Triple = (18,24,30)
Cube = 38, Triple = (6,20,36)
Cube = 40, Triple = (14,28,34)
Cube = 41, Triple = (2,17,40)
Cube = 41, Triple = (6,32,33)
Cube = 42, Triple = (21,28,35)
Cube = 44, Triple = (16,23,41)
Cube = 45, Triple = (5,30,40)
Cube = 46, Triple = (3,36,37)
Cube = 46, Triple = (27,30,37)
Cube = 48, Triple = (24,32,40)
Cube = 50, Triple = (8,34,44)
Cube = 53, Triple = (29,34,44)
Cube = 54, Triple = (6,36,48)
Cube = 54, Triple = (12,19,53)
Cube = 54, Triple = (27,36,45)
Cube = 56, Triple = (36,38,42)
Cube = 57, Triple = (9,30,54)
Cube = 58, Triple = (15,42,49)
Cube = 58, Triple = (22,30,54)
Cube = 60, Triple = (21,42,51)
Cube = 60, Triple = (30,40,50)
Cube = 63, Triple = (7,42,56)
Cube = 66, Triple = (33,44,55)
Cube = 67, Triple = (22,51,54)
Cube = 69, Triple = (36,38,61)
Cube = 70, Triple = (7,54,57)
Cube = 71, Triple = (14,23,70)
Cube = 72, Triple = (8,48,64)
Cube = 72, Triple = (34,39,65)
Cube = 72, Triple = (36,48,60)
Cube = 75, Triple = (12,51,66)
Cube = 75, Triple = (38,43,66)
Cube = 76, Triple = (12,40,72)
Cube = 76, Triple = (31,33,72)
Cube = 78, Triple = (39,52,65)
Cube = 80, Triple = (28,56,68)
Cube = 81, Triple = (9,54,72)
Cube = 81, Triple = (25,48,74)
Cube = 82, Triple = (4,34,80)
Cube = 82, Triple = (12,64,66)
Cube = 82, Triple = (19,60,69)
Cube = 84, Triple = (28,53,75)
Cube = 84, Triple = (42,56,70)
Cube = 84, Triple = (54,57,63)
Cube = 85, Triple = (50,61,64)
Cube = 87, Triple = (20,54,79)
Cube = 87, Triple = (26,55,78)
Cube = 87, Triple = (33,45,81)
Cube = 87, Triple = (38,48,79)
Cube = 88, Triple = (21,43,84)
Cube = 88, Triple = (25,31,86)
Cube = 88, Triple = (32,46,82)
Cube = 89, Triple = (17,40,86)
Cube = 90, Triple = (10,60,80)
Cube = 90, Triple = (25,38,87)
Cube = 90, Triple = (45,60,75)
Cube = 90, Triple = (58,59,69)
Cube = 92, Triple = (6,72,74)
Cube = 92, Triple = (54,60,74)
Cube = 93, Triple = (32,54,85)
Cube = 95, Triple = (15,50,90)
Cube = 96, Triple = (19,53,90)
Cube = 96, Triple = (48,64,80)
Cube = 97, Triple = (45,69,79)
Cube = 99, Triple = (11,66,88)
Cube = 100, Triple = (16,68,88)
Cube = 100, Triple = (35,70,85)
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut writer = vec![];
        solve4690(&mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {}", i);
    }
}
