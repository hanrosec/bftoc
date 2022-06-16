use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let in_file = std::fs::read(args[1].clone()).unwrap();
    let mut out_file = std::fs::File::create(args[2].clone()).unwrap();
    let data: String = del_unallowed_chars(in_file.iter().map(|&x| x as char).collect());
    
    let program = format!(r#"#include <stdio.h>
int main() {{
char memory[3000] = {{0}};
char *p = memory;
{}
return 0;
}};"#, bf_to_c(data));

    out_file.write_all(program.as_bytes()).unwrap();

}

fn del_unallowed_chars(data: String) -> String {
    let mut out = String::new();
    for c in data.chars() {
        if [".", ",", "+", "-", "<", ">", "[", "]"].contains(&c.to_string().as_str()) {
            out.push(c);
        }
    }
    out
}

fn bf_to_c(data: String) -> String {
    let mut out = String::new();
    for chr in data.chars() {
        match chr {
            '>' => out.extend("++p;\n".chars()),
            '<' => out.extend("--p;\n".chars()),
            '+' => out.extend("++(*p);\n".chars()),
            '-' => out.extend("--(*p);\n".chars()),
            '.' => out.extend("putchar(*p);\n".chars()),
            ',' => out.extend("*p = getchar();\n".chars()),
            '[' => out.extend("while(*p) {\n".chars()),
            ']' => out.extend("};\n".chars()),
            _ => {}
        }
    }
    out
}