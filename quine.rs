const NUMBER_COUNT: usize = 11;
const NUMBER: [i32; NUMBER_COUNT] = [
    31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,
];

const ROW_COUNT: usize = 5;
const COL_COUNT: usize = 3;

fn main() {
    interpret_quine();
}

fn interpret_quine() {
    let q = "const NUMBER_COUNT: usize = 11;\nconst NUMBER: [i32; NUMBER_COUNT] = [\n31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,\n];\n\nconst ROW_COUNT: usize = 5;\nconst COL_COUNT: usize = 3;\n\nfn main() {\ninterpret_quine();\n}\n\nfn interpret_quine() {\nlet q = \"?\";\n\nfor c in q.chars().collect::<Vec<char>>() {\nif c as u8 == 63 {\nfor c in q.chars() {\nmatch c {\n'\\n' => print!(\"\\\\n\"),\n'\"' => print!(\"\\\\\\\"\"),\n'\\\\' => print!(\"\\\\\\\\\"),\nc => print!(\"{c}\"),\n}\ncontinue;\n}\n} else {\nprint!(\"{c}\");\n}\n}\n\nprintln!(\"\");\n}\n\nfn _print_number(number: usize) {\nfor y in 0..ROW_COUNT {\nfor x in 0..COL_COUNT {\nlet c = (ROW_COUNT - y - 1) * COL_COUNT + x;\nif ((NUMBER[number] >> c) & 1) == 1 {\nprint!(\"*\");\n} else {\nprint!(\" \");\n}\n}\nprintln!(\"\");\n}\nprintln!(\"\");\n}\n";

    for c in q.chars().collect::<Vec<char>>() {
        if c as u8 == 63 {
            for c in q.chars() {
                if c == '\n' {
                    print!("\\n");
                } else if c == '"' {
                    print!("\\\"");
                } else if c == '\\' {
                    print!("\\\\");
                } else {
                    print!("{c}");
                }
            }
        } else {
            print!("{c}");
        }
    }
    println!("");
}

fn _print_number(number: usize) {
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
