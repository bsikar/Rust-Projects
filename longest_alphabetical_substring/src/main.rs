fn main() {
    let e1 = "azcbobobegghakl";
    let e2 = "abcbcd";

    println!("{}\n{}", p3(e1), p3(e2));
}

fn p3(s: &str) -> String {
    let mut s = s.chars();
    let mut current: String = s.nth(0).unwrap().to_string();
    let mut longest = String::from(&current);

    for c in s {
        if c >= current.chars().last().unwrap() {
            current.push(c);
        } else {
            if current.len() > longest.len() {
                longest = (*current).to_string();
            }
            current = c.to_string();
        }
    }

    longest
}
