//https://www.codewars.com/kata/513e08acc600c94f01000001/solutions/rust
use std::string::String;
fn rgb(r: i32, g: i32, b: i32) -> String {
    let vecs = vec![r, g, b];
    let result = vecs
        .iter()
        .map(|&x| match x {
            _val if x < 0 => 0,
            _val if x > 255 => 255,
            _ => x,
        })
        .map(|x| format!("{:x}", x).to_uppercase())
        .map(|x| match x {
            s if s.chars().count() == 1 => "0".to_owned() + &s,
            _ => x,
        })
        .collect::<Vec<String>>()
        .join("");
    return result;
}
fn main() {
    rgb(0, 255, 125);
    rgb(-20, 275, 211);
}
