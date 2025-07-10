use crate::NetConfig;

impl NetConfig {
    pub fn populate_secret_key(&mut self) {
        if self.secret_key.is_none() {
            self.secret_key = Some(iroh::SecretKey::generate(rand::rngs::OsRng));
        }
    }
}
