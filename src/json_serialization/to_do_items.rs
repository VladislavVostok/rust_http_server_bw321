use crate::to_do::structs::base::Base;
use serde_json::Serialize;

#[derive(Selialize)]
pub struct ToDoItems{
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_cout: i8,
    pub done_item_count: i8,

}