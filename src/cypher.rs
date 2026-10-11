pub fn vernan_cypher(message_bytes: &[u8], keystream_bytes: &[u8]) -> Vec<u8> {
    let message_len = message_bytes.len();

    if message_len > keystream_bytes.len() {
        println!("Encryption failed: message length is too long for keystring!");
        return Vec::new();
    }

    let mut encrypted_message: Vec<u8> = Vec::with_capacity(message_len);
    for i in 0..message_len {
        let encrypted = message_bytes[i] ^ keystream_bytes[i]; // XOR
        encrypted_message.push(encrypted);
    }

    return encrypted_message;
}

pub fn make_keystream_bytes(bbs_bits: Vec<u8>) -> Vec<u8> {
    let mut keystream_bytes: Vec<u8> = Vec::with_capacity(bbs_bits.len() / 8);

    for chunk in bbs_bits.chunks(8) {
        let mut single_byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            single_byte |= bit << i;
        }

        keystream_bytes.push(single_byte);
    }

    return keystream_bytes;
}