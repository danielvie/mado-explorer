//! Fixed little-endian protocol shared by our parent and private helper.
pub const HELPER_CAP: usize = 128 * 1024 * 1024;
pub const OUTPUT_CAP: usize = 32 * 1024 * 1024;
pub const INPUT_CAP: u64 = 16 * 1024 * 1024;
pub const DIMENSION_CAP: u32 = 8192;
pub const PATH_UNITS: usize = 32766;
pub const ERROR_CAP: usize = 1024;
pub const REQUEST_SIZE: usize = 40;
pub const HEADER_SIZE: usize = 72;
pub const VERSION: u32 = 1;
pub const IO_CHUNK: usize = 16 * 1024;

pub fn put32(b: &mut [u8], at: usize, n: u32) {
    b[at..at + 4].copy_from_slice(&n.to_le_bytes());
}
pub fn put64(b: &mut [u8], at: usize, n: u64) {
    b[at..at + 8].copy_from_slice(&n.to_le_bytes());
}
pub fn get32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
pub fn get64(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

#[allow(dead_code)] // Used only on the parent side of the shared protocol.
pub fn request(target: u64, generation: u64, path: &[u16], mode: u32, aux: u64) -> Vec<u8> {
    let mut b = vec![0; REQUEST_SIZE + 2 * path.len()];
    b[..4].copy_from_slice(b"MIQ1");
    put32(&mut b, 4, VERSION);
    put64(&mut b, 8, target);
    put64(&mut b, 16, generation);
    put32(&mut b, 24, path.len() as u32);
    put32(&mut b, 28, mode);
    put64(&mut b, 32, aux);
    for (i, unit) in path.iter().enumerate() {
        b[REQUEST_SIZE + 2 * i..REQUEST_SIZE + 2 * i + 2].copy_from_slice(&unit.to_le_bytes());
    }
    b
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub target: u64,
    pub generation: u64,
    pub status: u32,
    pub format: u32,
    pub orientation: u32,
    pub encoded_width: u32,
    pub encoded_height: u32,
    pub width: u32,
    pub height: u32,
    pub error_len: u32,
    pub file_bytes: u64,
    pub payload: u64,
}
impl Header {
    #[allow(dead_code)] // Used by helper and protocol tests.
    pub fn error(target: u64, generation: u64, status: u32, length: usize) -> Self {
        Self {
            target,
            generation,
            status,
            format: 0,
            orientation: 0,
            encoded_width: 0,
            encoded_height: 0,
            width: 0,
            height: 0,
            error_len: length as u32,
            file_bytes: 0,
            payload: length as u64,
        }
    }
    #[allow(dead_code)] // Used by helper and protocol tests.
    pub fn bytes(self) -> [u8; HEADER_SIZE] {
        let mut b = [0; HEADER_SIZE];
        b[..4].copy_from_slice(b"MIR1");
        put32(&mut b, 4, VERSION);
        put64(&mut b, 8, self.target);
        put64(&mut b, 16, self.generation);
        for (i, n) in [
            self.status,
            self.format,
            self.orientation,
            self.encoded_width,
            self.encoded_height,
            self.width,
            self.height,
            self.error_len,
        ]
        .into_iter()
        .enumerate()
        {
            put32(&mut b, 24 + i * 4, n);
        }
        put64(&mut b, 56, self.file_bytes);
        put64(&mut b, 64, self.payload);
        b
    }
    #[allow(dead_code)] // Used only on the parent side.
    pub fn parse(b: &[u8; HEADER_SIZE], target: u64, generation: u64) -> Result<Self, ()> {
        if &b[..4] != b"MIR1"
            || get32(b, 4) != VERSION
            || get64(b, 8) != target
            || get64(b, 16) != generation
        {
            return Err(());
        }
        let h = Self {
            target,
            generation,
            status: get32(b, 24),
            format: get32(b, 28),
            orientation: get32(b, 32),
            encoded_width: get32(b, 36),
            encoded_height: get32(b, 40),
            width: get32(b, 44),
            height: get32(b, 48),
            error_len: get32(b, 52),
            file_bytes: get64(b, 56),
            payload: get64(b, 64),
        };
        if h.status == 0 {
            let size = rgba_size(h.width, h.height)?;
            rgba_size(h.encoded_width, h.encoded_height)?;
            let dims = if h.orientation >= 5 {
                (h.encoded_height, h.encoded_width)
            } else {
                (h.encoded_width, h.encoded_height)
            };
            if !(1..=3).contains(&h.format)
                || !(1..=8).contains(&h.orientation)
                || h.error_len != 0
                || h.payload != size as u64
                || h.file_bytes > INPUT_CAP
                || h.file_bytes == 0
                || dims != (h.width, h.height)
            {
                return Err(());
            }
        } else if !(1..=5).contains(&h.status)
            || h.error_len as usize > ERROR_CAP
            || h.payload != h.error_len as u64
            || [
                h.format,
                h.orientation,
                h.encoded_width,
                h.encoded_height,
                h.width,
                h.height,
            ]
            .iter()
            .any(|&x| x != 0)
            || h.file_bytes != 0
        {
            return Err(());
        }
        Ok(h)
    }
}
pub fn rgba_size(w: u32, h: u32) -> Result<usize, ()> {
    if w == 0 || h == 0 || w > DIMENSION_CAP || h > DIMENSION_CAP {
        return Err(());
    }
    let bytes = (w as usize)
        .checked_mul(h as usize)
        .and_then(|p| p.checked_mul(4))
        .ok_or(())?;
    if bytes > OUTPUT_CAP {
        Err(())
    } else {
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn headers_reject_forged_identity_arithmetic_and_metadata() {
        let h = Header {
            target: 1,
            generation: 2,
            status: 0,
            format: 1,
            orientation: 6,
            encoded_width: 2,
            encoded_height: 3,
            width: 3,
            height: 2,
            error_len: 0,
            file_bytes: 100,
            payload: 24,
        };
        assert_eq!(Header::parse(&h.bytes(), 1, 2), Ok(h));
        assert!(Header::parse(&h.bytes(), 1, 3).is_err());
        for at in [4, 8, 16, 24, 28, 32, 36, 40, 44, 48, 52, 56, 64] {
            let mut b = h.bytes();
            b[at..at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
            assert!(Header::parse(&b, 1, 2).is_err(), "offset {at}");
        }
        assert!(rgba_size(8192, 8192).is_err());
        assert!(rgba_size(0, 1).is_err());
        assert!(Header::parse(&Header::error(1, 2, 1, ERROR_CAP + 1).bytes(), 1, 2).is_err());
    }
}
