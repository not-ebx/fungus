use fungus_database::models::user::{get_login_user, get_user_by_username, insert_new_user};
use fungus_packets::in_packet::InPacket;
use log::info;
use fungus_packets::packet_errors::PacketError;
use crate::enums::login_type::LoginType;
use crate::packets::login_packets::on_check_password_result;

pub fn handle_check_login_auth_info(in_packet: &mut InPacket) -> Result<(), PacketError> {
    let username: String = in_packet.read_string()?.to_string();
    let password: String = in_packet.read_string()?.to_string();
    let mut machine_id = vec![16];
    in_packet.read_exact(&mut machine_id)?;

    let user_result = get_login_user(
        username.clone(),
        password.clone()
    );
    match user_result {
        Ok(user) => {
            //TODO Check user ban and stuff

        },
        Err(e) => {
            // See if user exists
            let user_exists = get_user_by_username(username.clone());
            match user_exists {
                Ok(user) => {
                    info!("Could not log user {}", e);
                },
                Err(e_2) => {
                    // Create user, autoregister
                    let auto_reg = insert_new_user(username.clone(), password.clone()).unwrap();
                    if auto_reg > 0 {
                        let user_result = get_login_user(
                            username.clone(),
                            password.clone()
                        ).unwrap();
                        on_check_password_result(
                            Option::from(user_result),
                            LoginType::Success
                        );
                    }

                }
            }
        }
    }
    Ok(())
}