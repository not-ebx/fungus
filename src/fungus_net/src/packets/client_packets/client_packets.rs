use log::error;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_headers::OutHeader;
use fungus_packet_utils::packet_errors::PacketError;
use crate::session::client_session::ClientSession;

pub async fn handle_client_error(session: &mut ClientSession, in_packet: &mut InPacket) -> Result<(), PacketError>{
    session.close().await;
    if in_packet.length() < 8 {
        error!("Error: {}", in_packet);
    }

    let err_type = in_packet.read_short()?;
    let mut str_type = String::from("Unknown");
    if err_type == 0x01 {
        str_type = "SendBackupPacket".to_string();
    } else if err_type == 0x02 {
        str_type = "Crash Report".to_string();
    } else if err_type == 0x03 {
        str_type = "Exception".to_string();
    }

    let client_err_type = in_packet.read_int()?;
    let data_length = in_packet.read_short()?;
    let _ = in_packet.read_int()?;
    let op = in_packet.read_short()?;

    let as_op = OutHeader::from(op);
    error!("[ERROR {}] ({}) Data: {}", str_type, as_op, in_packet);

    Ok(())
}