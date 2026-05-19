#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x04],
    clkdiv: Clkdiv,
    clksrc: Clksrc,
    clkena: Clkena,
    tmout: Tmout,
    ctype: Ctype,
    blksiz: Blksiz,
    bytcnt: Bytcnt,
    intmask: Intmask,
    cmdarg: Cmdarg,
    cmd: Cmd,
    resp0: Resp0,
    resp1: Resp1,
    resp2: Resp2,
    resp3: Resp3,
    mintsts: Mintsts,
    rintsts: Rintsts,
    status: Status,
    fifoth: Fifoth,
    cdetect: Cdetect,
    wrtprt: Wrtprt,
    _reserved21: [u8; 0x04],
    tcbcnt: Tcbcnt,
    tbbcnt: Tbbcnt,
    debnce: Debnce,
    usrid: Usrid,
    verid: Verid,
    hcon: Hcon,
    uhs: Uhs,
    rst_n: RstN,
    _reserved29: [u8; 0x04],
    bmod: Bmod,
    pldmnd: Pldmnd,
    dbaddr: Dbaddr,
    idsts: Idsts,
    idinten: Idinten,
    dscaddr: Dscaddr,
    bufaddr: Bufaddr,
    _reserved36: [u8; 0x64],
    cardthrctl: Cardthrctl,
    _reserved37: [u8; 0x08],
    emmcddr: Emmcddr,
    enshift: Enshift,
    _reserved39: [u8; 0xec],
    buffifo: Buffifo,
    _reserved40: [u8; 0x05fc],
    clk_edge_sel: ClkEdgeSel,
    raw_ints: RawInts,
    dll_clk_conf: DllClkConf,
    dll_conf: DllConf,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Clock divider configuration register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x0c - Clock source selection register"]
    #[inline(always)]
    pub const fn clksrc(&self) -> &Clksrc {
        &self.clksrc
    }
    #[doc = "0x10 - Clock enable register"]
    #[inline(always)]
    pub const fn clkena(&self) -> &Clkena {
        &self.clkena
    }
    #[doc = "0x14 - Data and response timeout configuration register"]
    #[inline(always)]
    pub const fn tmout(&self) -> &Tmout {
        &self.tmout
    }
    #[doc = "0x18 - Card bus width configuration register"]
    #[inline(always)]
    pub const fn ctype(&self) -> &Ctype {
        &self.ctype
    }
    #[doc = "0x1c - Card data block size configuration register"]
    #[inline(always)]
    pub const fn blksiz(&self) -> &Blksiz {
        &self.blksiz
    }
    #[doc = "0x20 - Data transfer length configuration register"]
    #[inline(always)]
    pub const fn bytcnt(&self) -> &Bytcnt {
        &self.bytcnt
    }
    #[doc = "0x24 - SDIO interrupt mask register"]
    #[inline(always)]
    pub const fn intmask(&self) -> &Intmask {
        &self.intmask
    }
    #[doc = "0x28 - Command argument data register"]
    #[inline(always)]
    pub const fn cmdarg(&self) -> &Cmdarg {
        &self.cmdarg
    }
    #[doc = "0x2c - Command and boot configuration register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x30 - Response data register"]
    #[inline(always)]
    pub const fn resp0(&self) -> &Resp0 {
        &self.resp0
    }
    #[doc = "0x34 - Long response data register"]
    #[inline(always)]
    pub const fn resp1(&self) -> &Resp1 {
        &self.resp1
    }
    #[doc = "0x38 - Long response data register"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x3c - Long response data register"]
    #[inline(always)]
    pub const fn resp3(&self) -> &Resp3 {
        &self.resp3
    }
    #[doc = "0x40 - Masked interrupt status register"]
    #[inline(always)]
    pub const fn mintsts(&self) -> &Mintsts {
        &self.mintsts
    }
    #[doc = "0x44 - Raw interrupt status register"]
    #[inline(always)]
    pub const fn rintsts(&self) -> &Rintsts {
        &self.rintsts
    }
    #[doc = "0x48 - SD/MMC status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x4c - FIFO configuration register"]
    #[inline(always)]
    pub const fn fifoth(&self) -> &Fifoth {
        &self.fifoth
    }
    #[doc = "0x50 - Card detect register"]
    #[inline(always)]
    pub const fn cdetect(&self) -> &Cdetect {
        &self.cdetect
    }
    #[doc = "0x54 - Card write protection (WP) status register"]
    #[inline(always)]
    pub const fn wrtprt(&self) -> &Wrtprt {
        &self.wrtprt
    }
    #[doc = "0x5c - Transferred byte count register"]
    #[inline(always)]
    pub const fn tcbcnt(&self) -> &Tcbcnt {
        &self.tcbcnt
    }
    #[doc = "0x60 - Transferred byte count register"]
    #[inline(always)]
    pub const fn tbbcnt(&self) -> &Tbbcnt {
        &self.tbbcnt
    }
    #[doc = "0x64 - Debounce filter time configuration register"]
    #[inline(always)]
    pub const fn debnce(&self) -> &Debnce {
        &self.debnce
    }
    #[doc = "0x68 - User ID (scratchpad) register"]
    #[inline(always)]
    pub const fn usrid(&self) -> &Usrid {
        &self.usrid
    }
    #[doc = "0x6c - Version ID (scratchpad) register"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x70 - Hardware feature register"]
    #[inline(always)]
    pub const fn hcon(&self) -> &Hcon {
        &self.hcon
    }
    #[doc = "0x74 - UHS-1 register"]
    #[inline(always)]
    pub const fn uhs(&self) -> &Uhs {
        &self.uhs
    }
    #[doc = "0x78 - Card reset register"]
    #[inline(always)]
    pub const fn rst_n(&self) -> &RstN {
        &self.rst_n
    }
    #[doc = "0x80 - Burst mode transfer configuration register"]
    #[inline(always)]
    pub const fn bmod(&self) -> &Bmod {
        &self.bmod
    }
    #[doc = "0x84 - Poll demand configuration register"]
    #[inline(always)]
    pub const fn pldmnd(&self) -> &Pldmnd {
        &self.pldmnd
    }
    #[doc = "0x88 - Descriptor base address register"]
    #[inline(always)]
    pub const fn dbaddr(&self) -> &Dbaddr {
        &self.dbaddr
    }
    #[doc = "0x8c - IDMAC status register"]
    #[inline(always)]
    pub const fn idsts(&self) -> &Idsts {
        &self.idsts
    }
    #[doc = "0x90 - IDMAC interrupt enable register"]
    #[inline(always)]
    pub const fn idinten(&self) -> &Idinten {
        &self.idinten
    }
    #[doc = "0x94 - Host descriptor address pointer"]
    #[inline(always)]
    pub const fn dscaddr(&self) -> &Dscaddr {
        &self.dscaddr
    }
    #[doc = "0x98 - Host buffer address pointer register"]
    #[inline(always)]
    pub const fn bufaddr(&self) -> &Bufaddr {
        &self.bufaddr
    }
    #[doc = "0x100 - Card Threshold Control register"]
    #[inline(always)]
    pub const fn cardthrctl(&self) -> &Cardthrctl {
        &self.cardthrctl
    }
    #[doc = "0x10c - eMMC DDR register"]
    #[inline(always)]
    pub const fn emmcddr(&self) -> &Emmcddr {
        &self.emmcddr
    }
    #[doc = "0x110 - Enable Phase Shift register"]
    #[inline(always)]
    pub const fn enshift(&self) -> &Enshift {
        &self.enshift
    }
    #[doc = "0x200 - CPU write and read transmit data by FIFO"]
    #[inline(always)]
    pub const fn buffifo(&self) -> &Buffifo {
        &self.buffifo
    }
    #[doc = "0x800 - SDIO control register."]
    #[inline(always)]
    pub const fn clk_edge_sel(&self) -> &ClkEdgeSel {
        &self.clk_edge_sel
    }
    #[doc = "0x804 - SDIO raw ints register."]
    #[inline(always)]
    pub const fn raw_ints(&self) -> &RawInts {
        &self.raw_ints
    }
    #[doc = "0x808 - SDIO DLL clock control register."]
    #[inline(always)]
    pub const fn dll_clk_conf(&self) -> &DllClkConf {
        &self.dll_clk_conf
    }
    #[doc = "0x80c - SDIO DLL configuration register."]
    #[inline(always)]
    pub const fn dll_conf(&self) -> &DllConf {
        &self.dll_conf
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "CLKDIV (rw) register accessor: Clock divider configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock divider configuration register"]
pub mod clkdiv;
#[doc = "CLKSRC (rw) register accessor: Clock source selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksrc`] module"]
#[doc(alias = "CLKSRC")]
pub type Clksrc = crate::Reg<clksrc::ClksrcSpec>;
#[doc = "Clock source selection register"]
pub mod clksrc;
#[doc = "CLKENA (rw) register accessor: Clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkena`] module"]
#[doc(alias = "CLKENA")]
pub type Clkena = crate::Reg<clkena::ClkenaSpec>;
#[doc = "Clock enable register"]
pub mod clkena;
#[doc = "TMOUT (rw) register accessor: Data and response timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmout`] module"]
#[doc(alias = "TMOUT")]
pub type Tmout = crate::Reg<tmout::TmoutSpec>;
#[doc = "Data and response timeout configuration register"]
pub mod tmout;
#[doc = "CTYPE (rw) register accessor: Card bus width configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctype`] module"]
#[doc(alias = "CTYPE")]
pub type Ctype = crate::Reg<ctype::CtypeSpec>;
#[doc = "Card bus width configuration register"]
pub mod ctype;
#[doc = "BLKSIZ (rw) register accessor: Card data block size configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`blksiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksiz`] module"]
#[doc(alias = "BLKSIZ")]
pub type Blksiz = crate::Reg<blksiz::BlksizSpec>;
#[doc = "Card data block size configuration register"]
pub mod blksiz;
#[doc = "BYTCNT (rw) register accessor: Data transfer length configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bytcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bytcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bytcnt`] module"]
#[doc(alias = "BYTCNT")]
pub type Bytcnt = crate::Reg<bytcnt::BytcntSpec>;
#[doc = "Data transfer length configuration register"]
pub mod bytcnt;
#[doc = "INTMASK (rw) register accessor: SDIO interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`] module"]
#[doc(alias = "INTMASK")]
pub type Intmask = crate::Reg<intmask::IntmaskSpec>;
#[doc = "SDIO interrupt mask register"]
pub mod intmask;
#[doc = "CMDARG (rw) register accessor: Command argument data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdarg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdarg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdarg`] module"]
#[doc(alias = "CMDARG")]
pub type Cmdarg = crate::Reg<cmdarg::CmdargSpec>;
#[doc = "Command argument data register"]
pub mod cmdarg;
#[doc = "CMD (rw) register accessor: Command and boot configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command and boot configuration register"]
pub mod cmd;
#[doc = "RESP0 (r) register accessor: Response data register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`] module"]
#[doc(alias = "RESP0")]
pub type Resp0 = crate::Reg<resp0::Resp0Spec>;
#[doc = "Response data register"]
pub mod resp0;
#[doc = "RESP1 (r) register accessor: Long response data register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`] module"]
#[doc(alias = "RESP1")]
pub type Resp1 = crate::Reg<resp1::Resp1Spec>;
#[doc = "Long response data register"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: Long response data register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`] module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "Long response data register"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: Long response data register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`] module"]
#[doc(alias = "RESP3")]
pub type Resp3 = crate::Reg<resp3::Resp3Spec>;
#[doc = "Long response data register"]
pub mod resp3;
#[doc = "MINTSTS (r) register accessor: Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintsts`] module"]
#[doc(alias = "MINTSTS")]
pub type Mintsts = crate::Reg<mintsts::MintstsSpec>;
#[doc = "Masked interrupt status register"]
pub mod mintsts;
#[doc = "RINTSTS (rw) register accessor: Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rintsts`] module"]
#[doc(alias = "RINTSTS")]
pub type Rintsts = crate::Reg<rintsts::RintstsSpec>;
#[doc = "Raw interrupt status register"]
pub mod rintsts;
#[doc = "STATUS (r) register accessor: SD/MMC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "SD/MMC status register"]
pub mod status;
#[doc = "FIFOTH (rw) register accessor: FIFO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoth`] module"]
#[doc(alias = "FIFOTH")]
pub type Fifoth = crate::Reg<fifoth::FifothSpec>;
#[doc = "FIFO configuration register"]
pub mod fifoth;
#[doc = "CDETECT (r) register accessor: Card detect register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdetect::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdetect`] module"]
#[doc(alias = "CDETECT")]
pub type Cdetect = crate::Reg<cdetect::CdetectSpec>;
#[doc = "Card detect register"]
pub mod cdetect;
#[doc = "WRTPRT (r) register accessor: Card write protection (WP) status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtprt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtprt`] module"]
#[doc(alias = "WRTPRT")]
pub type Wrtprt = crate::Reg<wrtprt::WrtprtSpec>;
#[doc = "Card write protection (WP) status register"]
pub mod wrtprt;
#[doc = "TCBCNT (r) register accessor: Transferred byte count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcbcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcbcnt`] module"]
#[doc(alias = "TCBCNT")]
pub type Tcbcnt = crate::Reg<tcbcnt::TcbcntSpec>;
#[doc = "Transferred byte count register"]
pub mod tcbcnt;
#[doc = "TBBCNT (r) register accessor: Transferred byte count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbbcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbbcnt`] module"]
#[doc(alias = "TBBCNT")]
pub type Tbbcnt = crate::Reg<tbbcnt::TbbcntSpec>;
#[doc = "Transferred byte count register"]
pub mod tbbcnt;
#[doc = "DEBNCE (rw) register accessor: Debounce filter time configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`debnce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debnce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debnce`] module"]
#[doc(alias = "DEBNCE")]
pub type Debnce = crate::Reg<debnce::DebnceSpec>;
#[doc = "Debounce filter time configuration register"]
pub mod debnce;
#[doc = "USRID (rw) register accessor: User ID (scratchpad) register\n\nYou can [`read`](crate::Reg::read) this register and get [`usrid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usrid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrid`] module"]
#[doc(alias = "USRID")]
pub type Usrid = crate::Reg<usrid::UsridSpec>;
#[doc = "User ID (scratchpad) register"]
pub mod usrid;
#[doc = "VERID (r) register accessor: Version ID (scratchpad) register\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`] module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID (scratchpad) register"]
pub mod verid;
#[doc = "HCON (r) register accessor: Hardware feature register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcon`] module"]
#[doc(alias = "HCON")]
pub type Hcon = crate::Reg<hcon::HconSpec>;
#[doc = "Hardware feature register"]
pub mod hcon;
#[doc = "UHS (rw) register accessor: UHS-1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs`] module"]
#[doc(alias = "UHS")]
pub type Uhs = crate::Reg<uhs::UhsSpec>;
#[doc = "UHS-1 register"]
pub mod uhs;
#[doc = "RST_N (rw) register accessor: Card reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_n::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_n::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_n`] module"]
#[doc(alias = "RST_N")]
pub type RstN = crate::Reg<rst_n::RstNSpec>;
#[doc = "Card reset register"]
pub mod rst_n;
#[doc = "BMOD (rw) register accessor: Burst mode transfer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmod`] module"]
#[doc(alias = "BMOD")]
pub type Bmod = crate::Reg<bmod::BmodSpec>;
#[doc = "Burst mode transfer configuration register"]
pub mod bmod;
#[doc = "PLDMND (w) register accessor: Poll demand configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pldmnd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pldmnd`] module"]
#[doc(alias = "PLDMND")]
pub type Pldmnd = crate::Reg<pldmnd::PldmndSpec>;
#[doc = "Poll demand configuration register"]
pub mod pldmnd;
#[doc = "DBADDR (rw) register accessor: Descriptor base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbaddr`] module"]
#[doc(alias = "DBADDR")]
pub type Dbaddr = crate::Reg<dbaddr::DbaddrSpec>;
#[doc = "Descriptor base address register"]
pub mod dbaddr;
#[doc = "IDSTS (rw) register accessor: IDMAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`idsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idsts`] module"]
#[doc(alias = "IDSTS")]
pub type Idsts = crate::Reg<idsts::IdstsSpec>;
#[doc = "IDMAC status register"]
pub mod idsts;
#[doc = "IDINTEN (rw) register accessor: IDMAC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`idinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idinten`] module"]
#[doc(alias = "IDINTEN")]
pub type Idinten = crate::Reg<idinten::IdintenSpec>;
#[doc = "IDMAC interrupt enable register"]
pub mod idinten;
#[doc = "DSCADDR (r) register accessor: Host descriptor address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`dscaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscaddr`] module"]
#[doc(alias = "DSCADDR")]
pub type Dscaddr = crate::Reg<dscaddr::DscaddrSpec>;
#[doc = "Host descriptor address pointer"]
pub mod dscaddr;
#[doc = "BUFADDR (r) register accessor: Host buffer address pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufaddr`] module"]
#[doc(alias = "BUFADDR")]
pub type Bufaddr = crate::Reg<bufaddr::BufaddrSpec>;
#[doc = "Host buffer address pointer register"]
pub mod bufaddr;
#[doc = "CARDTHRCTL (rw) register accessor: Card Threshold Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cardthrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cardthrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cardthrctl`] module"]
#[doc(alias = "CARDTHRCTL")]
pub type Cardthrctl = crate::Reg<cardthrctl::CardthrctlSpec>;
#[doc = "Card Threshold Control register"]
pub mod cardthrctl;
#[doc = "EMMCDDR (rw) register accessor: eMMC DDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`emmcddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emmcddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcddr`] module"]
#[doc(alias = "EMMCDDR")]
pub type Emmcddr = crate::Reg<emmcddr::EmmcddrSpec>;
#[doc = "eMMC DDR register"]
pub mod emmcddr;
#[doc = "ENSHIFT (rw) register accessor: Enable Phase Shift register\n\nYou can [`read`](crate::Reg::read) this register and get [`enshift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enshift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enshift`] module"]
#[doc(alias = "ENSHIFT")]
pub type Enshift = crate::Reg<enshift::EnshiftSpec>;
#[doc = "Enable Phase Shift register"]
pub mod enshift;
#[doc = "BUFFIFO (rw) register accessor: CPU write and read transmit data by FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`buffifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buffifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffifo`] module"]
#[doc(alias = "BUFFIFO")]
pub type Buffifo = crate::Reg<buffifo::BuffifoSpec>;
#[doc = "CPU write and read transmit data by FIFO"]
pub mod buffifo;
#[doc = "CLK_EDGE_SEL (rw) register accessor: SDIO control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_edge_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_edge_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_edge_sel`] module"]
#[doc(alias = "CLK_EDGE_SEL")]
pub type ClkEdgeSel = crate::Reg<clk_edge_sel::ClkEdgeSelSpec>;
#[doc = "SDIO control register."]
pub mod clk_edge_sel;
#[doc = "RAW_INTS (r) register accessor: SDIO raw ints register.\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_ints::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_ints`] module"]
#[doc(alias = "RAW_INTS")]
pub type RawInts = crate::Reg<raw_ints::RawIntsSpec>;
#[doc = "SDIO raw ints register."]
pub mod raw_ints;
#[doc = "DLL_CLK_CONF (rw) register accessor: SDIO DLL clock control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_clk_conf`] module"]
#[doc(alias = "DLL_CLK_CONF")]
pub type DllClkConf = crate::Reg<dll_clk_conf::DllClkConfSpec>;
#[doc = "SDIO DLL clock control register."]
pub mod dll_clk_conf;
#[doc = "DLL_CONF (rw) register accessor: SDIO DLL configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_conf`] module"]
#[doc(alias = "DLL_CONF")]
pub type DllConf = crate::Reg<dll_conf::DllConfSpec>;
#[doc = "SDIO DLL configuration register."]
pub mod dll_conf;
