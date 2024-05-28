pub mod entities;
pub mod game_data;
pub mod errors;
pub mod services;


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::services::account_service::AccountService;
    use crate::services::character_service::CharacterService;
    use crate::services::game_data_service::GameDataService;
    use crate::services::user_service::UserService;
    use super::*;

    #[tokio::test]
    async fn test_create_character() {
        let game_service = GameDataService::new();
        let char_service = CharacterService::new();
        let user_service = UserService::new();
        let account_service = AccountService::new();

        let user = user_service.try_login(
            String::from("admin"),
            String::from("admin"),
        ).await.unwrap();

        let account = account_service.get_account(
            user.id,
            0
        ).await.unwrap();

        let character = char_service.create_character(
            Arc::from(game_service),
            account.id,
            "testuser",
            0,
            0,
            0,
            0,
            10000,
            10000,
            vec![1012011]
        ).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_characters_for_character_select() {
        let character_service = CharacterService::new();
        let chars = character_service.get_characters_for_selection(
            1,
        ).await;
    }
}