mod year2021;

use aoc_driver;
//use crate::year2021::day01;

fn main() {
    println!("Hello, world!");
    aoc_driver::aoc_complete! {
        session_file: ".session.txt"
        input_dir: "input"
        challenges: [
            {
                "2021-1-1": year2021::day01::part1,
                tests: [
                    { name: "1", input: "199
200
208
210
200
207
240
269
260
263", output: "7" }
                   // { name: "2", input: "14", output: "2" }
                ]
            }
            {
                "2021-1-2": year2021::day01::part2,
                tests: [
                    { name: "1", input: "100756", output: "50346" }
                ]
            }
        ]
    }
}
