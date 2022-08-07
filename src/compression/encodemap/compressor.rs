use super::encodemap::EncodeMap;





impl EncodeMap {
    pub fn compress(left : &u8, right : &u8) -> u8 { left.rotate_left(4)  + right }
    pub fn uncompress(decoded_bytes : &mut Vec<u8>, byte : &u8) {
            decoded_bytes.push((byte  & 0b11110000).rotate_right(4));
            decoded_bytes.push(byte  & 0b00001111);
    }
    pub fn uncompress_simple(byte : &u8) ->  (u8, u8) { ((byte  & 0b11110000).rotate_right(4), (byte  & 0b00001111)) }
     
}