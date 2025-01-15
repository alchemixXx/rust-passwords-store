use serde_derive::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::custom_result::{CustomError, CustomResult};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct PasswordEntity {
    pub id: i32,
    pub password: String,
    pub username: Option<String>,
    pub service: String,
    pub login: String,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct Storage {
    path: &'static str,
}

impl Storage {
    pub fn new(path: &'static str) -> Self {
        Storage { path }
    }
    pub fn load_data(&self) -> CustomResult<Vec<PasswordEntity>> {
        let data_str = fs::read_to_string(Path::new(self.path)).map_err(|err| {
            println!("{:?}", err);
            CustomError::CommandExecution("Error reading file".to_string())
        })?;
        let data = serde_json::from_str::<Vec<PasswordEntity>>(&data_str).map_err(|err| {
            println!("{:?}", err);
            CustomError::CommandExecution("Error parsing data".to_string())
        })?;

        Ok(data)
    }

    pub fn save_data(&self, data: Vec<PasswordEntity>) -> CustomResult<()> {
        let data_str = serde_json::to_string(&data).map_err(|err| {
            println!("{:?}", err);
            CustomError::CommandExecution("Error serializing data".to_string())
        })?;
        fs::write(Path::new(self.path), data_str).map_err(|err| {
            println!("{:?}", err);
            CustomError::CommandExecution("Error writing data to file".to_string())
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dat_0_records() {
        let path = "src/data/test_1_data.json";
        let storage = Storage::new(path);
        let insert = storage.save_data(vec![]);
        assert_eq!(insert, Ok(()));
        let data = storage.load_data();
        assert_eq!(data, Ok(vec!()));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_data_1_record() {
        let path = "src/data/test_2_data.json";
        let record = PasswordEntity {
            id: 1,
            password: "12345".to_string(),
            username: Some("test".to_string()),
            service: "test".to_string(),
            login: "username_here".to_string(),
            comment: None,
        };
        let storage = Storage::new(path);
        let insert = storage.save_data(vec![record.clone()]);
        assert_eq!(insert, Ok(()));
        let data = storage.load_data();
        assert_eq!(data, Ok(vec!(record)));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_missing_file() {
        let path = "src/data/test_3_data.json";
        let storage = Storage::new(path);
        let exists = fs::exists(path).unwrap();
        assert!(!exists);
        let insert = storage.save_data(vec![]);
        assert_eq!(insert, Ok(()));
        let exists = fs::exists(path).unwrap();
        assert!(exists);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_1_record_to_file() {
        let path = "src/data/test_4_data.json";
        let storage = Storage::new(path);
        let insert = storage.save_data(vec![]);
        assert_eq!(insert, Ok(()));
        let record = PasswordEntity {
            id: 1,
            password: "12345".to_string(),
            username: Some("test".to_string()),
            service: "test".to_string(),
            login: "username_here".to_string(),
            comment: None,
        };
        let insert = storage.save_data(vec![record.clone()]);
        assert_eq!(insert, Ok(()));
        let data = storage.load_data();
        assert_eq!(data, Ok(vec!(record)));
        fs::remove_file(path).unwrap();
    }
}
