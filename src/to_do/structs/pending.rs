use crate::to_do::enums::TaskStatus;
use crate::to_do::traits::create::Create;
use super::base::Base;

use super::super::traits::get::Get;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;



pub struct Pending{
    pub super_struct: Base
}

impl Pending{
    pub fn new(input_title: &str) -> Self{
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending{super_struct: base}
    }
}

impl Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
impl Create for Pending {}