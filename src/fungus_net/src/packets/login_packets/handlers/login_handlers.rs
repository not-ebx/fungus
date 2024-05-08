use crate::session::client_session::ClientSession;
use fungus_database::models::user::User;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::packet_errors::PacketError;
use fungus_utils::constants::server_constants::ALLOW_AUTO_REGISTER;
use fungus_utils::enums::login_type::LoginType;
use std::io::Read;
use crate::packets::login_packets::login_packets::on_check_password_result;

pub async fn handle_check_login_auth_info(
    session: &mut ClientSession,
    in_packet: &mut InPacket,
) -> Result<OutPacket, PacketError> {
    let username: String = in_packet.read_string()?.to_string();
    let password: String = in_packet.read_string()?.to_string();
    let mut machine_id = vec![16];
    in_packet.read_exact(&mut machine_id)?;

    /*let user_result = get_login_user(
        username.clone(),
        password.clone()
    );*/
    let user_result = User::get_user_by_username(username.clone());

    let packet = match user_result {
        Ok(user) => {
            // TODO - Check for bans and stuff.
            if user.check_password(password) {
                on_check_password_result(Option::from(user), LoginType::Success)
            } else {
                on_check_password_result(Option::from(user), LoginType::IncorrectPassword)
            }
        }
        _ => {
            let mut new_user: Option<User> = None;
            if ALLOW_AUTO_REGISTER {
                new_user = Some(User::insert_new_user(username.clone(), password.clone()).unwrap());
                on_check_password_result(new_user, LoginType::Success)
            } else {
                on_check_password_result(new_user, LoginType::NotRegistered)
            }
        }
    };

    Ok(packet)
}
