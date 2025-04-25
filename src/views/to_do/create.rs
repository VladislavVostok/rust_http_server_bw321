use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;
use crate::views::to_do::to_do_views_factory;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String{

    let state: Map<String, Value> = read_file("C:\\Users\\Студент\\Desktop\\Владислав Александроич\\web_app\\src\\state.json");
    let title: String = req.match_info().get("title").unwrap().to_string();

    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);

    format!("{} создан", title)
}