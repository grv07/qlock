fn main() {
    let file_name = std::env::args().nth(1).expect("Provide a full file name");
    let mut data = std::fs::read_to_string(file_name).unwrap();

    data = data.replace('\\', "\\\\");
    data = data.replace('\n', "\\n");
    data = data.replace('"', "\\\"");
    data = data.replace("  ", "");

    print!("{data}");
}
