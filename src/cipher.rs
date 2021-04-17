use aes::Aes128;
use cfb8::cipher::{NewStreamCipher, StreamCipher};
use cfb8::Cfb8;

type AesCfb8 = Cfb8<Aes128>;

pub struct Cipher {
    encryptor: Option<AesCfb8>,
}

impl Cipher {
    pub fn new() -> Self {
        Self { encryptor: None }
    }

    pub fn decrypt(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        match &mut self.encryptor {
            Some(encryptor) => {
                encryptor.decrypt(data.as_mut_slice());
                data
            }
            None => data,
        }
    }

    pub fn enable(&mut self, key: &[u8]) {
        let cipher = AesCfb8::new_var(key, key).unwrap();
        self.encryptor = Some(cipher);
    }

    pub fn disable(&mut self) {
        self.encryptor = None
    }
}
