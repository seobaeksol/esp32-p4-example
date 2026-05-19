#[repr(C)]
#[doc = "Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO"]
#[doc(alias = "TRGT")]
pub struct Trgt {
    hi: Hi,
    lo: Lo,
}
impl Trgt {
    #[doc = "0x00 - system timer comp0 value high register"]
    #[inline(always)]
    pub const fn hi(&self) -> &Hi {
        &self.hi
    }
    #[doc = "0x04 - system timer comp0 value low register"]
    #[inline(always)]
    pub const fn lo(&self) -> &Lo {
        &self.lo
    }
}
#[doc = "HI (rw) register accessor: system timer comp0 value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
#[doc(alias = "HI")]
pub type Hi = crate::Reg<hi::HiSpec>;
#[doc = "system timer comp0 value high register"]
pub mod hi;
#[doc = "LO (rw) register accessor: system timer comp0 value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
#[doc(alias = "LO")]
pub type Lo = crate::Reg<lo::LoSpec>;
#[doc = "system timer comp0 value low register"]
pub mod lo;
