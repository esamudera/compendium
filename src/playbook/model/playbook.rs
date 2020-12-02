use crate::schema::playbook;

pub struct Playbook {
    pub id: i64,
    pub title: String,
    pub create_time: i64,
    pub update_time: i64,
}

pub struct NewPlaybook {
    pub title: String,
    pub create_time: i64,
    pub update_time: i64,
}