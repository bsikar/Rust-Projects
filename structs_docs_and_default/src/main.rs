use std::default::Default;

#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Default for Month {
    fn default() -> Self {
        Month::January
    }
}

impl Month {
    fn new() -> Self {
        Default::default()
    }

    fn from_u8(x: u8) -> Self {
        use Month::*;

        match x {
            1 => January,
            2 => February,
            3 => March,
            4 => April,
            5 => May,
            6 => June,
            7 => July,
            8 => August,
            9 => September,
            10 => October,
            11 => November,
            12 => December,
            _ => Default::default(),
        }
    }
}

#[derive(Debug, Default)]
struct BirthDay {
    day: u8,
    month: Month,
    year: u16,
}

impl BirthDay {
    fn new(day: u8, month: Month, year: u16) -> Self {
        Self { day, month, year }
    }
}

#[derive(Debug, Default)]
struct Player<'a> {
    name: &'a str,
    age: u8,
    birth_day: BirthDay,
}

fn main() {}
