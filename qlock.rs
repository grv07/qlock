use std::time::{SystemTime, UNIX_EPOCH};

const NUMBER_COUNT: usize = 11;
const NUMBER: [i32; NUMBER_COUNT] = [
    31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,
];

const ROW_COUNT: usize = 5;
const COL_COUNT: usize = 3;
const PADDING: usize = 2;

fn main() {
    print_time();
}

fn print_time() {
    loop {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let (h, m, s) = (((t / 3600) + 6) % 12, ((t / 60) + 30) % 60, t % 60);

        println!("Current time: {}", format!("{h:02} : {m:02} : {s:02}"));

        println!("");

        let (h, m, s) = (h as usize, m as usize, s as usize);

        let hms: [usize; 8] = [h / 10, h % 10, 10, m / 10, m % 10, 10, s / 10, s % 10];

        let w = (COL_COUNT + PADDING) * (6 + 2);
        let h = ROW_COUNT;

        for y in 0..h {
            for x in 0..w {
                let i = x / (COL_COUNT + PADDING);

                let dx = x % (COL_COUNT + PADDING);

                let c = (ROW_COUNT - y - 1) * (COL_COUNT) + (dx);

                if dx < COL_COUNT && ((NUMBER[hms[i]] >> c) & 1) == 1 {
                    print!("\x1b[31m#\x1b[0m");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
        println!("");

        std::thread::sleep(std::time::Duration::from_secs(1));
        print!("{esc}c", esc = 27 as char);
    }
}
