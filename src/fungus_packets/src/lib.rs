pub mod out_packet;
pub mod in_headers;
pub mod out_headers;
pub mod in_packet;
pub mod packet_errors;
pub mod crypto;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
