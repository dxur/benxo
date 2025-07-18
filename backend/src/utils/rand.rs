use rand::{rngs::OsRng, TryRngCore};

pub fn generate_otp() -> String {
    let mut bytes = [0u8; 4];
    OsRng.try_fill_bytes(&mut bytes);
    let num = u32::from_be_bytes(bytes) % 1_000_000;
    format!("{:06}", num)
}
