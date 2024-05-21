use fungus_database::daos::account_dao::AccountDAO;
use fungus_database::daos::user_dao::UserDAO;
use fungus_database::database::get_db;
use fungus_utils::constants::server_constants::ALLOW_AUTO_REGISTER;
use crate::entities::account::Account;
use crate::entities::user::User;
use crate::errors::service_errors::ServiceError;

pub struct UserService {
    user_dao: UserDAO,
    account_dao: AccountDAO
}

impl UserService {
    pub fn new() -> Self {
        UserService{
            user_dao: UserDAO,
            account_dao: AccountDAO,
        }
    }
    
    pub async fn try_login(&self, username: String, password: String) -> Result<User, ServiceError>{
        let pool = &*get_db();
        let user_query = self.user_dao.get_user_by_username(
            pool,
            username.clone()
        ).await;
        match user_query {
            Ok(_) => {
                let user_serialized = self.user_dao.get_login_user(
                    pool,
                    username.clone(),
                    password.clone(),
                ).await;

                if let Err(e) = user_serialized {
                    Err(ServiceError::InvalidDetails)
                } else {
                    let user: User = User::from(user_serialized.unwrap());
                    Ok(user)
                }
            }
            Err(e) => {
                if ALLOW_AUTO_REGISTER {
                    self.create_user(
                        username.clone(),
                        password.clone()
                    ).await?;
                    self.try_login(username.clone(), password.clone()).await
                } else {
                    return Err(ServiceError::NotFound)
                }
            }
        }
    }

    pub async fn create_user(&self, username: String, password: String) -> Result<(), ServiceError>{
        let pool = &*get_db();
        let new_user = self.user_dao.insert_new_user(pool, username, password).await?;

        Ok(())
    }
}