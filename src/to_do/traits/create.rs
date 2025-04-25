use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;

pub trait Create{
    fn create(&self, title: &String, status: &String, state: &mut Map<String,Value>){
        state.insert(title.to_string(), json!(status));
        write_to_file("C:\\Users\\Студент\\Desktop\\Владислав Александроич\\web_app\\src\\state.json", state);
        println!("\n\n{} был создан\n\n",title);
    }
}