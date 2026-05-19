#[repr(C)]
#[doc = "Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI"]
#[doc(alias = "REAL_TARGET")]
pub struct RealTarget {
    lo: Lo,
    hi: Hi,
}
impl RealTarget {
    #[doc = "0x00 - system timer comp0 actual target value low register"]
    #[inline(always)]
    pub const fn lo(&self) -> &Lo {
        &self.lo
    }
    #[doc = "0x04 - system timer comp0 actual target value high register"]
    #[inline(always)]
    pub const fn hi(&self) -> &Hi {
        &self.hi
    }
}
#[doc = "LO (r) register accessor: system timer comp0 actual target value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
#[doc(alias = "LO")]
pub type Lo = crate::Reg<lo::LoSpec>;
#[doc = "system timer comp0 actual target value low register"]
pub mod lo;
#[doc = "HI (r) register accessor: system timer comp0 actual target value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
#[doc(alias = "HI")]
pub type Hi = crate::Reg<hi::HiSpec>;
#[doc = "system timer comp0 actual target value high register"]
pub mod hi;
