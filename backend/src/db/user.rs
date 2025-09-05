use log::error;

use crate::db::database::Database;

pub struct UserDB{
    database: Database
}
impl UserDB{
    pub fn new(database: Database)-> Self{
        UserDB{
            database
        }
    }

    pub async fn create_table(&self){
        let pool = match self.database.get_pool().await{
            Ok(p) => p,
            Err(e) => {
                error!("{}",e);
                return;
            }
        };
        
    }
}