// This is linked into place at variant/mod.rs because the build system mounts a temporary
// directory at variant/ - see README.md.

use crate::{ConfigurationFiles, Services, Settings};
use bottlerocket_release::BottlerocketRelease;
use model_derive::model;

// We expose anything defined by the current variant.
mod current;

// This is the top-level model exposed by the API system. It contains the common sections for all
// variants.  This allows a single API call to retrieve everything the API system knows, which is
// useful as a check and also, for example, as a data source for templated configuration files.
#[model]
struct Model {
    settings: Settings,
    services: Services,
    configuration_files: ConfigurationFiles,
    os: BottlerocketRelease,
}

use elf::{
    endian::AnyEndian,
    note::{Note, NoteAny},
    section::SectionHeader,
    ElfBytes,
};
use memmap2::MmapOptions;

use std::ffi::CStr;
use std::fs::File;

pub fn get_variant_from_elf_note() -> Option<String> {
    let file = File::open("/proc/self/exe").expect("failed to open file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("failed to mmap file") };
    let elf = ElfBytes::<AnyEndian>::minimal_parse(&mmap).expect("ruh roh");
    let shdr: SectionHeader = elf
        .section_header_by_name(".note.bottlerocket.variant")
        .expect("failed to parse section table")
        .expect("failed to find .note.bottlerocket.variant");
    let note: Vec<Note> = elf
        .section_data_as_notes(&shdr)
        .expect("failed to get note section data")
        .collect();
    let mut variant = None;
    if let Note::Unknown(NoteAny { desc, .. }) = &note[0] {
        if let Ok(v) = CStr::from_bytes_until_nul(desc) {
            if let Ok(v) = v.to_str() {
                variant = Some(v.to_string());
            }
        }
    }

    variant
}
