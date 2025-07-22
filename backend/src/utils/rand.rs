use rand::{rngs::OsRng, TryRngCore};

use crate::utils::error::{ApiError, ApiResult};

pub fn generate_otp() -> ApiResult<String> {
    let mut bytes = [0u8; 4];
    OsRng
        .try_fill_bytes(&mut bytes)
        .map_err(|_| ApiError::internal("Can't generate random otp"))?;
    let num = u32::from_be_bytes(bytes) % 1_000_000;
    Ok(format!("{:06}", num))
}
