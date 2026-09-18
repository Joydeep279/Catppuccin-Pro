use std::{collections::HashMap, fs::File, io::{self, Read}};

const APP_NAME: &str = "Catppuccin Dark Pro";
const VERSION: f64 = 1.0;
static mut REQUEST_COUNT: u64 = 0;

#[derive(Debug, Clone)]
struct User<'a> { id: u32, name: &'a str, roles: Vec<&'a str>, active: bool }

impl<'a> User<'a> {
    fn new(id: u32, name: &'a str) -> Self { Self { id, name, roles: vec!["developer", "admin"], active: true } }
    fn describe(&self) -> String { format!("User #{}: {} [{}]", self.id, self.name, self.roles.join(", ")) }
}

fn calculate_score(base: i32, bonus: i32, multiplier: f64) -> Result<f64, String> {
    if base < 0 { return Err("base score cannot be negative".into()); }
    Ok((base + bonus) as f64 * multiplier)
}

fn main() -> io::Result<()> {
    let user = User::new(101, "Joydeep");
    let mut cache: HashMap<String, Vec<i32>> = HashMap::new();
    let scores = vec![92, 87, 76, 98, 64, 89];

    cache.insert(user.name.to_string(), scores.iter().map(|x| x * 2).collect());
    let average = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
    let status = if average >= 90.0 { "excellent" } else if average >= 60.0 { "good" } else { "needs work" };

    match calculate_score(average as i32, 10, 1.25) {
        Ok(score) => println!("{} → {:.2} ({})", user.describe(), score, status),
        Err(error) => eprintln!("Calculation failed: {error}"),
    }

    let json_like = format!(r#"{{"name":"{}","active":{},"version":{}}}"#, user.name, user.active, VERSION);
    println!("{json_like}");

    for (index, score) in scores.iter().enumerate() {
        println!("score[{index}] = {score}");
    }

    let optional: Option<&str> = Some("Rust");
    if let Some(language) = optional { println!("Language: {language}"); }

    Ok(())
}