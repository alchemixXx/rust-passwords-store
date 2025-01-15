use uuid::Uuid;

pub struct PasswordEntity {
    pub id: Uuid,
    pub password: String,
    pub username: String,
    pub service: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Storage {
    path: String,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            path: String::from("./data/data.json"),
        }
    }
    pub fn load_data() -> Vec<PasswordEntity> {
        vec![]
    }

    pub fn save_data(data: Vec<PasswordEntity>) {}
}
