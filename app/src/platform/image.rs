//! WIC objects stay within a COM-initialized worker call; only owned pixels escape.
use std::path::Path;
pub const PIXEL_LIMIT: usize = 64 * 1024 * 1024;
pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}
fn allocation(width: u32, height: u32) -> Result<(u32, usize), String> {
    if width == 0 || height == 0 {
        return Err("Image has empty dimensions".into());
    }
    let stride = width.checked_mul(4).ok_or("Image stride overflow")?;
    let size = (stride as usize)
        .checked_mul(height as usize)
        .ok_or("Image size overflow")?;
    if size > PIXEL_LIMIT {
        return Err("Decoded image exceeds 64 MiB".into());
    }
    Ok((stride, size))
}
#[cfg(windows)]
pub fn decode(path: &Path) -> Result<DecodedImage, String> {
    use std::os::windows::ffi::OsStrExt;
    use windows::{
        Win32::{Foundation::GENERIC_READ, Graphics::Imaging::*, System::Com::*},
        core::PCWSTR,
    };
    struct Apartment;
    impl Drop for Apartment {
        fn drop(&mut self) {
            unsafe { CoUninitialize() };
        }
    }
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok() }.map_err(|e| e.to_string())?;
    let _apartment = Apartment;
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    if wide[..wide.len() - 1].contains(&0) {
        return Err("Path contains a NUL".into());
    }
    let run = || -> windows::core::Result<DecodedImage> {
        unsafe {
            let factory: IWICImagingFactory =
                CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?;
            let decoder = factory.CreateDecoderFromFilename(
                PCWSTR(wide.as_ptr()),
                None,
                GENERIC_READ,
                WICDecodeMetadataCacheOnDemand,
            )?;
            let frame = decoder.GetFrame(0)?;
            let (mut width, mut height) = (0, 0);
            frame.GetSize(&mut width, &mut height)?;
            let (stride, size) = allocation(width, height).map_err(|message| {
                windows::core::Error::new(windows::Win32::Foundation::E_INVALIDARG, message)
            })?;
            let converter = factory.CreateFormatConverter()?;
            converter.Initialize(
                &frame,
                &GUID_WICPixelFormat32bppPRGBA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeCustom,
            )?;
            let mut pixels = vec![0; size];
            converter.CopyPixels(std::ptr::null(), stride, &mut pixels)?;
            Ok(DecodedImage {
                width,
                height,
                pixels,
            })
        }
    };
    run().map_err(|e| format!("Image decode failed: {e}"))
}
#[cfg(not(windows))]
pub fn decode(_path: &Path) -> Result<DecodedImage, String> {
    Err("WIC previews require Windows".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn image_decode_bounds() {
        assert_eq!(allocation(2, 3).unwrap(), (8, 24));
        assert!(allocation(0, 1).is_err());
        assert!(allocation(u32::MAX, 2).is_err());
        assert!(allocation(8192, 8192).is_err());
        assert!(allocation(4096, 4096).is_ok());
    }
    #[test]
    #[ignore = "Native WIC fixture test; run on Windows"]
    fn native_image_decode_corrupt_cleanup() {
        let path =
            std::env::temp_dir().join(format!("wayfarer-wic-corrupt-{}.png", std::process::id()));
        std::fs::write(&path, b"not an image").unwrap();
        for _ in 0..3 {
            assert!(decode(&path).is_err());
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[cfg(all(test, windows))]
mod native_fixture_tests {
    use super::*;
    #[test]
    #[ignore = "Native WIC fixture test; run on Windows"]
    fn native_image_decode_owned_bmp() {
        // Independent 1x1 24-bit BMP, red pixel with row padding.
        let mut bmp = vec![0u8; 58];
        bmp[0..2].copy_from_slice(b"BM");
        bmp[2..6].copy_from_slice(&58u32.to_le_bytes());
        bmp[10..14].copy_from_slice(&54u32.to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&1u32.to_le_bytes());
        bmp[22..26].copy_from_slice(&1u32.to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&24u16.to_le_bytes());
        bmp[34..38].copy_from_slice(&4u32.to_le_bytes());
        bmp[56] = 255;
        let path =
            std::env::temp_dir().join(format!("wayfarer-wic-owned-{}.bmp", std::process::id()));
        std::fs::write(&path, bmp).unwrap();
        let result = decode(&path);
        std::fs::remove_file(&path).unwrap();
        let image = result.unwrap();
        assert_eq!((image.width, image.height), (1, 1));
        assert_eq!(image.pixels, [255, 0, 0, 255]);
    }
}

#[cfg(all(test, windows))]
mod alpha_fixture_tests {
    use super::*;
    #[test]
    #[ignore = "Native WIC fixture test; run on Windows"]
    fn native_image_decode_premultiplied_alpha_png() {
        // Independent PNG: one red pixel, alpha 128; uncompressed zlib payload.
        let png: &[u8] = &[
            137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1,
            8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 16, 73, 68, 65, 84, 120, 1, 1, 5, 0, 250,
            255, 0, 255, 0, 0, 128, 4, 129, 1, 128, 25, 47, 133, 176, 0, 0, 0, 0, 73, 69, 78, 68,
            174, 66, 96, 130,
        ];
        let path =
            std::env::temp_dir().join(format!("wayfarer-wic-alpha-{}.png", std::process::id()));
        std::fs::write(&path, png).unwrap();
        let result = decode(&path);
        std::fs::remove_file(&path).unwrap();
        let image = result.unwrap();
        assert_eq!((image.width, image.height), (1, 1));
        assert_eq!(image.pixels, [128, 0, 0, 128]);
    }
}
