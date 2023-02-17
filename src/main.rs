fn main() {
    println!("Hello, world!");
    to_camel_case("ave_maria");
}

fn to_camel_case(text: &str) -> String {
    text.to_string();
    let mut counter = 0;

    text.split(&['_', '-']);

    for i in text.chars() {
        if i == '-' || i == '_' {
            text.split(i, "");
        }
        counter += 1;
    }
}
