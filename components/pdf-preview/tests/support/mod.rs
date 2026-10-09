use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

pub struct FixtureDirectory(PathBuf);

impl FixtureDirectory {
    pub fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mado-p03-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        // No existing directory is claimed or cleaned up. Collision fails the check.
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn write(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

// Only a test fixture writer, not a PDF interpreter. All xref offsets are generated.
fn pdf(objects: &[&str], extra_trailer: &str) -> Vec<u8> {
    let mut bytes = b"%PDF-1.3\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(bytes.len());
        bytes.extend_from_slice(format!("{} 0 obj\n{object}\nendobj\n", index + 1).as_bytes());
    }
    let xref = bytes.len();
    bytes.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in offsets {
        bytes.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    bytes.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R {extra_trailer} >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    bytes
}

pub fn simple_pdf() -> Vec<u8> {
    // White page, with a red square in its center. No fonts or external resources.
    let stream = "1 0 0 rg 75 25 50 50 re f\n";
    let contents = format!("<< /Length {} >>\nstream\n{stream}endstream", stream.len());
    pdf(
        &[
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << >> /Contents 4 0 R >>",
            &contents,
        ],
        "",
    )
}

pub fn encrypted_pdf() -> Vec<u8> {
    // R2/RC4-40 parameters generated with pypdf 6.20.0, user=secret, owner=owner.
    // Fixed ID/O/U make a deterministic blank encrypted document; no crypto code here.
    let id = "6461613635663736666539376465383532613130666433313231656236363333";
    let trailer = format!("/Encrypt 4 0 R /ID [<{id}> <{id}>]");
    pdf(
        &[
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << >> >>",
            "<< /Filter /Standard /V 1 /R 2 /Length 40 /P -4 /O <92fe0f4454ad4c9644693f33c07cb54f587dce1e2682fe9ecea6107a1ef630dd> /U <c9b4a4046bedb8d07f5630089ff86342a08d1c39dccbdfdc07841faa9bc65046> >>",
        ],
        &trailer,
    )
}
