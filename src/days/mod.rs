pub mod day_1;

pub trait Chellange {
    fn solve_first(&self, input: &str) -> color_eyre::Result<String>;
    fn solve_second(&self, input: &str) -> color_eyre::Result<String>;
    fn get_day(&self) -> u8;
}