use std::fmt::Display;
use std::path::Path;


pub mod day_01;

pub trait Day<T: Display> {
    fn solve_part_1(&self, input: &str) -> T;
    fn solve_part_2(&self, input: &str) -> T;
    fn get_inputs(&self) -> &str;

    fn solve(&self) -> (&str, T, T) {
        let filename = Path::new(file!()).file_name().and_then(|s| s.to_str()).unwrap();

        let input = self.get_inputs();
        let result_1 = self.solve_part_1(&input);
        let result_2 = self.solve_part_2(&input);

        (filename, result_1, result_2)
    }
}
