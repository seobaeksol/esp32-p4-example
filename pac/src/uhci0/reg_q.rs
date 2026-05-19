#[repr(C)]
#[doc = "Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1"]
#[doc(alias = "REG_Q")]
pub struct RegQ {
    word0: Word0,
    word1: Word1,
}
impl RegQ {
    #[doc = "0x00 - UHCI Q0_WORD0 Quick Send Register"]
    #[inline(always)]
    pub const fn word0(&self) -> &Word0 {
        &self.word0
    }
    #[doc = "0x04 - UHCI Q0_WORD1 Quick Send Register"]
    #[inline(always)]
    pub const fn word1(&self) -> &Word1 {
        &self.word1
    }
}
#[doc = "WORD0 (rw) register accessor: UHCI Q0_WORD0 Quick Send Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word0`] module"]
#[doc(alias = "WORD0")]
pub type Word0 = crate::Reg<word0::Word0Spec>;
#[doc = "UHCI Q0_WORD0 Quick Send Register"]
pub mod word0;
#[doc = "WORD1 (rw) register accessor: UHCI Q0_WORD1 Quick Send Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word1`] module"]
#[doc(alias = "WORD1")]
pub type Word1 = crate::Reg<word1::Word1Spec>;
#[doc = "UHCI Q0_WORD1 Quick Send Register"]
pub mod word1;
