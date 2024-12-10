use std::{env, error};

#[derive(Debug)]
struct Config {
    year: u16,
    day: u8,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Self, Box<dyn error::Error>> {
        args.next();
        let year = args.next().ok_or("No year provided")?.parse()?;
        let day = args.next().ok_or("No day provided")?.parse()?;
        Ok(Self { year, day })
    }
    fn execute(&self) {
        match (self.year, self.day) {
            (2015, 1) => crate::year_2015::day_1::run(),
            (2015, 2) => crate::year_2015::day_2::run(),
            (2015, 3) => crate::year_2015::day_3::run(),
            (2015, 4) => crate::year_2015::day_4::run(),
            (2024, 1) => crate::year_2024::day_1::run(),
            (2024, 2) => crate::year_2024::day_2::run(),
            (2024, 3) => crate::year_2024::day_3::run(),
            (2024, 4) => crate::year_2024::day_4::run(),
            _ => unreachable!(),
        }
    }
}

pub fn run(args: env::Args) -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(args)?;
    config.execute();
    Ok(())
}
