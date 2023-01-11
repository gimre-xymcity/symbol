#[cfg(test)]
pub fn rand_bytes(size: usize) -> Vec<u8> {
    use rand::RngCore;

    let mut bytes = vec![0; size];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut bytes);
    bytes
}
