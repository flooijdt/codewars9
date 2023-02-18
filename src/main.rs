use inflector::Inflector;

fn main() {
    println!("{:?}", to_camel_case("the_stealth_warrior"));
}

fn to_camel_case(text: &str) -> String {
    let mut vecky: Vec<char> = text.chars().collect();
    let mut index = 0;
    for i in vecky.clone().iter_mut().enumerate() {
        if i.1 == &'-' || i.1 == &'_' {
            vecky[i.0 + 1] = vecky[i.0 + 1].to_ascii_uppercase();
            vecky.remove(i.0);
            i.0 -= 1;
        }
    }
    vecky.into_iter().collect::<String>()
}
