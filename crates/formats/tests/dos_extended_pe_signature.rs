use craft_anything_formats::dos::parse_mz;
use craft_anything_testkit::dos_mz;

/// Add a newer-format marker at the offset named by the DOS stub.
fn with_extension(marker: &[u8]) -> Vec<u8> {
    let mut stub = dos_mz();
    stub[6..8].fill(0);
    stub[24..26].copy_from_slice(&0x40_u16.to_le_bytes());
    let extension = u32::try_from(stub.len()).unwrap();
    stub[0x3c..0x40].copy_from_slice(&extension.to_le_bytes());
    stub.extend_from_slice(marker);
    stub
}

/// A two-byte prefix, a cut signature and nonzero trailing bytes are not PE markers.
#[test]
fn rejects_partial_and_mismatched_pe_signatures() {
    for marker in [b"PE".as_slice(), b"PE\0", b"PE\0X", b"PENA"] {
        assert!(parse_mz(&with_extension(marker)).unwrap().extended_header.is_none(), "{marker:?}");
    }
}

/// A newer header cannot begin inside the standard DOS header.
#[test]
fn rejects_a_signature_inside_the_dos_header() {
    let mut stub = with_extension(b"PE\0\0");
    stub[0x30..0x34].copy_from_slice(b"PE\0\0");
    stub[0x3c..0x40].copy_from_slice(&0x30_u32.to_le_bytes());
    assert!(parse_mz(&stub).unwrap().extended_header.is_none());
}

/// Signatures for the four supported extensions keep their current labels.
#[test]
fn recognizes_complete_extension_signatures() {
    for (marker, kind) in
        [(b"PE\0\0".as_slice(), "PE (32/64-bit Windows)"), (b"NE", "NE (16-bit Windows)"), (b"LE", "LE/LX (OS/2 or VxD)"), (b"LX", "LE/LX (OS/2 or VxD)")]
    {
        assert_eq!(parse_mz(&with_extension(marker)).unwrap().extended_header.as_deref(), Some(kind));
    }
}
