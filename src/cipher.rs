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

    pub fn decrypt(&mut self, d: u8) -> u8 {
        match &mut self.encryptor {
            Some(encryptor) => {
                let mut d_buffer = vec![d];
                encryptor.decrypt(d_buffer.as_mut_slice());
                d_buffer[0]
            }
            None => d,
        }
    }

    pub fn enable(&mut self, key: &[u8]) {
        let cipher = AesCfb8::new_var(key, &[0]).unwrap();
        self.encryptor = Some(cipher);
    }

    pub fn disable(&mut self) {
        self.encryptor = None
    }
}
