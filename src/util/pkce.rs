use rand::Rng;
use sha2::{Digest, Sha256};

pub(crate) struct Pkce {
    pub verifier: String,
    pub challenge: String,
}

impl Pkce {
    pub(crate) fn new() -> Self {
        let verifier = create_verifier();
        let challenge = create_challenge(&verifier);
        return Pkce {
            verifier,
            challenge,
        };
    }
}

fn create_verifier() -> String {
    let bytes = rand::thread_rng().gen::<[u8; 32]>();
    return base64::encode_config(bytes, base64::URL_SAFE_NO_PAD);
}

fn create_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier);
    let digest = hasher.finalize();
    return base64::encode_config(digest, base64::URL_SAFE_NO_PAD);
}

#[cfg(test)]
mod pkce_test {

    #[test]
    fn test_verfier() {
        assert_eq!("", "")
    }
}