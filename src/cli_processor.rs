use crate::{
    cli::{Cli, Commands},
    custom_result::CustomResult,
    encrypt::Encrypt,
    storage::{PasswordEntity, Storage},
};

pub struct CliProcessor {
    storage: Storage,
    encrypt: Encrypt,
}

impl CliProcessor {
    pub fn new(secret_key: String, init_vector: String, path: String) -> CliProcessor {
        let storage = Storage::new(path);
        let encrypt = Encrypt::new(secret_key, init_vector);
        CliProcessor { storage, encrypt }
    }
}

impl CliProcessor {
    pub fn process(&self, cli: Cli) -> CustomResult<()> {
        // You can check for the existence of subcommands, and if found use their
        // matches just as you would the top level cmd
        match &cli.command {
            Some(Commands::Test { list }) => {
                if *list {
                    println!("Printing testing lists...");
                } else {
                    println!("Not printing testing lists...");
                }
            }
            Some(Commands::Add {
                service,
                password,
                login,
                comment,
                username,
            }) => {
                self.add_item(service, login, password, comment, username)?;
            }
            Some(Commands::Get { service }) => {
                self.get_item(service)?;
            }
            Some(Commands::GetAll {}) => {
                self.get_all_items()?;
            }
            Some(Commands::Remove { service }) => {
                self.remove_item(service)?;
            }
            Some(Commands::Generate {}) => {
                self.generate_password()?;
            }
            Some(Commands::Update {
                service,
                password,
                login,
                comment,
                username,
            }) => {
                self.update_item(service, login, password, comment, username)?;
            }
            None => {
                println!("Nothing to execute...");
            }
        }

        Ok(())
    }
    pub fn add_item(
        &self,
        service: &String,
        login: &String,
        password: &String,
        comment: &Option<String>,
        username: &Option<String>,
    ) -> CustomResult<()> {
        println!(
            "Adding item: service: {}, login: {}, password: {}, comment: {:?}, username: {:?}",
            service, login, password, comment, username
        );

        if !service.is_empty() && !password.is_empty() && !login.is_empty() {
            let new_pass = self.encrypt.encrypt(password.as_str())?;
            let mut records = self.storage.load_data()?;
            let new_record = PasswordEntity {
                id: records.len() as i32 + 1,
                service: service.clone(),
                login: login.clone(),
                password: new_pass,
                comment: comment.clone(),
                username: username.clone(),
            };
            records.push(new_record);
            self.storage.save_data(records)?;
        } else {
            println!("Not adding new password...");
        }

        Ok(())
    }

    pub fn get_item(&self, service: &String) -> CustomResult<()> {
        println!("Getting item: service: {}", service);

        if !service.is_empty() {
            let records = self.storage.load_data()?;
            let record = records.iter().find(|r| r.service == *service);
            println!("{:?}", record);
            if let Some(record) = record {
                let decrypted_pass = self.encrypt.decrypt(&record.password)?;
                println!("Password for {} is: {}", record.service, decrypted_pass);
            } else {
                println!("No service found: {}", service);
            }
        } else {
            println!("Nothing to get...");
        }

        Ok(())
    }

    pub fn get_all_items(&self) -> CustomResult<()> {
        println!("Getting all items");

        let records = self.storage.load_data()?;
        for record in records {
            let decrypted_pass = self.encrypt.decrypt(&record.password)?;
            println!("{:#?}", record);
            println!("Password for '{}' is: '{}'", record.service, decrypted_pass);
        }

        Ok(())
    }

    pub fn remove_item(&self, service: &String) -> CustomResult<()> {
        println!("Removing item: service: {}", service);

        if !service.is_empty() {
            let records = self.storage.load_data()?;
            let target_record = records.iter().find(|r| r.service == *service);

            if let Some(record) = target_record {
                let records_filtered = records
                    .iter()
                    .filter(|r| r.id != record.id)
                    .cloned()
                    .collect();
                self.storage.save_data(records_filtered)?;
                println!("Record '{:#?}' removed...", record);
            } else {
                println!("Service '{}' not found...", service);
            }
        } else {
            println!("Nothing to remove...");
        }

        Ok(())
    }

    pub fn update_item(
        &self,
        service: &String,
        login: &Option<String>,
        password: &Option<String>,
        comment: &Option<String>,
        username: &Option<String>,
    ) -> CustomResult<()> {
        println!(
            "Updating item: service: {}, login: {:?}, password: {:?}, comment: {:?}, username: {:?}",
            service, login, password, comment, username
        );

        if !service.is_empty() {
            let records = self.storage.load_data()?;
            let target_record = records.iter().find(|r| r.service == *service);

            if let Some(record) = target_record {
                let mut new_record = record.clone();
                if let Some(pass) = password {
                    new_record.password = self.encrypt.encrypt(pass)?;
                }
                if let Some(log) = login {
                    new_record.login = log.clone();
                }
                if let Some(comm) = comment {
                    new_record.comment = Some(comm.clone());
                }
                if let Some(name) = username {
                    new_record.username = Some(name.clone());
                }
                let mut records_filtered: Vec<PasswordEntity> = records
                    .iter()
                    .filter(|r| r.id != record.id)
                    .cloned()
                    .collect();
                println!("Record '{:#?}' updated...", new_record);
                records_filtered.push(new_record);
                self.storage.save_data(records_filtered)?;
            } else {
                println!("Service '{}' not found...", service);
            }
        } else {
            println!("Nothing to update...");
        }

        Ok(())
    }

    pub fn generate_password(&self) -> CustomResult<()> {
        println!("Generating password");

        Ok(())
    }
}
