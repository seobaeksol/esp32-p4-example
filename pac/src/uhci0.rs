#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf0: Conf0,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    conf1: Conf1,
    state0: State0,
    state1: State1,
    escape_conf: EscapeConf,
    hung_conf: HungConf,
    ack_num: AckNum,
    rx_head: RxHead,
    quick_sent: QuickSent,
    reg_q: [RegQ; 7],
    esc_conf: [EscConf; 4],
    pkt_thres: PktThres,
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - UHCI Configuration Register0"]
    #[inline(always)]
    pub const fn conf0(&self) -> &Conf0 {
        &self.conf0
    }
    #[doc = "0x04 - UHCI Interrupt Raw Register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x08 - UHCI Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x0c - UHCI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x10 - UHCI Interrupt Clear Register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x14 - UHCI Configuration Register1"]
    #[inline(always)]
    pub const fn conf1(&self) -> &Conf1 {
        &self.conf1
    }
    #[doc = "0x18 - UHCI Receive Status Register"]
    #[inline(always)]
    pub const fn state0(&self) -> &State0 {
        &self.state0
    }
    #[doc = "0x1c - UHCI Transmit Status Register"]
    #[inline(always)]
    pub const fn state1(&self) -> &State1 {
        &self.state1
    }
    #[doc = "0x20 - UHCI Escapes Configuration Register0"]
    #[inline(always)]
    pub const fn escape_conf(&self) -> &EscapeConf {
        &self.escape_conf
    }
    #[doc = "0x24 - UHCI Hung Configuration Register0"]
    #[inline(always)]
    pub const fn hung_conf(&self) -> &HungConf {
        &self.hung_conf
    }
    #[doc = "0x28 - UHCI Ack Value Configuration Register0"]
    #[inline(always)]
    pub const fn ack_num(&self) -> &AckNum {
        &self.ack_num
    }
    #[doc = "0x2c - UHCI Head Register"]
    #[inline(always)]
    pub const fn rx_head(&self) -> &RxHead {
        &self.rx_head
    }
    #[doc = "0x30 - UCHI Quick send Register"]
    #[inline(always)]
    pub const fn quick_sent(&self) -> &QuickSent {
        &self.quick_sent
    }
    #[doc = "0x34..0x6c - Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1"]
    #[inline(always)]
    pub const fn reg_q(&self, n: usize) -> &RegQ {
        &self.reg_q[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x6c - Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1"]
    #[inline(always)]
    pub fn reg_q_iter(&self) -> impl Iterator<Item = &RegQ> {
        self.reg_q.iter()
    }
    #[doc = "0x6c..0x7c - UHCI Escapes Sequence Configuration Register%s"]
    #[inline(always)]
    pub const fn esc_conf(&self, n: usize) -> &EscConf {
        &self.esc_conf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6c..0x7c - UHCI Escapes Sequence Configuration Register%s"]
    #[inline(always)]
    pub fn esc_conf_iter(&self) -> impl Iterator<Item = &EscConf> {
        self.esc_conf.iter()
    }
    #[doc = "0x7c - UCHI Packet Length Configuration Register"]
    #[inline(always)]
    pub const fn pkt_thres(&self) -> &PktThres {
        &self.pkt_thres
    }
    #[doc = "0x80 - UHCI Version Register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "CONF0 (rw) register accessor: UHCI Configuration Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
#[doc(alias = "CONF0")]
pub type Conf0 = crate::Reg<conf0::Conf0Spec>;
#[doc = "UHCI Configuration Register0"]
pub mod conf0;
#[doc = "INT_RAW (rw) register accessor: UHCI Interrupt Raw Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "UHCI Interrupt Raw Register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: UHCI Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "UHCI Interrupt Status Register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: UHCI Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "UHCI Interrupt Enable Register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: UHCI Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "UHCI Interrupt Clear Register"]
pub mod int_clr;
#[doc = "CONF1 (rw) register accessor: UHCI Configuration Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
#[doc(alias = "CONF1")]
pub type Conf1 = crate::Reg<conf1::Conf1Spec>;
#[doc = "UHCI Configuration Register1"]
pub mod conf1;
#[doc = "STATE0 (r) register accessor: UHCI Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`state0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state0`] module"]
#[doc(alias = "STATE0")]
pub type State0 = crate::Reg<state0::State0Spec>;
#[doc = "UHCI Receive Status Register"]
pub mod state0;
#[doc = "STATE1 (r) register accessor: UHCI Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`state1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state1`] module"]
#[doc(alias = "STATE1")]
pub type State1 = crate::Reg<state1::State1Spec>;
#[doc = "UHCI Transmit Status Register"]
pub mod state1;
#[doc = "ESCAPE_CONF (rw) register accessor: UHCI Escapes Configuration Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`escape_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`escape_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@escape_conf`] module"]
#[doc(alias = "ESCAPE_CONF")]
pub type EscapeConf = crate::Reg<escape_conf::EscapeConfSpec>;
#[doc = "UHCI Escapes Configuration Register0"]
pub mod escape_conf;
#[doc = "HUNG_CONF (rw) register accessor: UHCI Hung Configuration Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`hung_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hung_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hung_conf`] module"]
#[doc(alias = "HUNG_CONF")]
pub type HungConf = crate::Reg<hung_conf::HungConfSpec>;
#[doc = "UHCI Hung Configuration Register0"]
pub mod hung_conf;
#[doc = "ACK_NUM (rw) register accessor: UHCI Ack Value Configuration Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`ack_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ack_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_num`] module"]
#[doc(alias = "ACK_NUM")]
pub type AckNum = crate::Reg<ack_num::AckNumSpec>;
#[doc = "UHCI Ack Value Configuration Register0"]
pub mod ack_num;
#[doc = "RX_HEAD (r) register accessor: UHCI Head Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_head::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_head`] module"]
#[doc(alias = "RX_HEAD")]
pub type RxHead = crate::Reg<rx_head::RxHeadSpec>;
#[doc = "UHCI Head Register"]
pub mod rx_head;
#[doc = "QUICK_SENT (rw) register accessor: UCHI Quick send Register\n\nYou can [`read`](crate::Reg::read) this register and get [`quick_sent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quick_sent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quick_sent`] module"]
#[doc(alias = "QUICK_SENT")]
pub type QuickSent = crate::Reg<quick_sent::QuickSentSpec>;
#[doc = "UCHI Quick send Register"]
pub mod quick_sent;
#[doc = "Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1"]
pub use self::reg_q::RegQ;
#[doc = r"Cluster"]
#[doc = "Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1"]
pub mod reg_q;
#[doc = "ESC_CONF (rw) register accessor: UHCI Escapes Sequence Configuration Register%s\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf`] module"]
#[doc(alias = "ESC_CONF")]
pub type EscConf = crate::Reg<esc_conf::EscConfSpec>;
#[doc = "UHCI Escapes Sequence Configuration Register%s"]
pub mod esc_conf;
#[doc = "PKT_THRES (rw) register accessor: UCHI Packet Length Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkt_thres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkt_thres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkt_thres`] module"]
#[doc(alias = "PKT_THRES")]
pub type PktThres = crate::Reg<pkt_thres::PktThresSpec>;
#[doc = "UCHI Packet Length Configuration Register"]
pub mod pkt_thres;
#[doc = "DATE (rw) register accessor: UHCI Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "UHCI Version Register"]
pub mod date;
