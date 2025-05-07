use owo_colors::OwoColorize;

pub fn handler(name: Option<String>) {
    if name.is_some() {
        println!("归档列表：{}", name.unwrap().green());
    } else {
        println!("归档列表：所有");
    }
}
