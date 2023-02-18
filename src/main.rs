use inflector::Inflector;

fn main() {
    println!("{:?}", to_camel_case("the_stealth_warrior"));
}

fn to_camel_case(text: &str) -> String {
    let mut vecky: Vec<char> = text.chars().collect();
    let mut to_remove: Vec<usize> = Vec::new();
    for i in vecky.clone().iter_mut().enumerate() {
        if i.1 == &'-' || i.1 == &'_' {
            vecky[i.0 + 1] = vecky[i.0 + 1].to_ascii_uppercase();
            to_remove.push(i.0);
        }
    }
    let mut counter = 0;
    for i in to_remove {
        vecky.remove(i - counter);
        counter += 1;
    }
    vecky.into_iter().collect::<String>()
}
