use crate::helper::{Deserializer, Seeker};
use crate::Result;

pub struct Rel {}

impl Rel {
    pub fn from_bytes<D: Deserializer + Seeker>(_reader: &mut D) -> Result<Self> {
        // let module = reader.read_bu32()?;
        // let _next = reader.read_bu32()?; // should be 0, used at runtime
        // let _prev = reader.read_bu32()?; // should be 0, used at runtime
        // let section_count = reader.read_bu32()?;
        // let section_offset = reader.read_bu32()?;
        // let name_offset = reader.read_bu32()?;
        // let name_size = reader.read_bu32()?;
        // let version = reader.read_bu32()?;
        // let bss_size = reader.read_bu32()?;
        // let relocation_offset = reader.read_bu32()?;
        // let import_offset = reader.read_bu32()?;
        // let import_size = reader.read_bu32()?;
        // let prolog_section = reader.read_bu8()?;
        // let epilog_section = reader.read_bu8()?;
        // let unresolved_section = reader.read_bu8()?;
        // let _bss_section = reader.read_bu8()?; // should be 0, used at runtime
        // let prolog_offset = reader.read_bu32()?;
        // let epilog_offset = reader.read_bu32()?;
        // let unresolved_offset = reader.read_bu32()?;
        //
        // // version 2
        // let (align, bss_align) = if version >= 2 {
        //     let align = reader.read_bu32()?;
        //     let bss_align = reader.read_bu32()?;
        //     (align, bss_align)
        // } else {
        //     (1, 1)
        // };
        //
        // // version 3
        // let fix_size = if version >= 3 {
        //     let fix_size = reader.read_bu32()?;
        //     fix_size
        // } else {
        //     0
        // };
        //
        // ensure!(version <= 3, UnsupportedVersion(version as usize));
        // ensure!(section_count != 0, InvalidData("expected sections (got 0)"));
        // ensure!(section_offset != 0, InvalidData("expected section offset (got 0)"));
        // ensure!(name_offset != 0, InvalidData("expected name offset (got 0)"));
        // ensure!(name_size != 0, InvalidData("expected name size (got 0)"));

        Ok(Rel {})
    }
}

pub fn from_bytes<D: Deserializer + Seeker>(reader: &mut D) -> Result<Rel> {
    Rel::from_bytes(reader)
}
