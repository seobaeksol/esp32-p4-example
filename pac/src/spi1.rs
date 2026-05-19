#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: Cmd,
    addr: Addr,
    ctrl: Ctrl,
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    clock: Clock,
    user: User,
    user1: User1,
    user2: User2,
    mosi_dlen: MosiDlen,
    miso_dlen: MisoDlen,
    rd_status: RdStatus,
    _reserved12: [u8; 0x04],
    misc: Misc,
    tx_crc: TxCrc,
    cache_fctrl: CacheFctrl,
    _reserved15: [u8; 0x18],
    w: [W; 16],
    flash_waiti_ctrl: FlashWaitiCtrl,
    flash_sus_ctrl: FlashSusCtrl,
    flash_sus_cmd: FlashSusCmd,
    sus_status: SusStatus,
    _reserved20: [u8; 0x18],
    int_ena: IntEna,
    int_clr: IntClr,
    int_raw: IntRaw,
    int_st: IntSt,
    _reserved24: [u8; 0x04],
    ddr: Ddr,
    _reserved25: [u8; 0xa8],
    timing_cali: TimingCali,
    _reserved26: [u8; 0x7c],
    clock_gate: ClockGate,
    _reserved27: [u8; 0x01f8],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI1 memory command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x04 - SPI1 address register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x08 - SPI1 control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - SPI1 control1 register."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x10 - SPI1 control2 register."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x14 - SPI1 clock division control register."]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x18 - SPI1 user register."]
    #[inline(always)]
    pub const fn user(&self) -> &User {
        &self.user
    }
    #[doc = "0x1c - SPI1 user1 register."]
    #[inline(always)]
    pub const fn user1(&self) -> &User1 {
        &self.user1
    }
    #[doc = "0x20 - SPI1 user2 register."]
    #[inline(always)]
    pub const fn user2(&self) -> &User2 {
        &self.user2
    }
    #[doc = "0x24 - SPI1 send data bit length control register."]
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MosiDlen {
        &self.mosi_dlen
    }
    #[doc = "0x28 - SPI1 receive data bit length control register."]
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MisoDlen {
        &self.miso_dlen
    }
    #[doc = "0x2c - SPI1 status register."]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RdStatus {
        &self.rd_status
    }
    #[doc = "0x34 - SPI1 misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &Misc {
        &self.misc
    }
    #[doc = "0x38 - SPI1 TX CRC data register."]
    #[inline(always)]
    pub const fn tx_crc(&self) -> &TxCrc {
        &self.tx_crc
    }
    #[doc = "0x3c - SPI1 bit mode control register."]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CacheFctrl {
        &self.cache_fctrl
    }
    #[doc = "0x58..0x98 - SPI1 memory data buffer%s"]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x98 - SPI1 memory data buffer%s"]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0x98 - SPI1 wait idle control register"]
    #[inline(always)]
    pub const fn flash_waiti_ctrl(&self) -> &FlashWaitiCtrl {
        &self.flash_waiti_ctrl
    }
    #[doc = "0x9c - SPI1 flash suspend control register"]
    #[inline(always)]
    pub const fn flash_sus_ctrl(&self) -> &FlashSusCtrl {
        &self.flash_sus_ctrl
    }
    #[doc = "0xa0 - SPI1 flash suspend command register"]
    #[inline(always)]
    pub const fn flash_sus_cmd(&self) -> &FlashSusCmd {
        &self.flash_sus_cmd
    }
    #[doc = "0xa4 - SPI1 flash suspend status register"]
    #[inline(always)]
    pub const fn sus_status(&self) -> &SusStatus {
        &self.sus_status
    }
    #[doc = "0xc0 - SPI1 interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0xc4 - SPI1 interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0xc8 - SPI1 interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0xcc - SPI1 interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0xd4 - SPI1 DDR control register"]
    #[inline(always)]
    pub const fn ddr(&self) -> &Ddr {
        &self.ddr
    }
    #[doc = "0x180 - SPI1 timing control register"]
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TimingCali {
        &self.timing_cali
    }
    #[doc = "0x200 - SPI1 clk_gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &ClockGate {
        &self.clock_gate
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: SPI1 memory command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "SPI1 memory command register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: SPI1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "SPI1 address register"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: SPI1 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SPI1 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI1 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "SPI1 control1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (w) register accessor: SPI1 control2 register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "SPI1 control2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI1 clock division control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "SPI1 clock division control register."]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI1 user register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
#[doc(alias = "USER")]
pub type User = crate::Reg<user::UserSpec>;
#[doc = "SPI1 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI1 user1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
#[doc(alias = "USER1")]
pub type User1 = crate::Reg<user1::User1Spec>;
#[doc = "SPI1 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI1 user2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
#[doc(alias = "USER2")]
pub type User2 = crate::Reg<user2::User2Spec>;
#[doc = "SPI1 user2 register."]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: SPI1 send data bit length control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_dlen`] module"]
#[doc(alias = "MOSI_DLEN")]
pub type MosiDlen = crate::Reg<mosi_dlen::MosiDlenSpec>;
#[doc = "SPI1 send data bit length control register."]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: SPI1 receive data bit length control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_dlen`] module"]
#[doc(alias = "MISO_DLEN")]
pub type MisoDlen = crate::Reg<miso_dlen::MisoDlenSpec>;
#[doc = "SPI1 receive data bit length control register."]
pub mod miso_dlen;
#[doc = "RD_STATUS (rw) register accessor: SPI1 status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
#[doc(alias = "RD_STATUS")]
pub type RdStatus = crate::Reg<rd_status::RdStatusSpec>;
#[doc = "SPI1 status register."]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: SPI1 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
#[doc(alias = "MISC")]
pub type Misc = crate::Reg<misc::MiscSpec>;
#[doc = "SPI1 misc register"]
pub mod misc;
#[doc = "TX_CRC (r) register accessor: SPI1 TX CRC data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc`] module"]
#[doc(alias = "TX_CRC")]
pub type TxCrc = crate::Reg<tx_crc::TxCrcSpec>;
#[doc = "SPI1 TX CRC data register."]
pub mod tx_crc;
#[doc = "CACHE_FCTRL (rw) register accessor: SPI1 bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
#[doc(alias = "CACHE_FCTRL")]
pub type CacheFctrl = crate::Reg<cache_fctrl::CacheFctrlSpec>;
#[doc = "SPI1 bit mode control register."]
pub mod cache_fctrl;
#[doc = "W (rw) register accessor: SPI1 memory data buffer%s\n\nYou can [`read`](crate::Reg::read) this register and get [`w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`] module"]
pub type W = crate::Reg<w::WSpec>;
#[doc = "SPI1 memory data buffer%s"]
pub mod w;
#[doc = "FLASH_WAITI_CTRL (rw) register accessor: SPI1 wait idle control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_waiti_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_waiti_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_waiti_ctrl`] module"]
#[doc(alias = "FLASH_WAITI_CTRL")]
pub type FlashWaitiCtrl = crate::Reg<flash_waiti_ctrl::FlashWaitiCtrlSpec>;
#[doc = "SPI1 wait idle control register"]
pub mod flash_waiti_ctrl;
#[doc = "FLASH_SUS_CTRL (rw) register accessor: SPI1 flash suspend control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_ctrl`] module"]
#[doc(alias = "FLASH_SUS_CTRL")]
pub type FlashSusCtrl = crate::Reg<flash_sus_ctrl::FlashSusCtrlSpec>;
#[doc = "SPI1 flash suspend control register"]
pub mod flash_sus_ctrl;
#[doc = "FLASH_SUS_CMD (rw) register accessor: SPI1 flash suspend command register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_cmd`] module"]
#[doc(alias = "FLASH_SUS_CMD")]
pub type FlashSusCmd = crate::Reg<flash_sus_cmd::FlashSusCmdSpec>;
#[doc = "SPI1 flash suspend command register"]
pub mod flash_sus_cmd;
#[doc = "SUS_STATUS (rw) register accessor: SPI1 flash suspend status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sus_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sus_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sus_status`] module"]
#[doc(alias = "SUS_STATUS")]
pub type SusStatus = crate::Reg<sus_status::SusStatusSpec>;
#[doc = "SPI1 flash suspend status register"]
pub mod sus_status;
#[doc = "INT_ENA (rw) register accessor: SPI1 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "SPI1 interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: SPI1 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "SPI1 interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: SPI1 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "SPI1 interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: SPI1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "SPI1 interrupt status register"]
pub mod int_st;
#[doc = "DDR (rw) register accessor: SPI1 DDR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
#[doc(alias = "DDR")]
pub type Ddr = crate::Reg<ddr::DdrSpec>;
#[doc = "SPI1 DDR control register"]
pub mod ddr;
#[doc = "TIMING_CALI (rw) register accessor: SPI1 timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
#[doc(alias = "TIMING_CALI")]
pub type TimingCali = crate::Reg<timing_cali::TimingCaliSpec>;
#[doc = "SPI1 timing control register"]
pub mod timing_cali;
#[doc = "CLOCK_GATE (rw) register accessor: SPI1 clk_gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
#[doc(alias = "CLOCK_GATE")]
pub type ClockGate = crate::Reg<clock_gate::ClockGateSpec>;
#[doc = "SPI1 clk_gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control register"]
pub mod date;
