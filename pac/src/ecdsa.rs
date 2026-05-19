#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    conf: Conf,
    clk: Clk,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    start: Start,
    state: State,
    result: Result,
    _reserved9: [u8; 0xd4],
    date: Date,
    _reserved10: [u8; 0x0100],
    sha_mode: ShaMode,
    _reserved11: [u8; 0x0c],
    sha_start: ShaStart,
    sha_continue: ShaContinue,
    sha_busy: ShaBusy,
    _reserved14: [u8; 0x64],
    message_mem: [MessageMem; 8],
    _reserved15: [u8; 0x0140],
    r_mem: [RMem; 8],
    _reserved16: [u8; 0x10],
    s_mem: [SMem; 8],
    _reserved17: [u8; 0x10],
    z_mem: [ZMem; 8],
    _reserved18: [u8; 0x10],
    qax_mem: [QaxMem; 8],
    _reserved19: [u8; 0x10],
    qay_mem: [QayMem; 8],
}
impl RegisterBlock {
    #[doc = "0x04 - ECDSA configure register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x08 - ECDSA clock gate register"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x0c - ECDSA interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x10 - ECDSA interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x14 - ECDSA interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x18 - ECDSA interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x1c - ECDSA start register"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x20 - ECDSA status register"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x24 - ECDSA result register"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x200 - ECDSA control SHA register"]
    #[inline(always)]
    pub const fn sha_mode(&self) -> &ShaMode {
        &self.sha_mode
    }
    #[doc = "0x210 - ECDSA control SHA register"]
    #[inline(always)]
    pub const fn sha_start(&self) -> &ShaStart {
        &self.sha_start
    }
    #[doc = "0x214 - ECDSA control SHA register"]
    #[inline(always)]
    pub const fn sha_continue(&self) -> &ShaContinue {
        &self.sha_continue
    }
    #[doc = "0x218 - ECDSA status register"]
    #[inline(always)]
    pub const fn sha_busy(&self) -> &ShaBusy {
        &self.sha_busy
    }
    #[doc = "0x280..0x2a0 - The memory that stores message."]
    #[inline(always)]
    pub const fn message_mem(&self, n: usize) -> &MessageMem {
        &self.message_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x2a0 - The memory that stores message."]
    #[inline(always)]
    pub fn message_mem_iter(&self) -> impl Iterator<Item = &MessageMem> {
        self.message_mem.iter()
    }
    #[doc = "0x3e0..0x400 - The memory that stores r."]
    #[inline(always)]
    pub const fn r_mem(&self, n: usize) -> &RMem {
        &self.r_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3e0..0x400 - The memory that stores r."]
    #[inline(always)]
    pub fn r_mem_iter(&self) -> impl Iterator<Item = &RMem> {
        self.r_mem.iter()
    }
    #[doc = "0x410..0x430 - The memory that stores s."]
    #[inline(always)]
    pub const fn s_mem(&self, n: usize) -> &SMem {
        &self.s_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x410..0x430 - The memory that stores s."]
    #[inline(always)]
    pub fn s_mem_iter(&self) -> impl Iterator<Item = &SMem> {
        self.s_mem.iter()
    }
    #[doc = "0x440..0x460 - The memory that stores software written z."]
    #[inline(always)]
    pub const fn z_mem(&self, n: usize) -> &ZMem {
        &self.z_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x440..0x460 - The memory that stores software written z."]
    #[inline(always)]
    pub fn z_mem_iter(&self) -> impl Iterator<Item = &ZMem> {
        self.z_mem.iter()
    }
    #[doc = "0x470..0x490 - The memory that stores x coordinates of QA or software written k."]
    #[inline(always)]
    pub const fn qax_mem(&self, n: usize) -> &QaxMem {
        &self.qax_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x470..0x490 - The memory that stores x coordinates of QA or software written k."]
    #[inline(always)]
    pub fn qax_mem_iter(&self) -> impl Iterator<Item = &QaxMem> {
        self.qax_mem.iter()
    }
    #[doc = "0x4a0..0x4c0 - The memory that stores y coordinates of QA."]
    #[inline(always)]
    pub const fn qay_mem(&self, n: usize) -> &QayMem {
        &self.qay_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4a0..0x4c0 - The memory that stores y coordinates of QA."]
    #[inline(always)]
    pub fn qay_mem_iter(&self) -> impl Iterator<Item = &QayMem> {
        self.qay_mem.iter()
    }
}
#[doc = "CONF (rw) register accessor: ECDSA configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "ECDSA configure register"]
pub mod conf;
#[doc = "CLK (rw) register accessor: ECDSA clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "ECDSA clock gate register"]
pub mod clk;
#[doc = "INT_RAW (r) register accessor: ECDSA interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "ECDSA interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: ECDSA interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "ECDSA interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: ECDSA interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "ECDSA interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: ECDSA interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "ECDSA interrupt clear register."]
pub mod int_clr;
#[doc = "START (w) register accessor: ECDSA start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "ECDSA start register"]
pub mod start;
#[doc = "STATE (r) register accessor: ECDSA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "ECDSA status register"]
pub mod state;
#[doc = "RESULT (r) register accessor: ECDSA result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "ECDSA result register"]
pub mod result;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control register"]
pub mod date;
#[doc = "SHA_MODE (rw) register accessor: ECDSA control SHA register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_mode`] module"]
#[doc(alias = "SHA_MODE")]
pub type ShaMode = crate::Reg<sha_mode::ShaModeSpec>;
#[doc = "ECDSA control SHA register"]
pub mod sha_mode;
#[doc = "SHA_START (w) register accessor: ECDSA control SHA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_start`] module"]
#[doc(alias = "SHA_START")]
pub type ShaStart = crate::Reg<sha_start::ShaStartSpec>;
#[doc = "ECDSA control SHA register"]
pub mod sha_start;
#[doc = "SHA_CONTINUE (w) register accessor: ECDSA control SHA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_continue`] module"]
#[doc(alias = "SHA_CONTINUE")]
pub type ShaContinue = crate::Reg<sha_continue::ShaContinueSpec>;
#[doc = "ECDSA control SHA register"]
pub mod sha_continue;
#[doc = "SHA_BUSY (r) register accessor: ECDSA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_busy`] module"]
#[doc(alias = "SHA_BUSY")]
pub type ShaBusy = crate::Reg<sha_busy::ShaBusySpec>;
#[doc = "ECDSA status register"]
pub mod sha_busy;
#[doc = "MESSAGE_MEM (rw) register accessor: The memory that stores message.\n\nYou can [`read`](crate::Reg::read) this register and get [`message_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`message_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@message_mem`] module"]
#[doc(alias = "MESSAGE_MEM")]
pub type MessageMem = crate::Reg<message_mem::MessageMemSpec>;
#[doc = "The memory that stores message."]
pub mod message_mem;
#[doc = "R_MEM (rw) register accessor: The memory that stores r.\n\nYou can [`read`](crate::Reg::read) this register and get [`r_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_mem`] module"]
#[doc(alias = "R_MEM")]
pub type RMem = crate::Reg<r_mem::RMemSpec>;
#[doc = "The memory that stores r."]
pub mod r_mem;
#[doc = "S_MEM (rw) register accessor: The memory that stores s.\n\nYou can [`read`](crate::Reg::read) this register and get [`s_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_mem`] module"]
#[doc(alias = "S_MEM")]
pub type SMem = crate::Reg<s_mem::SMemSpec>;
#[doc = "The memory that stores s."]
pub mod s_mem;
#[doc = "Z_MEM (rw) register accessor: The memory that stores software written z.\n\nYou can [`read`](crate::Reg::read) this register and get [`z_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@z_mem`] module"]
#[doc(alias = "Z_MEM")]
pub type ZMem = crate::Reg<z_mem::ZMemSpec>;
#[doc = "The memory that stores software written z."]
pub mod z_mem;
#[doc = "QAX_MEM (rw) register accessor: The memory that stores x coordinates of QA or software written k.\n\nYou can [`read`](crate::Reg::read) this register and get [`qax_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qax_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qax_mem`] module"]
#[doc(alias = "QAX_MEM")]
pub type QaxMem = crate::Reg<qax_mem::QaxMemSpec>;
#[doc = "The memory that stores x coordinates of QA or software written k."]
pub mod qax_mem;
#[doc = "QAY_MEM (rw) register accessor: The memory that stores y coordinates of QA.\n\nYou can [`read`](crate::Reg::read) this register and get [`qay_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qay_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qay_mem`] module"]
#[doc(alias = "QAY_MEM")]
pub type QayMem = crate::Reg<qay_mem::QayMemSpec>;
#[doc = "The memory that stores y coordinates of QA."]
pub mod qay_mem;
