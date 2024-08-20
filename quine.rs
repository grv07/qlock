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
}
