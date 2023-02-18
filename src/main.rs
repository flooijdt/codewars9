use inflector::Inflector;

fn main() {
    println!("{:?}", to_camel_case("the_stealth_warrior"));
}

fn to_camel_case(text: &str) -> String {
    let mut vecky: Vec<char> = text.chars().collect();
    let mut index = 0;
    for i in vecky.clone().iter() {
        if *i == '-' || *i == '_' {
            vecky[index + 1] = vecky[index + 1].to_ascii_uppercase();
            vecky.remove(index);
        }
        index += 1;
    }
    vecky.into_iter().collect::<String>()
}
