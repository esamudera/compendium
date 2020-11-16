use crate::schema::playbook;

#[derive(Queryable)]
pub struct Playbook {
    pub id: i64,
    pub title: String,
    pub create_time: i32,
    pub update_time: i32,
}

#[derive(Insertable)]
#[table_name="playbook"]
pub struct NewPlaybook {
    pub title: String,
    pub create_time: i32,
    pub update_time: i32,
}