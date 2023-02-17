use inflector::Inflector;

fn main() {
    println!("Hello, world!");
    to_camel_case("ave_maria");
}

fn to_camel_case(text: &str) -> String {
    text.to_string();
    let mut counter = 0;

    let mut vecky: Vec<&str> = text.split(&['_', '-']).collect();
    for i in &vecky {
        println!("{:?}", i.to_camel_case());
    }

    println!("{:?}", vecky);

    // for i in text.chars() {
    //     if i == '-' || i == '_' {
    //         text.split(i, "");
    //     }
    //     counter += 1;
    // }
    "fim".to_string()
}
