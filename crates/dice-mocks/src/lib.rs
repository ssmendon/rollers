use rand::{CryptoRng, RngCore, rand_core};

/// An RNG that will always roll a '1'.
///
/// See the [`rng`] crate's book: https://rust-random.github.io/book/guide-test-fn-rng.html
#[derive(Clone, Debug)]
pub struct MockCryptoRng {
    data: Vec<u64>,
    index: usize,
}
// impls for Rng //
impl MockCryptoRng {
    pub fn new(seed: &[u64]) -> Self {
        assert!(seed.len() > 0);
        Self {
            data: Vec::from(seed),
            index: 0,
        }
    }
}
impl CryptoRng for MockCryptoRng {}
impl RngCore for MockCryptoRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    fn next_u64(&mut self) -> u64 {
        let r = *self.data.get(self.index).unwrap_or(&0);
        self.index = (self.index + 1) % self.data.len();
        r
    }
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dst);
    }
}
impl Default for MockCryptoRng {
    /// Always rolls a '1'!
    fn default() -> Self {
        Self {
            data: vec![5, 5, 5, 5],
            index: Default::default(),
        }
    }
}
// end impls for Rng //
