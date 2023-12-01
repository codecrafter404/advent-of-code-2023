use super::Chellange;

pub struct DayOne;
impl Chellange for DayOne {
    fn solve_first(&self, input: &str) -> color_eyre::Result<String> {
        let input = input.split("\n");
        let mut sum = 0;
        input.into_iter().for_each(|line| {
            let x = line.chars().into_iter().filter(|x| x.is_digit(10)).collect::<Vec<_>>();
            let res = format!("{}{}", x.first().unwrap(), x.last().unwrap()).parse::<i32>().unwrap();
            sum += res;
        });
        Ok(sum.to_string())
    }

    fn solve_second(&self, input: &str) -> color_eyre::Result<String> {
        let numbers = vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let input = input.split("\n");
        let mut sum = 0;
        input.into_iter().for_each(|line| {
            let mut res = String::new();
            line.chars().for_each(|c| {
                if c.is_digit(10) {
                    res += numbers.iter().find(|x| x.1.to_string() == c.to_string()).unwrap().0;
                }else {
                    res += &c.to_string();
                }
            });
            let res = res.trim();
            let mut canidates = Vec::new();
            numbers.iter().for_each(|num| {
                if res.contains(num.0) {
                    res.match_indices(num.0).for_each(|x| {
                        canidates.push((x.0, num.clone()));
                    });
                }
            });
            canidates.sort_by(|a, b| a.0.cmp(&b.0));
            let num = format!("{}{}", canidates.first().unwrap().1.1, canidates.last().unwrap().1.1);
            sum += num.parse::<i32>().unwrap();

        });
        Ok(sum.to_string())
    }

    fn get_day(&self) -> u8 {
        1
    }
}
#[test]
fn test_first(){
    let day = DayOne {};
    assert_eq!(day.solve_first("1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet").unwrap(), "142");
}
#[test]
fn test_second(){
    let day = DayOne {};
    assert_eq!(day.solve_second("two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen").unwrap(), "281");
}