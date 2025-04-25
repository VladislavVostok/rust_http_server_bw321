use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;


use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::{to_do_factory, ItemTypes};

pub fn get() -> impl Responder{
    let state: Map<String, Value> = read_file("C:\\Users\\Студент\\Desktop\\Владислав Александроич\\web_app\\src\\state.json");
    let mut array_buffer = Vec::new();

    for(key, value) in state{
        let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
        let item: ItemTypes = to_do_factory(&key, status);
        array_buffer.push(item);
    }

    //let return_package: ToDo
}

