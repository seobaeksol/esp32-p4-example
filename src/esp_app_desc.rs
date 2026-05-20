//! Minimal ESP-IDF compatible application descriptor.
//!
//! ESP-IDF 2nd stage bootloader tooling expects an `esp_app_desc` symbol in
//! the `.flash.appdesc` section. We provide only that metadata block directly,
//! without depending on `esp-hal` or `esp-bootloader-esp-idf`.

#[repr(C)]
pub struct EspAppDesc {
    magic_word: u32,
    secure_version: u32,
    reserv1: [u32; 2],
    version: [i8; 32],
    project_name: [i8; 32],
    time: [i8; 16],
    date: [i8; 16],
    idf_ver: [i8; 32],
    app_elf_sha256: [u8; 32],
    min_efuse_blk_rev_full: u16,
    max_efuse_blk_rev_full: u16,
    mmu_page_size: u8,
    reserv3: [u8; 3],
    reserv2: [u32; 18],
}

const ESP_APP_DESC_MAGIC_WORD: u32 = 0xABCD_5432;

const fn cstr32(bytes: &[u8]) -> [i8; 32] {
    let mut out = [0i8; 32];
    let mut i = 0;

    while i < bytes.len() && i < 31 {
        out[i] = bytes[i] as i8;
        i += 1;
    }

    out
}

const fn cstr16(bytes: &[u8]) -> [i8; 16] {
    let mut out = [0i8; 16];
    let mut i = 0;

    while i < bytes.len() && i < 15 {
        out[i] = bytes[i] as i8;
        i += 1;
    }

    out
}

// ESP-IDF app descriptor. `espflash` validates this when generating an
// `esp-idf` application image.
#[unsafe(export_name = "esp_app_desc")]
#[unsafe(link_section = ".flash.appdesc")]
#[used]
pub static ESP_APP_DESC: EspAppDesc = EspAppDesc {
    magic_word: ESP_APP_DESC_MAGIC_WORD,
    secure_version: 0,
    reserv1: [0; 2],
    version: cstr32(env!("CARGO_PKG_VERSION").as_bytes()),
    project_name: cstr32(env!("CARGO_PKG_NAME").as_bytes()),
    time: cstr16(b"00:00:00"),
    date: cstr16(b"2026-05-20"),
    idf_ver: cstr32(b"bare-metal"),
    app_elf_sha256: [0; 32],
    min_efuse_blk_rev_full: 0,
    max_efuse_blk_rev_full: u16::MAX,
    // ESP app descriptor stores log2(page_size). 64 KiB => 16.
    mmu_page_size: 16,
    reserv3: [0; 3],
    reserv2: [0; 18],
};
