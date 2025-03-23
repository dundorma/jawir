use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;

fn base64_decode(data: &str) -> Vec<u8> {
    general_purpose::URL_SAFE
        .decode(data)
        .expect("Invalid base64 input")
}

fn result_builder(jwt: &str, final_result: String) -> String {
    let parts: Vec<&str> = jwt.split('.').collect();
    let jwt_header = base64_decode(parts[0]);
    let jwt_payload = base64_decode(parts[1]);

    format!(
        "Header: {}\nPayload: {}\nSecret: {}",
        String::from_utf8(jwt_header).expect("error while converting vec<u8> into string"),
        String::from_utf8(jwt_payload).expect("error while converting vec<u8> into string"),
        final_result
    )
}

fn get_validation(jwt: &str) -> Validation {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        panic!("Invalid JWT format");
    }

    let header_json = base64_decode(parts[0]);
    let header: Value = serde_json::from_slice(&header_json).expect("Invalid JSON in header");

    let alg = header["alg"].as_str().expect("Missing 'alg' in header");
    let algorithm = match alg {
        "HS256" => Algorithm::HS256,
        "HS384" => Algorithm::HS384,
        "HS512" => Algorithm::HS512,
        "RS256" => Algorithm::RS256,
        "RS384" => Algorithm::RS384,
        "RS512" => Algorithm::RS512,
        "PS256" => Algorithm::PS256,
        "PS384" => Algorithm::PS384,
        "PS512" => Algorithm::PS512,
        "ES256" => Algorithm::ES256,
        "ES384" => Algorithm::ES384,
        "EdDSA" => Algorithm::EdDSA,
        _ => panic!("Unsupported algorithm: {}", alg),
    };

    let mut validator = Validation::new(algorithm);
    validator.required_spec_claims = HashSet::new();
    validator.validate_exp = false;
    validator.validate_nbf = false;
    validator.validate_aud = false;
    validator
}

fn try_secret(jwt: &str, secret: &str, validation: &Validation) -> bool {
    decode::<Value>(jwt, &DecodingKey::from_secret(secret.as_ref()), validation).is_ok()
}

pub fn brute_force_jwt(jwt: &str, wordlist_path: &str, thread_count: usize) -> Option<String> {
    let validation = get_validation(jwt);

    // Open the wordlist and count total lines using byte-level splitting
    let file = File::open(wordlist_path).expect("Failed to open wordlist");
    let reader = BufReader::new(file);
    let total_lines = reader.split(b'\n').count();
    if total_lines == 0 {
        return None;
    }

    let lines_per_thread = (total_lines + thread_count - 1) / thread_count;
    let result = Arc::new(Mutex::new(None::<String>));
    let found = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let start_line = thread_id * lines_per_thread;
        let end_line = std::cmp::min((thread_id + 1) * lines_per_thread, total_lines);

        if start_line >= total_lines {
            continue;
        }

        let thread_result = Arc::clone(&result);
        let thread_found = Arc::clone(&found);
        let thread_wordlist_path = wordlist_path.to_owned();
        let thread_validator = validation.clone();
        let thread_jwt = jwt.to_owned();

        let handle = thread::spawn(move || {
            let file = File::open(thread_wordlist_path).expect("failed to open file in thread");
            let reader = BufReader::new(file);
            let lines = reader.split(b'\n').skip(start_line);

            for (i, line_result) in lines.enumerate() {
                if i >= (end_line - start_line) || thread_found.load(Ordering::Relaxed) {
                    break;
                }
                let line_bytes = match line_result {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        eprintln!("Skipping invalid line bytes: {:?}", err);
                        continue;
                    }
                };

                let file_line = String::from_utf8_lossy(&line_bytes).into_owned();

                if try_secret(&thread_jwt, &file_line, &thread_validator) {
                    thread_found.store(true, Ordering::Relaxed);
                    let mut result_guard = thread_result.lock().unwrap();
                    *result_guard = Some(file_line);
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }

    let final_result = Arc::try_unwrap(result)
        .expect("reference to the result still exists")
        .into_inner()
        .expect("failed to unlock the mutex");

    match final_result {
        Some(f_result) => Some(result_builder(jwt, f_result)),
        _ => None,
    }
}
