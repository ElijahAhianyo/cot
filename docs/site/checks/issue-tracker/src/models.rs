use cot::db::{Auto, model};

#[derive(Debug)]
#[model]
pub struct Issue {
    #[model(primary_key)]
    pub id: Auto<i64>,
    pub title: String,
    pub description: String,
}
