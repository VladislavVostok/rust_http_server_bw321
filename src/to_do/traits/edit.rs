use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;
use crate::to_do::enums::TaskStatus;

pub trait Edit{
    fn set_to_done(&self, title: &String, state: &mut Map<String,Value>){
        state.insert(title.to_string(), json!(TaskStatus::DONE.stringify()));
        write_to_file("C:\\Users\\Студент\\Desktop\\Владислав Александроич\\web_app\\src\\state.json", state);
        println!("\n\n{} выполнена\n\n",title);
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String,Value>){
        state.insert(title.to_string(), json!(TaskStatus::PENDING.stringify()));
        write_to_file("C:\\Users\\Студент\\Desktop\\Владислав Александроич\\web_app\\src\\state.json", state);
        println!("\n\n{} в процессе\n\n",title);
    }
}