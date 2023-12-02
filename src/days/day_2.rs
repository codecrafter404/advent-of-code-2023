use super::Chellange;

struct Set {
    red: u32,
    green: u32,
    blue: u32
}
pub struct DayTwo;
impl Chellange for DayTwo {
    fn solve_first(&self, input: &str) -> color_eyre::Result<String> {
        let lines = input.split("\n");
        let mut res = 0;
        lines.for_each(|line| {
            let line = line.trim();
            if let Some((_, game_id, sets)) = lazy_regex::regex_captures!("^Game (\\d*):(.*)$", line) {
                let sets = self.parse_sets(sets);
                if sets.iter().all(|x| {
                    x.red <= 12 && x.green <= 13 && x.blue <= 14
                }) {
                    res += game_id.parse::<i32>().unwrap();
                }
            }

        });
        Ok(res.to_string())
    }

    fn solve_second(&self, input: &str) -> color_eyre::Result<String> {
        let lines = input.split("\n");
        let mut res = 0;
        lines.for_each(|line| {
            let line = line.trim();
            if let Some((_, game_id, sets)) = lazy_regex::regex_captures!("^Game (\\d*):(.*)$", line) {
                let sets = self.parse_sets(sets);
                let m_red = sets.iter().max_by(|a, b| a.red.cmp(&b.red)).unwrap().red;
                let m_green = sets.iter().max_by(|a, b| a.green.cmp(&b.green)).unwrap().green;
                let m_blue = sets.iter().max_by(|a, b| a.blue.cmp(&b.blue)).unwrap().blue;
                res += m_red * m_green * m_blue;
            }

        });
        Ok(res.to_string())
    }

    fn get_day(&self) -> u8 {
        2
    }
}
impl DayTwo {
    fn parse_sets(&self, sets: &str) -> Vec<Set> {
        let sets = sets.replace(" ", "");
        let sets = sets.split(";");
        let sets = sets.map(|x| {
            let cubes = x.split(",");
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            cubes.for_each(|cube| {
                if cube.ends_with("blue") {
                    blue += cube.strip_suffix("blue").unwrap().parse::<i32>().unwrap();
                }
                if cube.ends_with("green") {
                    green += cube.strip_suffix("green").unwrap().parse::<i32>().unwrap();
                }
                if cube.ends_with("red") {
                    red += cube.strip_suffix("red").unwrap().parse::<i32>().unwrap();
                }
            });
            Set {red: red as u32, green: green as u32, blue: blue as u32}
        }).collect::<Vec<_>>();
        sets
    }
}
#[test]
fn test_first() {
    let day = DayTwo {};
    assert_eq!(day.solve_first("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ").unwrap(), "8");
}
#[test]
fn test_second() {
    let day = DayTwo {};
    assert_eq!(day.solve_second("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(), "2286");
}