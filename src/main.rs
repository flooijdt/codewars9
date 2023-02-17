fn main() {
    println!("Hello, world!");
    to_camel_case("ave_maria");
}

fn to_camel_case(text: &str) -> String {
    println!("{:?}", &text.chars());
    text.to_string()
}
