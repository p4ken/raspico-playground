use core::ops::Index;

pub struct Font<'a> {
    unicode_bytes: &'a [u8],
    glyph_bytes: &'a [u8],
    frame_size: usize,
}

impl<'a> Font<'a> {
    pub const fn new(bin: &'a [u8]) -> Self {
        let (count_bin, bin) = bin.split_first_chunk().expect("count required");
        let count = u16::from_le_bytes(*count_bin) as usize;

        // let (_reserved, bin) = bin.split_at_checked(2).expect("reserved required");
        let (unicode_bytes, glyph_bytes) =
            bin.split_at_checked(count * 2).expect("unicodes required");

        let frame_size = (glyph_bytes.len() * 8 / count).isqrt();
        assert!(frame_size * frame_size * count / 8 == glyph_bytes.len());

        Self {
            unicode_bytes,
            glyph_bytes,
            frame_size,
        }
    }
}

impl Index<char> for Font<'_> {
    type Output = [u8];

    // TODO: getも提供する
    fn index(&self, unicode: char) -> &Self::Output {
        let idx = self
            .unicode_bytes
            .chunks_exact(2)
            .position(|w| u16::from_le_bytes(*w.as_array().unwrap()) == unicode as u16)
            .unwrap();
        &self
            .glyph_bytes
            .chunks_exact(self.frame_size * self.frame_size / 8)
            .nth(idx) // TODO: アドレスアクセス
            .unwrap_or(&[])
    }
}
