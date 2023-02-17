use inflector::Inflector;

fn main() {
    println!("Hello, world!");
    to_camel_case("ave_maria");
}

fn to_camel_case(text: &str) -> String {
    text.to_string();
    let mut counter = 0;

    let mut vecky: String = text.split(&['_', '-']).into_iter().as_str().collect();
    vecky = vecky.to_title_case();

    println!("{:?}", vecky);

    // for i in text.chars() {
    //     if i == '-' || i == '_' {
    //         text.split(i, "");
    //     }
    //     counter += 1;
    // }
    "fim".to_string()
}
