use serde_json::{Map, Value};

pub trait Get{
    fn get(&self, title: &String, state: &mut Map<String,Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => println!("\n\nЭлемент: {} Статус: {}", title, result),
            None => println!("\n\nЭлемент: {} отсутствует\n\n",title)
        }

    }
}