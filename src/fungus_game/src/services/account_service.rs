use sqlx::{Acquire, Error};
use fungus_database::daos::account_dao::AccountDAO;
use fungus_database::database::get_db;
use crate::entities::account::Account;
use crate::errors::service_errors::ServiceError;

pub struct AccountService {
    account_dao: AccountDAO
}

impl AccountService {
    pub fn new() -> AccountService {
        AccountService{
            account_dao: AccountDAO,
        }
    }
    pub async fn get_account(&self, user_id: i32, world_id: i16) -> Result<Account, ServiceError> {
        let pool = &*get_db();
        let acc_res = self.account_dao.get_user_account(
            pool,
            user_id,
            world_id
        ).await;

        match acc_res {
            Ok(acc) => {
                Ok(Account::from(acc))
            },
            Err(sqlx::Error::RowNotFound) => {
                // Create a new account, if it doesn't exists.
                let mut tx = pool.begin().await?;
                self.account_dao.create_account(&mut tx, user_id, world_id).await?;
                self.get_account(user_id, world_id)
            },
            Err(_) => Err(ServiceError::InvalidDetails)
        }
    }

    pub async fn create_account(&self, user_id: i32, world_id: 16) -> Result<(), ServiceError> {

    }

}