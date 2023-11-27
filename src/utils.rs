use std::fs;

pub trait Solve {
    fn process_input(&mut self, path: &str);
    fn part1(&mut self);
    fn part2(&mut self);
}

pub fn read_file(fp: &str) -> String {
    let mut path = std::env::current_dir().unwrap();
    path = path.join(format!("src/problems/{}", fp));
    fs::read_to_string(path).unwrap()
}
