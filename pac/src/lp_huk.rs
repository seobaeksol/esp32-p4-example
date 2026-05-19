#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    clk: Clk,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    _reserved5: [u8; 0x08],
    conf: Conf,
    start: Start,
    state: State,
    _reserved8: [u8; 0x08],
    status: Status,
    _reserved9: [u8; 0xc4],
    date: Date,
    info_mem: [InfoMem; 96],
}
impl RegisterBlock {
    #[doc = "0x04 - HUK Generator clock gate control register"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x08 - HUK Generator interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x0c - HUK Generator interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x10 - HUK Generator interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x14 - HUK Generator interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x20 - HUK Generator configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x24 - HUK Generator control register"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x28 - HUK Generator state register"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x34 - HUK Generator HUK status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x100..0x280 - The memory that stores HUK info."]
    #[inline(always)]
    pub const fn info_mem(&self, n: usize) -> &InfoMem {
        &self.info_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x280 - The memory that stores HUK info."]
    #[inline(always)]
    pub fn info_mem_iter(&self) -> impl Iterator<Item = &InfoMem> {
        self.info_mem.iter()
    }
}
#[doc = "CLK (rw) register accessor: HUK Generator clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "HUK Generator clock gate control register"]
pub mod clk;
#[doc = "INT_RAW (r) register accessor: HUK Generator interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "HUK Generator interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: HUK Generator interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "HUK Generator interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: HUK Generator interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "HUK Generator interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: HUK Generator interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "HUK Generator interrupt clear register."]
pub mod int_clr;
#[doc = "CONF (rw) register accessor: HUK Generator configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "HUK Generator configuration register"]
pub mod conf;
#[doc = "START (w) register accessor: HUK Generator control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "HUK Generator control register"]
pub mod start;
#[doc = "STATE (r) register accessor: HUK Generator state register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "HUK Generator state register"]
pub mod state;
#[doc = "STATUS (r) register accessor: HUK Generator HUK status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "HUK Generator HUK status register"]
pub mod status;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control register"]
pub mod date;
#[doc = "INFO_MEM (rw) register accessor: The memory that stores HUK info.\n\nYou can [`read`](crate::Reg::read) this register and get [`info_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info_mem`] module"]
#[doc(alias = "INFO_MEM")]
pub type InfoMem = crate::Reg<info_mem::InfoMemSpec>;
#[doc = "The memory that stores HUK info."]
pub mod info_mem;
