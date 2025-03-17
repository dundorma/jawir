use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::Value;

fn base64_decode(data: &str) -> Vec<u8> {
    general_purpose::URL_SAFE
        .decode(data)
        .expect("Invalid base64 input")
}

fn decode_input(input: &str, is_base64: bool) -> String {
    if is_base64 {
        String::from_utf8(base64_decode(input)).expect("Invalid UTF-8 after Base64 decode")
    } else {
        input.to_string()
    }
}

pub fn create_jwt(header: &str, payload: &str, secret: &str, is_base64: bool) -> String {
    let header_json = decode_input(header, is_base64);
    let payload_json = decode_input(payload, is_base64);

    let claims: Value = serde_json::from_str(&payload_json).expect("Invalid payload JSON");
    let header: Header = serde_json::from_str(&header_json).unwrap_or_default();

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("JWT creation failed")
}
