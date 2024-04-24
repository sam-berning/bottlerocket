use elf::{
    endian::AnyEndian,
    note::{Note, NoteAny},
    section::SectionHeader,
    ElfBytes,
};
use memmap2::MmapOptions;
use snafu::ResultExt;

use std::ffi::CStr;
use std::fs::File;

const VARIANT_NOTE_NAME: &'static str = ".note.bottlerocket.variant";
const DEFAULT_VARIANT: &'static str = "aws-dev";

pub fn get_variant_from_elf_note() -> String {
    if let Ok(variant) = read_self_elf_note(VARIANT_NOTE_NAME) {
        variant
    } else {
        String::from(DEFAULT_VARIANT)
    }
}

fn read_self_elf_note(name: &str) -> Result<String, error::Error> {
    let file = File::open("/proc/self/exe").context(error::IOSnafu)?;
    // MmapOptions.map() is marked unsafe due to the possibility for undefined behavior if the
    // underlying file is modified. We're opening the current executable, which should not change.
    let mmap = unsafe { MmapOptions::new().map(&file).context(error::IOSnafu)? };
    let elf = ElfBytes::<AnyEndian>::minimal_parse(&mmap).context(error::ParseSnafu)?;
    let mut shdr: SectionHeader = elf.section_header_by_name(name).context(error::ParseSnafu)?.unwrap();
    shdr.sh_addralign = 4;
    let note: Vec<Note> = elf.section_data_as_notes(&shdr).context(error::ParseSnafu)?.collect();
    if let Note::Unknown(NoteAny { desc, .. }) = &note[0] {
        return Ok(CStr::from_bytes_until_nul(desc).context(error::FromBytesUntilNulSnafu)?.to_string_lossy().to_string());
    }
    Err(error::Error::NoElfNote{ name: name.to_string() })
}

mod error {
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(super)))]
    pub(super) enum Error {
        #[snafu(display("No ELF note found with name {}", name))]
        NoElfNote {
            name: String,
        },
        #[snafu(display("IO error: {}", source))]
        IO {
            source: std::io::Error,
        },
        #[snafu(display("Parse error: {}", source))]
        Parse {
            source: elf::ParseError,
        },
        #[snafu(display("FromBytesUntilNul error: {}", source))]
        FromBytesUntilNul {
            source: std::ffi::FromBytesUntilNulError,
        }
    }
}
