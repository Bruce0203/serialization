pub enum Endian {
    Big,
    Little,
}

impl Endian {
    pub const NATIVE: Endian = {
        if cfg!(target_endian = "big") {
            Endian::Big
        } else {
            Endian::Little
        }
    };
}
