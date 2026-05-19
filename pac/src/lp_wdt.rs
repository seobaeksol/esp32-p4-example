#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config0: Config0,
    config1: Config1,
    config2: Config2,
    config3: Config3,
    config4: Config4,
    feed: Feed,
    wprotect: Wprotect,
    swd_config: SwdConfig,
    swd_wprotect: SwdWprotect,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    _reserved13: [u8; 0x03c8],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn config0(&self) -> &Config0 {
        &self.config0
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn config1(&self) -> &Config1 {
        &self.config1
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn config2(&self) -> &Config2 {
        &self.config2
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn config3(&self) -> &Config3 {
        &self.config3
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn config4(&self) -> &Config4 {
        &self.config4
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn feed(&self) -> &Feed {
        &self.feed
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn wprotect(&self) -> &Wprotect {
        &self.wprotect
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn swd_config(&self) -> &SwdConfig {
        &self.swd_config
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn swd_wprotect(&self) -> &SwdWprotect {
        &self.swd_wprotect
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "CONFIG0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`] module"]
#[doc(alias = "CONFIG0")]
pub type Config0 = crate::Reg<config0::Config0Spec>;
#[doc = "need_des"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`] module"]
#[doc(alias = "CONFIG1")]
pub type Config1 = crate::Reg<config1::Config1Spec>;
#[doc = "need_des"]
pub mod config1;
#[doc = "CONFIG2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config2`] module"]
#[doc(alias = "CONFIG2")]
pub type Config2 = crate::Reg<config2::Config2Spec>;
#[doc = "need_des"]
pub mod config2;
#[doc = "CONFIG3 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config3`] module"]
#[doc(alias = "CONFIG3")]
pub type Config3 = crate::Reg<config3::Config3Spec>;
#[doc = "need_des"]
pub mod config3;
#[doc = "CONFIG4 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config4`] module"]
#[doc(alias = "CONFIG4")]
pub type Config4 = crate::Reg<config4::Config4Spec>;
#[doc = "need_des"]
pub mod config4;
#[doc = "FEED (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feed`] module"]
#[doc(alias = "FEED")]
pub type Feed = crate::Reg<feed::FeedSpec>;
#[doc = "need_des"]
pub mod feed;
#[doc = "WPROTECT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`wprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wprotect`] module"]
#[doc(alias = "WPROTECT")]
pub type Wprotect = crate::Reg<wprotect::WprotectSpec>;
#[doc = "need_des"]
pub mod wprotect;
#[doc = "SWD_CONFIG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swd_config`] module"]
#[doc(alias = "SWD_CONFIG")]
pub type SwdConfig = crate::Reg<swd_config::SwdConfigSpec>;
#[doc = "need_des"]
pub mod swd_config;
#[doc = "SWD_WPROTECT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_wprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swd_wprotect`] module"]
#[doc(alias = "SWD_WPROTECT")]
pub type SwdWprotect = crate::Reg<swd_wprotect::SwdWprotectSpec>;
#[doc = "need_des"]
pub mod swd_wprotect;
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
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "need_des"]
pub mod date;
