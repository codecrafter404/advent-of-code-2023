use super::Chellange;

pub struct DayFour {}
impl Chellange for DayFour {
    fn solve_first(&self, input: &str) -> color_eyre::Result<String> {
        let lines = input.split("\n");
        let mut score = 0;
        for line in lines {
            let card = Card::parse_card(line);
            score += card.get_score();
        }
        Ok(score.to_string())
    }

    fn solve_second(&self, input: &str) -> color_eyre::Result<String> {
        todo!()
    }

    fn get_day(&self) -> u8 {
        4
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>
}
impl Card {
    fn parse_card(card: &str) -> Self {
        println!("{}", card);
        let id = card.split(":").nth(0).unwrap().strip_prefix("Card ").unwrap().trim().parse::<u32>().unwrap();
        let winning_numbers = card.split(":").nth(1).unwrap().split("|").nth(0).unwrap().trim().split(" ").filter(|x| !x.replace(" ", "").is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<_>>();
        let my_numbers = card.split(":").nth(1).unwrap().split("|").nth(1).unwrap().trim().split(" ").filter(|x| !x.replace(" ", "").is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<_>>();
        Card {id, winning_numbers, my_numbers}
    }
    fn get_score(&self) -> u32 {
        let mut score = 0;
        for x in &self.my_numbers {
            if self.winning_numbers.contains(&x) {
                if score == 0 {
                    score = 1;
                }else {
                    score *= 2;
                }
            }
        }
        score
    }
}
#[test]
fn test_card_parse() {
    assert_eq!(
        Card::parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        Card {
            id: 1,
            winning_numbers: vec![41, 48,83, 86, 17],
            my_numbers: vec![83, 86, 6, 31, 17 , 9, 48, 53]
        }
    );
    assert_eq!(
        Card {
            id: 1,
            winning_numbers: vec![41, 48,83, 86, 17],
            my_numbers: vec![83, 86, 6, 31, 17 , 9, 48, 53]
        }.get_score(),
        8
    );
}
#[test]
fn test_first() {
    let day = DayFour {};
    assert_eq!(day.solve_first(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card   100: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card  6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ).unwrap(), "13")
}