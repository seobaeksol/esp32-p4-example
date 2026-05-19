#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    chn_status: ChnStatus,
    status_0: Status0,
    status_1: Status1,
    status_2: Status2,
    status_3: Status3,
    status_4: Status4,
    status_5: Status5,
    status_6: Status6,
    status_7: Status7,
    status_8: Status8,
    status_9: Status9,
    status_10: Status10,
    status_11: Status11,
    status_12: Status12,
    status_13: Status13,
    status_14: Status14,
    status_15: Status15,
    status_16: Status16,
    status_17: Status17,
    chn_tmp_status: ChnTmpStatus,
    _reserved24: [u8; 0xa0],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn chn_status(&self) -> &ChnStatus {
        &self.chn_status
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn status_0(&self) -> &Status0 {
        &self.status_0
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn status_1(&self) -> &Status1 {
        &self.status_1
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn status_2(&self) -> &Status2 {
        &self.status_2
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn status_3(&self) -> &Status3 {
        &self.status_3
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn status_4(&self) -> &Status4 {
        &self.status_4
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn status_5(&self) -> &Status5 {
        &self.status_5
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn status_6(&self) -> &Status6 {
        &self.status_6
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn status_7(&self) -> &Status7 {
        &self.status_7
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn status_8(&self) -> &Status8 {
        &self.status_8
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn status_9(&self) -> &Status9 {
        &self.status_9
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn status_10(&self) -> &Status10 {
        &self.status_10
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn status_11(&self) -> &Status11 {
        &self.status_11
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn status_12(&self) -> &Status12 {
        &self.status_12
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn status_13(&self) -> &Status13 {
        &self.status_13
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn status_14(&self) -> &Status14 {
        &self.status_14
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn status_15(&self) -> &Status15 {
        &self.status_15
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn status_16(&self) -> &Status16 {
        &self.status_16
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn status_17(&self) -> &Status17 {
        &self.status_17
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn chn_tmp_status(&self) -> &ChnTmpStatus {
        &self.chn_tmp_status
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "need_des"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "need_des"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "need_des"]
pub mod int_clr;
#[doc = "CHN_STATUS (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn_status`] module"]
#[doc(alias = "CHN_STATUS")]
pub type ChnStatus = crate::Reg<chn_status::ChnStatusSpec>;
#[doc = "need_des"]
pub mod chn_status;
#[doc = "STATUS_0 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_0`] module"]
#[doc(alias = "STATUS_0")]
pub type Status0 = crate::Reg<status_0::Status0Spec>;
#[doc = "need_des"]
pub mod status_0;
#[doc = "STATUS_1 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_1`] module"]
#[doc(alias = "STATUS_1")]
pub type Status1 = crate::Reg<status_1::Status1Spec>;
#[doc = "need_des"]
pub mod status_1;
#[doc = "STATUS_2 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_2`] module"]
#[doc(alias = "STATUS_2")]
pub type Status2 = crate::Reg<status_2::Status2Spec>;
#[doc = "need_des"]
pub mod status_2;
#[doc = "STATUS_3 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_3`] module"]
#[doc(alias = "STATUS_3")]
pub type Status3 = crate::Reg<status_3::Status3Spec>;
#[doc = "need_des"]
pub mod status_3;
#[doc = "STATUS_4 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_4`] module"]
#[doc(alias = "STATUS_4")]
pub type Status4 = crate::Reg<status_4::Status4Spec>;
#[doc = "need_des"]
pub mod status_4;
#[doc = "STATUS_5 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_5`] module"]
#[doc(alias = "STATUS_5")]
pub type Status5 = crate::Reg<status_5::Status5Spec>;
#[doc = "need_des"]
pub mod status_5;
#[doc = "STATUS_6 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_6`] module"]
#[doc(alias = "STATUS_6")]
pub type Status6 = crate::Reg<status_6::Status6Spec>;
#[doc = "need_des"]
pub mod status_6;
#[doc = "STATUS_7 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_7`] module"]
#[doc(alias = "STATUS_7")]
pub type Status7 = crate::Reg<status_7::Status7Spec>;
#[doc = "need_des"]
pub mod status_7;
#[doc = "STATUS_8 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_8`] module"]
#[doc(alias = "STATUS_8")]
pub type Status8 = crate::Reg<status_8::Status8Spec>;
#[doc = "need_des"]
pub mod status_8;
#[doc = "STATUS_9 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_9`] module"]
#[doc(alias = "STATUS_9")]
pub type Status9 = crate::Reg<status_9::Status9Spec>;
#[doc = "need_des"]
pub mod status_9;
#[doc = "STATUS_10 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_10`] module"]
#[doc(alias = "STATUS_10")]
pub type Status10 = crate::Reg<status_10::Status10Spec>;
#[doc = "need_des"]
pub mod status_10;
#[doc = "STATUS_11 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_11`] module"]
#[doc(alias = "STATUS_11")]
pub type Status11 = crate::Reg<status_11::Status11Spec>;
#[doc = "need_des"]
pub mod status_11;
#[doc = "STATUS_12 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_12`] module"]
#[doc(alias = "STATUS_12")]
pub type Status12 = crate::Reg<status_12::Status12Spec>;
#[doc = "need_des"]
pub mod status_12;
#[doc = "STATUS_13 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_13`] module"]
#[doc(alias = "STATUS_13")]
pub type Status13 = crate::Reg<status_13::Status13Spec>;
#[doc = "need_des"]
pub mod status_13;
#[doc = "STATUS_14 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_14`] module"]
#[doc(alias = "STATUS_14")]
pub type Status14 = crate::Reg<status_14::Status14Spec>;
#[doc = "need_des"]
pub mod status_14;
#[doc = "STATUS_15 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_15`] module"]
#[doc(alias = "STATUS_15")]
pub type Status15 = crate::Reg<status_15::Status15Spec>;
#[doc = "need_des"]
pub mod status_15;
#[doc = "STATUS_16 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_16`] module"]
#[doc(alias = "STATUS_16")]
pub type Status16 = crate::Reg<status_16::Status16Spec>;
#[doc = "need_des"]
pub mod status_16;
#[doc = "STATUS_17 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_17`] module"]
#[doc(alias = "STATUS_17")]
pub type Status17 = crate::Reg<status_17::Status17Spec>;
#[doc = "need_des"]
pub mod status_17;
#[doc = "CHN_TMP_STATUS (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_tmp_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn_tmp_status`] module"]
#[doc(alias = "CHN_TMP_STATUS")]
pub type ChnTmpStatus = crate::Reg<chn_tmp_status::ChnTmpStatusSpec>;
#[doc = "need_des"]
pub mod chn_tmp_status;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "need_des"]
pub mod date;
