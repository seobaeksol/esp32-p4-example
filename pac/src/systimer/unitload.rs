#[repr(C)]
#[doc = "Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
#[doc(alias = "UNITLOAD")]
pub struct Unitload {
    hi: Hi,
    lo: Lo,
}
impl Unitload {
    #[doc = "0x00 - system timer unit0 value high load register"]
    #[inline(always)]
    pub const fn hi(&self) -> &Hi {
        &self.hi
    }
    #[doc = "0x04 - system timer unit0 value low load register"]
    #[inline(always)]
    pub const fn lo(&self) -> &Lo {
        &self.lo
    }
}
#[doc = "HI (rw) register accessor: system timer unit0 value high load register\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
#[doc(alias = "HI")]
pub type Hi = crate::Reg<hi::HiSpec>;
#[doc = "system timer unit0 value high load register"]
pub mod hi;
#[doc = "LO (rw) register accessor: system timer unit0 value low load register\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
#[doc(alias = "LO")]
pub type Lo = crate::Reg<lo::LoSpec>;
#[doc = "system timer unit0 value low load register"]
pub mod lo;
