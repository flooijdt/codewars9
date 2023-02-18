use inflector::Inflector;

fn main() {
    println!("{:?}", to_camel_case("Ave_maria"));
}

fn to_camel_case(text: &str) -> String {
    let mut vecky: Vec<char> = text.chars().collect();
    for i in vecky.iter() {
        if *i == '-' || *i == '_' {
            vecky.remove(vecky.get(*i).unwrap());
            // text.split(*i).collect::<Vec<&str>>();
            println!("{:?}", vecky);
        }
    }
    // text.to_camel_case().to_string()
    // text.to_string();
    // let mut counter = 0;

    // let mut vecky: Vec<&str> = text.split(&['_', '-']).collect();
    // for i in &vecky {
    //     println!("{:?}", i.to_camel_case());
    // }

    // println!("{:?}", vecky);

    // for i in text.chars() {
    // if i == '-' || i == '_' {
    //     text.split(i, "");
    // }
    //     counter += 1;
    // }
    "fim".to_string()
}
