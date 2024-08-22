const NUMBER_COUNT: usize = 11;
const NUMBER: [i32; NUMBER_COUNT] = [
    31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,
];

const ROW_COUNT: usize = 5;
const COL_COUNT: usize = 3;

fn main() {
    let q = "fn main() {\n    let q = \"?\";\n\n    for c in q.chars().collect::<Vec<char>>() {\n        if c as u8 == 63 {\n            for c in q.chars() {\n                match c {\n                    '\\n' => print!(\"\\\\n\"),\n                    '\"' => print!(\"\\\\\\\"\"),\n                    '\\\\' => print!(\"\\\\\\\\\"),\n                    c => print!(\"{c}\"),\n                }\n                continue;\n            }\n        } else {\n            print!(\"{c}\");\n        }\n    }\n}\n";

    for c in q.chars().collect::<Vec<char>>() {
        if c as u8 == 63 {
            for c in q.chars() {
                match c {
                    '\n' => print!("\\n"),
                    '"' => print!("\\\""),
                    '\\' => print!("\\\\"),
                    c => print!("{c}"),
                }
                continue;
            }
        } else {
            print!("{c}");
        }
    }
    println!("");

    print_number(7);
}

fn print_number(number: usize) {
    for y in 0..ROW_COUNT {
        for x in 0..COL_COUNT {
            let c = (ROW_COUNT - y - 1) * COL_COUNT + x;
            if ((NUMBER[number] >> c) & 1) == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}
