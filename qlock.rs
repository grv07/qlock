use std::time::{SystemTime, UNIX_EPOCH};const NUMBER_COUNT: usize = 11;
const NUMBER: [i32; NUMBER_COUNT] = [31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,];
const ROW_COUNT: usize =5;const COL_COUNT: usize =3;const PADDING: usize =0;
fn main() { interpret_quine(); }
fn pixel(x: usize, y: usize, v: &str) {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let (h, m, s) = (((t / 3600) + 6) % 12, ((t / 60) + 30) % 60, t % 60);
    let hms: [usize; 8] = [h / 10, h % 10, 10, m / 10, m % 10, 10, s / 10, s % 10];
    let i = x / (COL_COUNT + 2);

    let dx = x % (COL_COUNT + 2);
    if i < 8
        && y < ROW_COUNT
        && dx < COL_COUNT
        && ((NUMBER[hms[i]] >> (ROW_COUNT - y - 1) * COL_COUNT + dx) & 1) == 1
    {
        print!("\x1b[31m{v}\x1b[0m");
    } else {
        print!("{v}");
    }
}

fn interpret_quine() {
    let q = "use std::time::{SystemTime, UNIX_EPOCH};const NUMBER_COUNT: usize = 11;\nconst NUMBER: [i32; NUMBER_COUNT] = [31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,];\nconst ROW_COUNT: usize = 5;const COL_COUNT: usize = 3;const PADDING: usize = 2;fn main(){interpret_quine();}\nfn pixel(x: usize, y: usize) {let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;let (h, m, s) = (((t / 3600) + 6) % 12, ((t / 60) + 30) % 60, t % 60);\nlet hms: [usize; 8] = [h / 10, h % 10, 10, m / 10, m % 10, 10, s / 10, s % 10];\nlet i = x / (COL_COUNT + PADDING);\n\nlet dx = x % (COL_COUNT + PADDING);\n\nlet c = (ROW_COUNT - y - 1) * (COL_COUNT) + (dx);\n\nif dx < COL_COUNT && ((NUMBER[hms[i]] >> c) & 1) == 1 {\nprint!(\"\\x1b[31m#\\x1b[0m\");\n} else {\nprint!(\"#\");\n}\n}\n\nfn interpret_quine() {\nlet q = \"?\";\n\nfor c in q.chars().collect::<Vec<char>>() {\nif c as u8 == 63 {\nfor c in q.chars() {\nif c == '\\n' {\nprint!(\"\\\\n\");\n} else if c == '\"' {\nprint!(\"\\\\\\\"\");\n} else if c == '\\\\' {\nprint!(\"\\\\\\\\\");\n} else {\nprint!(\"{c}\");\n}\n}\n} else {\nprint!(\"{c}\");\n}\n}\nprintln!(\"\");\n}\n";
    let mut x = 0;
    let mut y = 0;
    for c in q.chars().collect::<Vec<char>>() {
        if y >= ROW_COUNT {
            break;
        }
        if c as u8 == 63 {
            for c in q.chars() {
                match c {
                    '\n' => {
                        // print!("\\n");
                        pixel(x, y, "\\n");
                        x += 1;
                    }
                    '"' => {
                        pixel(x, y, "\\");
                        x += 1;
                        pixel(x, y, "\"");
                        x += 1;
                    }
                    '\\' => {
                        pixel(x, y, "\\");
                        x += 1;
                        pixel(x, y, "\\");
                        x += 1;
                    }
                    _ => {
                        pixel(x, y, &c.to_string());
                        x += 1;
                        // print!("{c}");
                    }
                }
            }
        } else if c == '\n' {
            y += 1;
            x = 0;
            pixel(x, y, &'\n'.to_string());
        } else {
            // print!("{c}");
            pixel(x, y, &c.to_string());
            x += 1;
        }
    }
    println!("");
}
