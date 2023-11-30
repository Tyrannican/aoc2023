use reqwest;
use std::env;
use std::fs;

pub trait Solve {
    fn process_input(&mut self, path: &str);
    fn part1(&mut self);
    fn part2(&mut self);
}

pub fn read_file(fp: &str) -> String {
    let mut path = std::env::current_dir().unwrap();
    path = path.join(format!("src/problems/{}", fp));
    let mut contents = fs::read_to_string(&path).unwrap();
    if contents.is_empty() {
        contents = load_input_from_aoc(fp.to_string());
        fs::write(&path, &contents).unwrap();
    }

    return contents;
}

fn get_env_vars() -> (String, String) {
    let aoc_session = env::var("ADVENT_OF_CODE_SESSION");
    let aoc_year = env::var("ADVENT_OF_CODE_YEAR");

    if aoc_session.is_err() || aoc_year.is_err() {
        eprintln!("No environment variables set to automatically get AoC input!");
        eprintln!("Create a .env file with ADVENT_OF_CODE_SESSION set to your AoC session and ADVENT_OF_CODE_YEAR set to the appropriate year!");
        eprintln!("Or, just simply fill in the `input.txt` file for the day manually :D");
        std::process::exit(0);
    }

    return (aoc_session.unwrap(), aoc_year.unwrap());
}

fn load_input_from_aoc(day: String) -> String {
    let day_num = day
        .chars()
        .into_iter()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<i32>()
        .unwrap();

    let (aoc_session, aoc_year) = get_env_vars();
    let url = format!("https://adventofcode.com/{aoc_year}/day/{day_num}/input");
    let client = reqwest::blocking::Client::new();

    client
        .get(url)
        .header("Cookie", format!("session={aoc_session}"))
        .send()
        .unwrap()
        .text()
        .unwrap()
}
