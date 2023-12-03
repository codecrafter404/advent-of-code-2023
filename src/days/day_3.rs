use core::num;
use std::cmp::max;

use super::Chellange;

enum EnginePlan {
    Number,
    Symbol,
    None,
}

pub struct DayThree {}
impl Chellange for DayThree {
    fn solve_first(&self, input: &str) -> color_eyre::Result<String> {
        let lines = input.split("\n").filter(|x| *x != "").collect::<Vec<_>>();
        let mut part_sum = 0;
        for (y, line) in lines.iter().enumerate() {
            let line = line.trim().replace(" ", "");
            let mut cur = String::new();
            for (x, char) in line.chars().enumerate() {
                if char.is_digit(10) {
                    cur += &char.to_string();
                }

                if cur != "" && (!char.is_digit(10) || x + 1 == line.chars().count()) {
                    let mut adjacent = Vec::new();
                    if x + 1 < line.chars().count() || !char.is_digit(10) {
                        adjacent.push(char);
                        cur += &char.to_string();
                    }

                    let mut start_pos = x as i32 - cur.len() as i32;
                    if start_pos >= 0 {
                        let char = line.chars().nth(start_pos as usize).unwrap();
                        adjacent.push(char);
                        cur = char.to_string() + &cur;
                    }
                    start_pos = max(start_pos, 0);
                    if y as i32 - 1 >= 0 {
                        let mut top = lines[y - 1]
                            .chars()
                            .skip(start_pos as usize)
                            .take(cur.len())
                            .collect::<Vec<_>>();
                        adjacent.append(&mut top);
                    }
                    if y + 1 < lines.len() {
                        let mut bottom = lines[y + 1]
                            .chars()
                            .skip(start_pos as usize)
                            .take(cur.len())
                            .collect::<Vec<_>>();
                        adjacent.append(&mut bottom);
                    }

                    if adjacent.iter().any(|x| *x != '.' && !x.is_digit(10)) {
                        let cur =
                            String::from_iter(cur.chars().into_iter().filter(|x| x.is_digit(10)))
                                .parse::<i32>()
                                .unwrap();
                        part_sum += cur;
                    }
                    // println!("cur: {}; adjacent: {:?}; start_pos: {}", cur, adjacent, start_pos);
                    cur = String::new();
                }
            }
        }
        Ok(part_sum.to_string())
    }

    fn solve_second(&self, input: &str) -> color_eyre::Result<String> {
        let lines = input.split("\n").filter(|x| *x != "").collect::<Vec<_>>();
        let mut gears = Vec::new();
        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                if char == '*' {
                    gears.push((x, y))
                }
            })
        });
        let sides = vec![
            // sides
            (-1, 0),
            (1, 0),
            // top
            (0, 1),
            (-1, 1),
            (1, 1),
            // bottom
            (0, -1),
            (-1, -1),
            (1, -1),
        ];
        let mut gear_ratio_sum = 0;
        for (x, y) in gears {
            let mut adjacent_numbers = Vec::new();
            for (dx, dy) in &sides {
                let x = x as i32 + dx;
                let y = y as i32 + dy;

                if x < 0
                    || y < 0
                    || y >= lines.len() as i32
                    || x >= lines[y as usize].chars().count() as i32
                {
                    continue;
                }
                let char = lines[y as usize].chars().collect::<Vec<_>>()[x as usize];
                if char.is_digit(10) {
                    if !adjacent_numbers.contains(&(x - 1, y)) &&  !adjacent_numbers.contains(&(x + 1, y)){
                        adjacent_numbers.push((x, y));
                    }
                }
            }
            let mut numbers = Vec::new();
            for (x, y) in &adjacent_numbers {
                let line = lines[*y as usize];
                let mut number = line.chars().collect::<Vec<_>>()[*x as usize].to_string();
                for dx in 1.. {
                    if (x - dx) >= 0 {
                        let lchar = line.chars().collect::<Vec<_>>()[(x - dx) as usize];
                        if lchar.is_digit(10) {
                            number = lchar.to_string() + &number;
                        }else {
                            break;
                        }
                    }else {
                        break;
                    }
                }
                for dx in 1.. {
                    if (x + dx) < line.chars().count() as i32{
                        let rchar = line.chars().collect::<Vec<_>>()[(x + dx) as usize];
                        if rchar.is_digit(10) {
                            number = number + &rchar.to_string();
                        }else {
                            break;
                        }
                    }else {
                        break;
                    }
                }
                numbers.push(number.parse::<i32>().unwrap());
            }

            println!("G{:?} | Nums:{:?} | adjacent: {:?}",(x, y), numbers, adjacent_numbers);

            if numbers.len() > 1 {
                gear_ratio_sum += numbers[0] * numbers[1];
            }
        }
        Ok(gear_ratio_sum.to_string())
    }

    fn get_day(&self) -> u8 {
        3
    }
}
#[test]
fn test_first() {
    let day = DayThree {};
    assert_eq!(
        day.solve_first(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        )
        .unwrap(),
        "4361"
    );
    assert_eq!(
        day.solve_first(
            "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"
        )
        .unwrap(),
        "925"
    );
    assert_eq!(
        day.solve_first(
            "........
.24$-4..
......*."
        )
        .unwrap(),
        "28"
    );
}
#[test]
fn test_second() {
    let day = DayThree {};
    assert_eq!(day.solve_second("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..").unwrap(),"467835");
}