use crate::schema::playbook;

#[derive(Queryable)]
pub struct Playbook {
    pub id: i64,
    pub title: String,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Insertable)]
#[table_name="playbook"]
pub struct NewPlaybook {
    pub title: String,
    pub create_time: i64,
    pub update_time: i64,
}