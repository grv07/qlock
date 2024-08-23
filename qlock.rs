fn p(x:usize,y:usize,v:&str){let t=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;
    let (h,m,s)=(((t/3600)+6)%12,((t/60)+30)%60,t%60);
    let hms:[usize;8]=[h/10,h%10,10,m/10,m%10,10,s/10,s%10];
    let i= x/(3+2);let dx=x%(3+2);let n:[i32;11]=[31599,19812,14479,31207,23524,29411,29679,30866,31727,31719,1040];
    if i<8 && y<5 && dx<3 && ((n[hms[i]] >> (5-y-1) * 3+dx) & 1)==1{
        print!("\x1b[31m{v}\x1b[0m");}else{print!("{v}");}}
fn main() { let q = "fn p(x:usize,y:usize,v:&str){let t=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;\nlet (h,m,s)=(((t/3600)+6)%12,((t/60)+30)%60,t%60);\nlet hms:[usize;8]=[h/10,h%10,10,m/10,m%10,10,s/10,s%10];\nlet i= x/(3+2);let dx=x%(3+2);let n:[i32;11]=[31599,19812,14479,31207,23524,29411,29679,30866,31727,31719,1040];\nif i<8 && y<5 && dx<3 && ((n[hms[i]] >> (5-y-1) * 3+dx) & 1)==1{\nprint!(\"\\x1b[31m{v}\\x1b[0m\");}else{print!(\"{v}\");}}\nfn main() { let q = \"?\";\nlet mut x = 0;let mut y =0;for c in q.chars().collect::<Vec<char>>() { if c as u8 ==63 { for c in q.chars() { match c { '\\n' => { p(x, y, \"\\\\n\"); x += 1; }\n'\"' => {p(x,y,\"\\\\\");x+=1;p(x,y,\"\\\"\");x+=1;} '\\\\' => {p(x,y,\"\\\\\");x+=1;p(x,y,\"\\\\\");x+=1;} _ => {p(x,y,&c.to_string());x+=1;}}\n} } else if c == '\\n' {y+=1;x=0;p(x,y,&'\\n'.to_string()); } else {p(x,y,&c.to_string());x+=1;}}println!(\"\");\n}\n";
    let mut x = 0;let mut y =0;for c in q.chars().collect::<Vec<char>>() { if c as u8 ==63 { for c in q.chars() { match c { '\n' => { p(x, y, "\\n"); x += 1; }
                    '"' => {p(x,y,"\\");x+=1;p(x,y,"\"");x+=1;} '\\' => {p(x,y,"\\");x+=1;p(x,y,"\\");x+=1;} _ => {p(x,y,&c.to_string());x+=1;}}
            } } else if c == '\n' {y+=1;x=0;p(x,y,&'\n'.to_string()); } else {p(x,y,&c.to_string());x+=1;}}println!("");
}
