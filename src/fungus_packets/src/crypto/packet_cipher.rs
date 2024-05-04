use std::io::Read;
use aes::{Aes128, Aes256};
use aes::cipher::{BlockEncrypt, Key, KeyInit, StreamCipherCoreWrapper};
use ofb::cipher::{KeyIvInit, StreamCipher};
use ofb::{Ofb, OfbCore};
use fungus_utils::constants::server_constants::VERSION;

const AES_KEY: [u8; 32] = [
    0x13, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00,
    0xB4, 0x00, 0x00, 0x00,
    0x1B, 0x00, 0x00, 0x00,
    0x0F, 0x00, 0x00, 0x00,
    0x33, 0x00, 0x00, 0x00,
    0x52, 0x00, 0x00, 0x00   
];

const SHUFFLE_BYTES: [u8; 256] = [
    0xEC, 0x3F, 0x77, 0xA4, 0x45, 0xD0, 0x71, 0xBF, 0xB7, 0x98, 0x20, 0xFC,
    0x4B, 0xE9, 0xB3, 0xE1, 0x5C, 0x22, 0xF7, 0x0C, 0x44, 0x1B, 0x81, 0xBD, 0x63, 0x8D, 0xD4, 0xC3,
    0xF2, 0x10, 0x19, 0xE0, 0xFB, 0xA1, 0x6E, 0x66, 0xEA, 0xAE, 0xD6, 0xCE, 0x06, 0x18, 0x4E, 0xEB,
    0x78, 0x95, 0xDB, 0xBA, 0xB6, 0x42, 0x7A, 0x2A, 0x83, 0x0B, 0x54, 0x67, 0x6D, 0xE8, 0x65, 0xE7,
    0x2F, 0x07, 0xF3, 0xAA, 0x27, 0x7B, 0x85, 0xB0, 0x26, 0xFD, 0x8B, 0xA9, 0xFA, 0xBE, 0xA8, 0xD7,
    0xCB, 0xCC, 0x92, 0xDA, 0xF9, 0x93, 0x60, 0x2D, 0xDD, 0xD2, 0xA2, 0x9B, 0x39, 0x5F, 0x82, 0x21,
    0x4C, 0x69, 0xF8, 0x31, 0x87, 0xEE, 0x8E, 0xAD, 0x8C, 0x6A, 0xBC, 0xB5, 0x6B, 0x59, 0x13, 0xF1,
    0x04, 0x00, 0xF6, 0x5A, 0x35, 0x79, 0x48, 0x8F, 0x15, 0xCD, 0x97, 0x57, 0x12, 0x3E, 0x37, 0xFF,
    0x9D, 0x4F, 0x51, 0xF5, 0xA3, 0x70, 0xBB, 0x14, 0x75, 0xC2, 0xB8, 0x72, 0xC0, 0xED, 0x7D, 0x68,
    0xC9, 0x2E, 0x0D, 0x62, 0x46, 0x17, 0x11, 0x4D, 0x6C, 0xC4, 0x7E, 0x53, 0xC1, 0x25, 0xC7, 0x9A,
    0x1C, 0x88, 0x58, 0x2C, 0x89, 0xDC, 0x02, 0x64, 0x40, 0x01, 0x5D, 0x38, 0xA5, 0xE2, 0xAF, 0x55,
    0xD5, 0xEF, 0x1A, 0x7C, 0xA7, 0x5B, 0xA6, 0x6F, 0x86, 0x9F, 0x73, 0xE6, 0x0A, 0xDE, 0x2B, 0x99,
    0x4A, 0x47, 0x9C, 0xDF, 0x09, 0x76, 0x9E, 0x30, 0x0E, 0xE4, 0xB2, 0x94, 0xA0, 0x3B, 0x34, 0x1D,
    0x28, 0x0F, 0x36, 0xE3, 0x23, 0xB4, 0x03, 0xD8, 0x90, 0xC8, 0x3C, 0xFE, 0x5E, 0x32, 0x24, 0x50,
    0x1F, 0x3A, 0x43, 0x8A, 0x96, 0x41, 0x74, 0xAC, 0x52, 0x33, 0xF0, 0xD9, 0x29, 0x80, 0xB1, 0x16,
    0xD3, 0xAB, 0x91, 0xB9, 0x84, 0x7F, 0x61, 0x1E, 0xCF, 0xC5, 0xD1, 0x56, 0x3D, 0xCA, 0xF4, 0x05,
    0xC6, 0xE5, 0x08, 0x49
];

pub struct AesCipher {
    
}

pub struct PacketCipher {
    send_cipher: StreamCipherCoreWrapper<OfbCore<Aes256>>,
    recv_cipher: StreamCipherCoreWrapper<OfbCore<Aes256>>,
    g_version: u16,
    s_version: u16,
    r_version: u16,
    pub send_iv: [u8; 16],
    pub recv_iv: [u8; 16]
}

impl PacketCipher {
    pub fn new(siv: [u8; 4], riv: [u8;4]) -> Self{
        let g_version= VERSION as u16;
        let s_version = (((0xFFFF - g_version) >> 8) & 0xFF) | (((0xFFFF - g_version) << 8) & 0xFF00);
        let r_version= ((g_version >> 8) & 0xFF) | ((g_version << 8) & 0xFF00);

        let mut new_siv = [0u8; 16];
        new_siv[..4].copy_from_slice(&siv);
        for i in 4..16 {
            new_siv[i] = siv[i%4];
        }
        let send_cipher = Ofb::<Aes256>::new_from_slices(
            &AES_KEY,
            &new_siv
        ).expect("send ded");

        let mut new_riv = [0u8; 16];
        new_riv[..4].copy_from_slice(&siv);
        for i in 4..16 {
            new_riv[i] = riv[i%4];
        }
        let recv_cipher = Ofb::<Aes256>::new_from_slices(
            &AES_KEY,
            &new_riv
        ).expect("recv ded");

        PacketCipher {
            send_cipher,
            recv_cipher,
            g_version,
            s_version,
            r_version,
            send_iv: new_siv,
            recv_iv: new_riv
        }
    }

    fn aes_crypt(&mut self, bytes: &mut [u8], iv: &[u8]) {
        let mut send_cipher = Ofb::<Aes256>::new_from_slices(
            &AES_KEY,
            &iv
        ).expect("recv ded");
        send_cipher.apply_keystream(bytes);
    }

    fn multiply_bytes(iv: &[u8], i: usize, i0: usize) -> Vec<u8> {
        let mut ret = vec![0u8; i * i0];
        for x in 0..ret.len() {
            ret[x] = iv[x % i];
        }
        ret
    }

    // delta => Data
    // gamma => iv
    pub fn crypt(&mut self, data: &mut [u8], iv: &[u8]) {
        let mut remaining: i32 = data.len() as i32;
        let mut length = 0x5B0;
        let mut start = 0;

        while remaining > 0 {
            let mut myIv = Self::multiply_bytes(iv, 4, 4);
            if remaining < length {
                length = remaining;
            }

            for e in start..(start+length) {
                if (e-start) % myIv.len() as i32 == 0 {
                    self.aes_crypt(&mut myIv, iv);
                }
                data[e as usize] ^= myIv[((e - start) % myIv.len() as i32) as usize];
            }

            // Block?
            start += length;
            remaining -= length;
            length = 0x5B4;
        }
    }

    pub fn get_header(&mut self, delta: usize, iv: &[u8;16]) -> [u8; 4] {
        let mut a: i32 = (iv[3] & 0xFF) as i32;
        a |= (((iv[2] as i32) << 8_i32) & 0xFF00);
        a = (a ^ self.s_version as i32);
        let b = (((delta << 8) & 0xFF00) | (delta >> 8)) as i32;
        let c = a ^ b;
        [
            ((a >> 8) & 0xFF) as u8,
            (a & 0xFF) as u8,
            ((c >> 8) & 0xFF) as u8,
            (c & 0xFF) as u8,
        ]
    }

    pub fn get_length(&self, delta: i32) -> usize {
        let mut a = (delta >> 16) ^ (delta & 0xFFFF);
        a = ((a << 8) & 0xFF00) | ((a >> 8) & 0xFF);
        a as usize
    }

    fn check_packet(&self, delta: &[u8], gamma: &[u8], r_version: u16) -> bool {
        ((delta[0] ^ gamma[2]) & 0xFF) == ((r_version >> 8) as u8) &&
            ((delta[1] ^ gamma[3]) & 0xFF) == (r_version as u8)
    }

    fn check_packet_int(&self, delta: u32, gamma: &[u8], r_version: u16) -> bool {
        let a = [(delta >> 24) as u8, (delta >> 16) as u8];
        self.check_packet(&a, gamma, r_version)
    }

    pub fn get_new_siv(&mut self) {
        let mut send_iv = self.send_iv.clone();
        self.get_new_iv(&mut send_iv, &SHUFFLE_BYTES);

        self.send_iv= send_iv;
    }

    pub fn get_new_riv(&mut self) {
        let mut recv_iv = self.recv_iv.clone();
        self.get_new_iv(&mut recv_iv, &SHUFFLE_BYTES);

        self.recv_iv = recv_iv;
    }

    fn get_new_iv(&self, delta: &mut [u8; 16], shuffle_bytes: &[u8]) {
        let mut n_iv: [u8; 4] = [0xF2, 0x53, 0x50, 0xC6];
        for i in 0..4 {
            let a = delta[i] as usize;
            let b = shuffle_bytes[a];
            n_iv[0] = n_iv[0].wrapping_add(shuffle_bytes[n_iv[1] as usize].wrapping_sub(a as u8));
            n_iv[1] = n_iv[1].wrapping_sub((n_iv[2] ^ b) as u8);
            n_iv[2] = n_iv[2] ^ shuffle_bytes[n_iv[3] as usize].wrapping_add(a as u8);
            n_iv[3] = n_iv[3].wrapping_sub(n_iv[0].wrapping_sub(b));
            let c = (n_iv[0] as u32)
                | ((n_iv[1] as u32) << 8)
                | ((n_iv[2] as u32) << 16)
                | ((n_iv[3] as u32) << 24);
            let d = (c << 3) | (c >> 29);
            n_iv[0] = (d & 0xFF) as u8;
            n_iv[1] = ((d >> 8) & 0xFF) as u8;
            n_iv[2] = ((d >> 16) & 0xFF) as u8;
            n_iv[3] = ((d >> 24) & 0xFF) as u8;
        }
        for i in 0..4 {
            delta[i] = n_iv[i];
        }
    }

    //
    //
    // Shanda shit now
    //
    //
    fn roll_left(&self, value: u8, shift: usize) -> u8 {
        let overflow = (value as u16) << (shift % 8); // Use a larger integer to handle the shift without overflow
        let ret = ((overflow & 0xff) | (overflow >> 8)) as u8; // Consolidate back to a byte
        ret
    }

    fn roll_right(&self, value: u8, shift: usize) -> u8 {
        let overflow = ((value as u16) << 8) >> (shift % 8); // Shift into a higher byte then rotate right
        let ret = ((overflow & 0xff) | (overflow >> 8)) as u8; // Consolidate back to a byte
        ret
    }

    pub fn encrypt_shanda(&self, data: &mut [u8]) {
        for j in 0..6 {
            let mut remember = 0u8;
            let mut data_length = data.len();
            if j % 2 == 0 {
                for i in 0..data.len() {
                    let mut cur = data[i];
                    cur = self.roll_left(cur, 3);
                    cur = cur.wrapping_add(data_length as u8);
                    cur ^= remember;
                    remember = cur;
                    cur = self.roll_right(cur, data_length);
                    cur = !cur;
                    cur = cur.wrapping_add(0x48);
                    data_length = data_length.wrapping_sub(1);
                    data[i] = cur;
                }
            } else {
                for i in (0..data.len()).rev() {
                    let mut cur = data[i];
                    cur = self.roll_left(cur, 4);
                    cur = cur.wrapping_add(data_length as u8);
                    cur ^= remember;
                    remember = cur;
                    cur ^= 0x13;
                    cur = self.roll_right(cur, 3);
                    data_length = data_length.wrapping_sub(1);
                    data[i] = cur;
                }
            }
        }
    }

    pub fn decrypt_shanda(&self, data: &mut [u8]) {
        for j in 1..=6 {
            let mut remember = 0u8;
            let mut data_length = data.len() as u8;
            let mut next_remember;
            if j % 2 == 0 {
                for i in 0..data.len() {
                    let mut cur = data[i];
                    cur = cur.wrapping_sub(0x48);
                    cur = (!cur) & 0xff;
                    cur = self.roll_left(cur, data_length as usize);
                    next_remember = cur;
                    cur ^= remember;
                    remember = next_remember;
                    cur = cur.wrapping_sub(data_length);
                    cur = self.roll_right(cur, 3);
                    data[i] = cur;
                    data_length = data_length.wrapping_sub(1);
                }
            } else {
                for i in (0..data.len()).rev() {
                    let mut cur = data[i];
                    cur = self.roll_left(cur, 3);
                    cur ^= 0x13;
                    next_remember = cur;
                    cur ^= remember;
                    remember = next_remember;
                    cur = cur.wrapping_sub(data_length);
                    cur = self.roll_right(cur, 4);
                    data[i] = cur;
                    data_length = data_length.wrapping_sub(1);
                }
            }
        }
    }

}