use std::collections::BTreeSet;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

mod runner;

fn main() {
    let mut bench = false;
    let mut days = BTreeSet::new();
    for arg in std::env::args().skip(1) {
        if arg == "--bench" {
            bench = true;
        } else if arg == "--all" {
            days.extend(1..=25);
        } else {
            let day = arg
                .parse::<u8>()
                .ok()
                .filter(|&d| d != 0 && d <= 25)
                .expect("Argument should be a day number, `--bench` or `--all`");
            days.insert(day);
        }
    }

    let mut runner = runner::Runner::new(bench);
    for day in days {
        match day {
            //1 => runner.run(day, day_01::part_1, day_01::part_2),
            //2 => runner.run(day, day_02::part_1, day_02::part_2),
            //3 => runner.run(day, day_03::part_1, day_03::part_2),
            //4 => runner.run(day, day_04::part_1, day_04::part_2),
            //5 => runner.run(day, day_05::part_1, day_05::part_2),
            //6 => runner.run(day, day_06::part_1, day_06::part_2),
            //7 => runner.run(day, day_07::part_1, day_07::part_2),
            //8 => runner.run(day, day_08::part_1, day_08::part_2),
            //9 => runner.run(day, day_09::part_1, day_09::part_2),
            //10 => runner.run(day, day_10::part_1, day_10::part_2),
            //11 => runner.run(day, day_11::part_1, day_11::part_2),
            //12 => runner.run(day, day_12::part_1, day_12::part_2),
            //13 => runner.run(day, day_13::part_1, day_13::part_2),
            //14 => runner.run(day, day_14::part_1, day_14::part_2),
            //15 => runner.run(day, day_15::part_1, day_15::part_2),
            //16 => runner.run(day, day_16::part_1, day_16::part_2),
            //17 => runner.run(day, day_17::part_1, day_17::part_2),
            //18 => runner.run(day, day_18::part_1, day_18::part_2),
            //19 => runner.run(day, day_19::part_1, day_19::part_2),
            //20 => runner.run(day, day_20::part_1, day_20::part_2),
            //21 => runner.run(day, day_21::part_1, day_21::part_2),
            //22 => runner.run(day, day_22::part_1, day_22::part_2),
            //23 => runner.run(day, day_23::part_1, day_23::part_2),
            //24 => runner.run(day, day_24::part_1, day_24::part_2),
            //25 => runner.run(day, day_25::part_1, day_25::part_2),
            _ => todo!("day {day} not implemented yet"),
        }
    }
}
