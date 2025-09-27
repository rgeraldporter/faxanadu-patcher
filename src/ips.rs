use std::fs::File;
use std::io::{Result, Write};

pub fn write_ips(patch_file: &str, edits: &[(usize, Vec<u8>)]) -> Result<()> {
    let mut f = File::create(patch_file)?;
    f.write_all(b"PATCH")?;

    for (offset, bytes) in edits {
        let offset_bytes = [
            ((*offset >> 16) & 0xFF) as u8,
            ((*offset >> 8) & 0xFF) as u8,
            (*offset & 0xFF) as u8,
        ];
        f.write_all(&offset_bytes)?;
        let size = bytes.len();
        let size_bytes = [(size >> 8) as u8, (size & 0xFF) as u8];
        f.write_all(&size_bytes)?;
        f.write_all(bytes)?;
    }

    f.write_all(b"EOF")?;
    Ok(())
}
