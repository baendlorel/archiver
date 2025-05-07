use owo_colors::OwoColorize;

pub fn handler(time_interval: Option<String>) {
    println!("归档日志");
    if time_interval.is_some() {
        println!("时间区间：{}", time_interval.unwrap().green());
    } else {
        println!("时间区间：所有");
    }
}
