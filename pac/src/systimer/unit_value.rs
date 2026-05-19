#[repr(C)]
#[doc = "Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
#[doc(alias = "UNIT_VALUE")]
pub struct UnitValue {
    hi: Hi,
    lo: Lo,
}
impl UnitValue {
    #[doc = "0x00 - system timer unit0 value high register"]
    #[inline(always)]
    pub const fn hi(&self) -> &Hi {
        &self.hi
    }
    #[doc = "0x04 - system timer unit0 value low register"]
    #[inline(always)]
    pub const fn lo(&self) -> &Lo {
        &self.lo
    }
}
#[doc = "HI (r) register accessor: system timer unit0 value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
#[doc(alias = "HI")]
pub type Hi = crate::Reg<hi::HiSpec>;
#[doc = "system timer unit0 value high register"]
pub mod hi;
#[doc = "LO (r) register accessor: system timer unit0 value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
#[doc(alias = "LO")]
pub type Lo = crate::Reg<lo::LoSpec>;
#[doc = "system timer unit0 value low register"]
pub mod lo;
