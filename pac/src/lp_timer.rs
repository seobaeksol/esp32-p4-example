#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tar0_low: Tar0Low,
    tar0_high: Tar0High,
    tar1_low: Tar1Low,
    tar1_high: Tar1High,
    update: Update,
    main_buf0_low: MainBuf0Low,
    main_buf0_high: MainBuf0High,
    main_buf1_low: MainBuf1Low,
    main_buf1_high: MainBuf1High,
    main_overflow: MainOverflow,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    lp_int_raw: LpIntRaw,
    lp_int_st: LpIntSt,
    lp_int_ena: LpIntEna,
    lp_int_clr: LpIntClr,
    _reserved18: [u8; 0x03b4],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn tar0_low(&self) -> &Tar0Low {
        &self.tar0_low
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn tar0_high(&self) -> &Tar0High {
        &self.tar0_high
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn tar1_low(&self) -> &Tar1Low {
        &self.tar1_low
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn tar1_high(&self) -> &Tar1High {
        &self.tar1_high
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn update(&self) -> &Update {
        &self.update
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn main_buf0_low(&self) -> &MainBuf0Low {
        &self.main_buf0_low
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn main_buf0_high(&self) -> &MainBuf0High {
        &self.main_buf0_high
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn main_buf1_low(&self) -> &MainBuf1Low {
        &self.main_buf1_low
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn main_buf1_high(&self) -> &MainBuf1High {
        &self.main_buf1_high
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn main_overflow(&self) -> &MainOverflow {
        &self.main_overflow
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_int_raw(&self) -> &LpIntRaw {
        &self.lp_int_raw
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_int_st(&self) -> &LpIntSt {
        &self.lp_int_st
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn lp_int_ena(&self) -> &LpIntEna {
        &self.lp_int_ena
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn lp_int_clr(&self) -> &LpIntClr {
        &self.lp_int_clr
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "TAR0_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tar0_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar0_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar0_low`] module"]
#[doc(alias = "TAR0_LOW")]
pub type Tar0Low = crate::Reg<tar0_low::Tar0LowSpec>;
#[doc = "need_des"]
pub mod tar0_low;
#[doc = "TAR0_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tar0_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar0_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar0_high`] module"]
#[doc(alias = "TAR0_HIGH")]
pub type Tar0High = crate::Reg<tar0_high::Tar0HighSpec>;
#[doc = "need_des"]
pub mod tar0_high;
#[doc = "TAR1_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tar1_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar1_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar1_low`] module"]
#[doc(alias = "TAR1_LOW")]
pub type Tar1Low = crate::Reg<tar1_low::Tar1LowSpec>;
#[doc = "need_des"]
pub mod tar1_low;
#[doc = "TAR1_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tar1_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar1_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar1_high`] module"]
#[doc(alias = "TAR1_HIGH")]
pub type Tar1High = crate::Reg<tar1_high::Tar1HighSpec>;
#[doc = "need_des"]
pub mod tar1_high;
#[doc = "UPDATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@update`] module"]
#[doc(alias = "UPDATE")]
pub type Update = crate::Reg<update::UpdateSpec>;
#[doc = "need_des"]
pub mod update;
#[doc = "MAIN_BUF0_LOW (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_buf0_low::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_buf0_low`] module"]
#[doc(alias = "MAIN_BUF0_LOW")]
pub type MainBuf0Low = crate::Reg<main_buf0_low::MainBuf0LowSpec>;
#[doc = "need_des"]
pub mod main_buf0_low;
#[doc = "MAIN_BUF0_HIGH (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_buf0_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_buf0_high`] module"]
#[doc(alias = "MAIN_BUF0_HIGH")]
pub type MainBuf0High = crate::Reg<main_buf0_high::MainBuf0HighSpec>;
#[doc = "need_des"]
pub mod main_buf0_high;
#[doc = "MAIN_BUF1_LOW (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_buf1_low::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_buf1_low`] module"]
#[doc(alias = "MAIN_BUF1_LOW")]
pub type MainBuf1Low = crate::Reg<main_buf1_low::MainBuf1LowSpec>;
#[doc = "need_des"]
pub mod main_buf1_low;
#[doc = "MAIN_BUF1_HIGH (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_buf1_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_buf1_high`] module"]
#[doc(alias = "MAIN_BUF1_HIGH")]
pub type MainBuf1High = crate::Reg<main_buf1_high::MainBuf1HighSpec>;
#[doc = "need_des"]
pub mod main_buf1_high;
#[doc = "MAIN_OVERFLOW (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`main_overflow::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_overflow`] module"]
#[doc(alias = "MAIN_OVERFLOW")]
pub type MainOverflow = crate::Reg<main_overflow::MainOverflowSpec>;
#[doc = "need_des"]
pub mod main_overflow;
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
#[doc = "LP_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_raw`] module"]
#[doc(alias = "LP_INT_RAW")]
pub type LpIntRaw = crate::Reg<lp_int_raw::LpIntRawSpec>;
#[doc = "need_des"]
pub mod lp_int_raw;
#[doc = "LP_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_st`] module"]
#[doc(alias = "LP_INT_ST")]
pub type LpIntSt = crate::Reg<lp_int_st::LpIntStSpec>;
#[doc = "need_des"]
pub mod lp_int_st;
#[doc = "LP_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_ena`] module"]
#[doc(alias = "LP_INT_ENA")]
pub type LpIntEna = crate::Reg<lp_int_ena::LpIntEnaSpec>;
#[doc = "need_des"]
pub mod lp_int_ena;
#[doc = "LP_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_clr`] module"]
#[doc(alias = "LP_INT_CLR")]
pub type LpIntClr = crate::Reg<lp_int_clr::LpIntClrSpec>;
#[doc = "need_des"]
pub mod lp_int_clr;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "need_des"]
pub mod date;
