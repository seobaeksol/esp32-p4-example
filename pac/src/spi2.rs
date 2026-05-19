#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: Cmd,
    addr: Addr,
    ctrl: Ctrl,
    clock: Clock,
    user: User,
    user1: User1,
    user2: User2,
    ms_dlen: MsDlen,
    misc: Misc,
    din_mode: DinMode,
    din_num: DinNum,
    dout_mode: DoutMode,
    dma_conf: DmaConf,
    dma_int_ena: DmaIntEna,
    dma_int_clr: DmaIntClr,
    dma_int_raw: DmaIntRaw,
    dma_int_st: DmaIntSt,
    dma_int_set: DmaIntSet,
    _reserved18: [u8; 0x50],
    w: [W; 16],
    _reserved19: [u8; 0x08],
    slave: Slave,
    slave1: Slave1,
    clk_gate: ClkGate,
    _reserved22: [u8; 0x04],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - Command control register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x04 - Address value register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x08 - SPI control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - SPI clock control register"]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x10 - SPI USER control register"]
    #[inline(always)]
    pub const fn user(&self) -> &User {
        &self.user
    }
    #[doc = "0x14 - SPI USER control register 1"]
    #[inline(always)]
    pub const fn user1(&self) -> &User1 {
        &self.user1
    }
    #[doc = "0x18 - SPI USER control register 2"]
    #[inline(always)]
    pub const fn user2(&self) -> &User2 {
        &self.user2
    }
    #[doc = "0x1c - SPI data bit length control register"]
    #[inline(always)]
    pub const fn ms_dlen(&self) -> &MsDlen {
        &self.ms_dlen
    }
    #[doc = "0x20 - SPI misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &Misc {
        &self.misc
    }
    #[doc = "0x24 - SPI input delay mode configuration"]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DinMode {
        &self.din_mode
    }
    #[doc = "0x28 - SPI input delay number configuration"]
    #[inline(always)]
    pub const fn din_num(&self) -> &DinNum {
        &self.din_num
    }
    #[doc = "0x2c - SPI output delay mode configuration"]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DoutMode {
        &self.dout_mode
    }
    #[doc = "0x30 - SPI DMA control register"]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DmaConf {
        &self.dma_conf
    }
    #[doc = "0x34 - SPI interrupt enable register"]
    #[inline(always)]
    pub const fn dma_int_ena(&self) -> &DmaIntEna {
        &self.dma_int_ena
    }
    #[doc = "0x38 - SPI interrupt clear register"]
    #[inline(always)]
    pub const fn dma_int_clr(&self) -> &DmaIntClr {
        &self.dma_int_clr
    }
    #[doc = "0x3c - SPI interrupt raw register"]
    #[inline(always)]
    pub const fn dma_int_raw(&self) -> &DmaIntRaw {
        &self.dma_int_raw
    }
    #[doc = "0x40 - SPI interrupt status register"]
    #[inline(always)]
    pub const fn dma_int_st(&self) -> &DmaIntSt {
        &self.dma_int_st
    }
    #[doc = "0x44 - SPI interrupt software set register"]
    #[inline(always)]
    pub const fn dma_int_set(&self) -> &DmaIntSet {
        &self.dma_int_set
    }
    #[doc = "0x98..0xd8 - SPI CPU-controlled buffer%s"]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0xd8 - SPI CPU-controlled buffer%s"]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0xe0 - SPI slave control register"]
    #[inline(always)]
    pub const fn slave(&self) -> &Slave {
        &self.slave
    }
    #[doc = "0xe4 - SPI slave control register 1"]
    #[inline(always)]
    pub const fn slave1(&self) -> &Slave1 {
        &self.slave1
    }
    #[doc = "0xe8 - SPI module clock and register clock control"]
    #[inline(always)]
    pub const fn clk_gate(&self) -> &ClkGate {
        &self.clk_gate
    }
    #[doc = "0xf0 - Version control"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: Command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command control register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: Address value register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address value register"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: SPI control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SPI control register"]
pub mod ctrl;
#[doc = "CLOCK (rw) register accessor: SPI clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "SPI clock control register"]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI USER control register\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
#[doc(alias = "USER")]
pub type User = crate::Reg<user::UserSpec>;
#[doc = "SPI USER control register"]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI USER control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
#[doc(alias = "USER1")]
pub type User1 = crate::Reg<user1::User1Spec>;
#[doc = "SPI USER control register 1"]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI USER control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
#[doc(alias = "USER2")]
pub type User2 = crate::Reg<user2::User2Spec>;
#[doc = "SPI USER control register 2"]
pub mod user2;
#[doc = "MS_DLEN (rw) register accessor: SPI data bit length control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_dlen`] module"]
#[doc(alias = "MS_DLEN")]
pub type MsDlen = crate::Reg<ms_dlen::MsDlenSpec>;
#[doc = "SPI data bit length control register"]
pub mod ms_dlen;
#[doc = "MISC (rw) register accessor: SPI misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
#[doc(alias = "MISC")]
pub type Misc = crate::Reg<misc::MiscSpec>;
#[doc = "SPI misc register"]
pub mod misc;
#[doc = "DIN_MODE (rw) register accessor: SPI input delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
#[doc(alias = "DIN_MODE")]
pub type DinMode = crate::Reg<din_mode::DinModeSpec>;
#[doc = "SPI input delay mode configuration"]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: SPI input delay number configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
#[doc(alias = "DIN_NUM")]
pub type DinNum = crate::Reg<din_num::DinNumSpec>;
#[doc = "SPI input delay number configuration"]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: SPI output delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
#[doc(alias = "DOUT_MODE")]
pub type DoutMode = crate::Reg<dout_mode::DoutModeSpec>;
#[doc = "SPI output delay mode configuration"]
pub mod dout_mode;
#[doc = "DMA_CONF (rw) register accessor: SPI DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
#[doc(alias = "DMA_CONF")]
pub type DmaConf = crate::Reg<dma_conf::DmaConfSpec>;
#[doc = "SPI DMA control register"]
pub mod dma_conf;
#[doc = "DMA_INT_ENA (rw) register accessor: SPI interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_ena`] module"]
#[doc(alias = "DMA_INT_ENA")]
pub type DmaIntEna = crate::Reg<dma_int_ena::DmaIntEnaSpec>;
#[doc = "SPI interrupt enable register"]
pub mod dma_int_ena;
#[doc = "DMA_INT_CLR (w) register accessor: SPI interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_clr`] module"]
#[doc(alias = "DMA_INT_CLR")]
pub type DmaIntClr = crate::Reg<dma_int_clr::DmaIntClrSpec>;
#[doc = "SPI interrupt clear register"]
pub mod dma_int_clr;
#[doc = "DMA_INT_RAW (rw) register accessor: SPI interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_raw`] module"]
#[doc(alias = "DMA_INT_RAW")]
pub type DmaIntRaw = crate::Reg<dma_int_raw::DmaIntRawSpec>;
#[doc = "SPI interrupt raw register"]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST (r) register accessor: SPI interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_st`] module"]
#[doc(alias = "DMA_INT_ST")]
pub type DmaIntSt = crate::Reg<dma_int_st::DmaIntStSpec>;
#[doc = "SPI interrupt status register"]
pub mod dma_int_st;
#[doc = "DMA_INT_SET (w) register accessor: SPI interrupt software set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_set`] module"]
#[doc(alias = "DMA_INT_SET")]
pub type DmaIntSet = crate::Reg<dma_int_set::DmaIntSetSpec>;
#[doc = "SPI interrupt software set register"]
pub mod dma_int_set;
#[doc = "W (rw) register accessor: SPI CPU-controlled buffer%s\n\nYou can [`read`](crate::Reg::read) this register and get [`w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`] module"]
pub type W = crate::Reg<w::WSpec>;
#[doc = "SPI CPU-controlled buffer%s"]
pub mod w;
#[doc = "SLAVE (rw) register accessor: SPI slave control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave`] module"]
#[doc(alias = "SLAVE")]
pub type Slave = crate::Reg<slave::SlaveSpec>;
#[doc = "SPI slave control register"]
pub mod slave;
#[doc = "SLAVE1 (rw) register accessor: SPI slave control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`slave1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave1`] module"]
#[doc(alias = "SLAVE1")]
pub type Slave1 = crate::Reg<slave1::Slave1Spec>;
#[doc = "SPI slave control register 1"]
pub mod slave1;
#[doc = "CLK_GATE (rw) register accessor: SPI module clock and register clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gate`] module"]
#[doc(alias = "CLK_GATE")]
pub type ClkGate = crate::Reg<clk_gate::ClkGateSpec>;
#[doc = "SPI module clock and register clock control"]
pub mod clk_gate;
#[doc = "DATE (rw) register accessor: Version control\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control"]
pub mod date;
