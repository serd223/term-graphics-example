use std::io::Write;

use base64::{Engine, prelude::BASE64_STANDARD};

pub struct KittyState {
    b64_buffer: String,
    swap_buffers: bool,
    cols: u16,
    rows: u16,
}

impl KittyState {
    const CHUNK_SIZE: usize = 4096;

    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            b64_buffer: String::new(),
            swap_buffers: false,
            cols,
            rows,
        }
    }

    pub fn cleanup(&mut self, mut writer: impl Write) -> std::io::Result<()> {
        write!(writer, "\x1b_Ga=d\x1b\\")
    }

    pub fn send_frame(
        &mut self,
        mut writer: impl Write,
        buf: &[u8],
        width: usize,
        height: usize,
    ) -> std::io::Result<()> {
        let front_buffer_id = if self.swap_buffers { 2 } else { 1 };
        let back_buffer_id = if self.swap_buffers { 1 } else { 2 };

        self.b64_buffer.clear();
        BASE64_STANDARD.encode_string(&buf, &mut self.b64_buffer);
        let b64 = self.b64_buffer.as_bytes();
        writer.write_all(b"\x1b[H")?;

        let chunks = b64.chunks(Self::CHUNK_SIZE);
        let total_chunks = chunks.len();

        for (idx, chunk) in chunks.enumerate() {
            let is_last = idx == total_chunks - 1;
            let m = if is_last { 0 } else { 1 };

            if idx == 0 {
                write!(
                    writer,
                    // q=2 instructs kitty to be quiet and not respond
                    "\x1b_Ga=T,f=32,s={width},v={height},c={},r={},i={front_buffer_id},C=1,m={m},q=2;",
                    self.cols, self.rows
                )?;
            } else {
                write!(writer, "\x1b_Gm={m},q=2;")?;
            }
            writer.write_all(chunk)?;
            writer.write_all(b"\x1b\\")?;
        }

        write!(writer, "\x1b_Ga=d,d=i,i={back_buffer_id},q=2\x1b\\")?;

        writer.flush()?;
        self.swap_buffers = !self.swap_buffers;
        Ok(())
    }
}
