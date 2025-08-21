pub trait Storage {
    fn save_block(&self, slot: u64, blockhash: &str);
    fn save_tx(&self, signature: &str, slot: u64);
}
