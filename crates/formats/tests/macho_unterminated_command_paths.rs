use craft_anything_formats::{
    binary::{Arch, Binary},
    macho, macho_deps,
};
use craft_anything_testkit::apple;

/// Remove the terminator and its padding from a load command with a path at the given offset.
fn unterminated(mut command: Vec<u8>, offset: usize) -> Vec<u8> {
    let end = offset + command[offset..].iter().position(|&byte| byte == 0).unwrap();
    command[end..].fill(b'x');
    command
}

/// Both views ignore malformed path bytes and still read the following valid commands.
#[test]
fn malformed_command_paths_do_not_become_dependencies() {
    let data = apple::macho_min(
        apple::CPU_X86_64,
        2,
        &[
            unterminated(apple::lc_dylib(0xc, "/bad.dylib"), 24),
            unterminated(apple::lc_rpath("/bad"), 12),
            apple::lc_dylib(0xc, "/good.dylib"),
            apple::lc_rpath("/good"),
        ],
        &[],
    );
    let deps = macho_deps::parse_thin(&data).unwrap();
    assert_eq!(deps.dylibs.iter().map(|dep| dep.name.as_str()).collect::<Vec<_>>(), ["/good.dylib"]);
    assert_eq!(deps.rpaths, ["/good"]);
    let mut binary = Binary::raw(Arch::X86_64, 0, 0, 0);
    macho::enrich(&data, &mut binary);
    assert_eq!(binary.libraries, ["/good.dylib"]);
    assert_eq!(binary.info.get("rpath").map(String::as_str), Some("/good"));
}
