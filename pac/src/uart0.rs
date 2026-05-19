#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fifo: Fifo,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    clkdiv: Clkdiv,
    rx_filt: RxFilt,
    status: Status,
    conf0: Conf0,
    conf1: Conf1,
    _reserved10: [u8; 0x04],
    hwfc_conf: HwfcConf,
    sleep_conf0: SleepConf0,
    sleep_conf1: SleepConf1,
    sleep_conf2: SleepConf2,
    swfc_conf0: SwfcConf0,
    swfc_conf1: SwfcConf1,
    txbrk_conf: TxbrkConf,
    idle_conf: IdleConf,
    rs485_conf: Rs485Conf,
    at_cmd_precnt: AtCmdPrecnt,
    at_cmd_postcnt: AtCmdPostcnt,
    at_cmd_gaptout: AtCmdGaptout,
    at_cmd_char: AtCmdChar,
    mem_conf: MemConf,
    tout_conf: ToutConf,
    mem_tx_status: MemTxStatus,
    mem_rx_status: MemRxStatus,
    fsm_status: FsmStatus,
    pospulse: Pospulse,
    negpulse: Negpulse,
    lowpulse: Lowpulse,
    highpulse: Highpulse,
    rxd_cnt: RxdCnt,
    clk_conf: ClkConf,
    date: Date,
    afifo_status: AfifoStatus,
    _reserved36: [u8; 0x04],
    reg_update: RegUpdate,
    id: Id,
}
impl RegisterBlock {
    #[doc = "0x00 - FIFO data register"]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
    #[doc = "0x04 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x08 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x0c - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x10 - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x14 - Clock divider configuration"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x18 - Rx Filter configuration"]
    #[inline(always)]
    pub const fn rx_filt(&self) -> &RxFilt {
        &self.rx_filt
    }
    #[doc = "0x1c - UART status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x20 - a"]
    #[inline(always)]
    pub const fn conf0(&self) -> &Conf0 {
        &self.conf0
    }
    #[doc = "0x24 - Configuration register 1"]
    #[inline(always)]
    pub const fn conf1(&self) -> &Conf1 {
        &self.conf1
    }
    #[doc = "0x2c - Hardware flow-control configuration"]
    #[inline(always)]
    pub const fn hwfc_conf(&self) -> &HwfcConf {
        &self.hwfc_conf
    }
    #[doc = "0x30 - UART sleep configure register 0"]
    #[inline(always)]
    pub const fn sleep_conf0(&self) -> &SleepConf0 {
        &self.sleep_conf0
    }
    #[doc = "0x34 - UART sleep configure register 1"]
    #[inline(always)]
    pub const fn sleep_conf1(&self) -> &SleepConf1 {
        &self.sleep_conf1
    }
    #[doc = "0x38 - UART sleep configure register 2"]
    #[inline(always)]
    pub const fn sleep_conf2(&self) -> &SleepConf2 {
        &self.sleep_conf2
    }
    #[doc = "0x3c - Software flow-control character configuration"]
    #[inline(always)]
    pub const fn swfc_conf0(&self) -> &SwfcConf0 {
        &self.swfc_conf0
    }
    #[doc = "0x40 - Software flow-control character configuration"]
    #[inline(always)]
    pub const fn swfc_conf1(&self) -> &SwfcConf1 {
        &self.swfc_conf1
    }
    #[doc = "0x44 - Tx Break character configuration"]
    #[inline(always)]
    pub const fn txbrk_conf(&self) -> &TxbrkConf {
        &self.txbrk_conf
    }
    #[doc = "0x48 - Frame-end idle configuration"]
    #[inline(always)]
    pub const fn idle_conf(&self) -> &IdleConf {
        &self.idle_conf
    }
    #[doc = "0x4c - RS485 mode configuration"]
    #[inline(always)]
    pub const fn rs485_conf(&self) -> &Rs485Conf {
        &self.rs485_conf
    }
    #[doc = "0x50 - Pre-sequence timing configuration"]
    #[inline(always)]
    pub const fn at_cmd_precnt(&self) -> &AtCmdPrecnt {
        &self.at_cmd_precnt
    }
    #[doc = "0x54 - Post-sequence timing configuration"]
    #[inline(always)]
    pub const fn at_cmd_postcnt(&self) -> &AtCmdPostcnt {
        &self.at_cmd_postcnt
    }
    #[doc = "0x58 - Timeout configuration"]
    #[inline(always)]
    pub const fn at_cmd_gaptout(&self) -> &AtCmdGaptout {
        &self.at_cmd_gaptout
    }
    #[doc = "0x5c - AT escape sequence detection configuration"]
    #[inline(always)]
    pub const fn at_cmd_char(&self) -> &AtCmdChar {
        &self.at_cmd_char
    }
    #[doc = "0x60 - UART memory power configuration"]
    #[inline(always)]
    pub const fn mem_conf(&self) -> &MemConf {
        &self.mem_conf
    }
    #[doc = "0x64 - UART threshold and allocation configuration"]
    #[inline(always)]
    pub const fn tout_conf(&self) -> &ToutConf {
        &self.tout_conf
    }
    #[doc = "0x68 - Tx-SRAM write and read offset address."]
    #[inline(always)]
    pub const fn mem_tx_status(&self) -> &MemTxStatus {
        &self.mem_tx_status
    }
    #[doc = "0x6c - Rx-SRAM write and read offset address."]
    #[inline(always)]
    pub const fn mem_rx_status(&self) -> &MemRxStatus {
        &self.mem_rx_status
    }
    #[doc = "0x70 - UART transmit and receive status."]
    #[inline(always)]
    pub const fn fsm_status(&self) -> &FsmStatus {
        &self.fsm_status
    }
    #[doc = "0x74 - Autobaud high pulse register"]
    #[inline(always)]
    pub const fn pospulse(&self) -> &Pospulse {
        &self.pospulse
    }
    #[doc = "0x78 - Autobaud low pulse register"]
    #[inline(always)]
    pub const fn negpulse(&self) -> &Negpulse {
        &self.negpulse
    }
    #[doc = "0x7c - Autobaud minimum low pulse duration register"]
    #[inline(always)]
    pub const fn lowpulse(&self) -> &Lowpulse {
        &self.lowpulse
    }
    #[doc = "0x80 - Autobaud minimum high pulse duration register"]
    #[inline(always)]
    pub const fn highpulse(&self) -> &Highpulse {
        &self.highpulse
    }
    #[doc = "0x84 - Autobaud edge change count register"]
    #[inline(always)]
    pub const fn rxd_cnt(&self) -> &RxdCnt {
        &self.rxd_cnt
    }
    #[doc = "0x88 - UART core clock configuration"]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &ClkConf {
        &self.clk_conf
    }
    #[doc = "0x8c - UART Version register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x90 - UART AFIFO Status"]
    #[inline(always)]
    pub const fn afifo_status(&self) -> &AfifoStatus {
        &self.afifo_status
    }
    #[doc = "0x98 - UART Registers Configuration Update register"]
    #[inline(always)]
    pub const fn reg_update(&self) -> &RegUpdate {
        &self.reg_update
    }
    #[doc = "0x9c - UART ID register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
}
#[doc = "FIFO (rw) register accessor: FIFO data register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`] module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "FIFO data register"]
pub mod fifo;
#[doc = "INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CLKDIV (rw) register accessor: Clock divider configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock divider configuration"]
pub mod clkdiv;
#[doc = "RX_FILT (rw) register accessor: Rx Filter configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_filt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_filt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_filt`] module"]
#[doc(alias = "RX_FILT")]
pub type RxFilt = crate::Reg<rx_filt::RxFiltSpec>;
#[doc = "Rx Filter configuration"]
pub mod rx_filt;
#[doc = "STATUS (r) register accessor: UART status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "UART status register"]
pub mod status;
#[doc = "CONF0 (rw) register accessor: a\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
#[doc(alias = "CONF0")]
pub type Conf0 = crate::Reg<conf0::Conf0Spec>;
#[doc = "a"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
#[doc(alias = "CONF1")]
pub type Conf1 = crate::Reg<conf1::Conf1Spec>;
#[doc = "Configuration register 1"]
pub mod conf1;
#[doc = "HWFC_CONF (rw) register accessor: Hardware flow-control configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hwfc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwfc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwfc_conf`] module"]
#[doc(alias = "HWFC_CONF")]
pub type HwfcConf = crate::Reg<hwfc_conf::HwfcConfSpec>;
#[doc = "Hardware flow-control configuration"]
pub mod hwfc_conf;
#[doc = "SLEEP_CONF0 (rw) register accessor: UART sleep configure register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep_conf0`] module"]
#[doc(alias = "SLEEP_CONF0")]
pub type SleepConf0 = crate::Reg<sleep_conf0::SleepConf0Spec>;
#[doc = "UART sleep configure register 0"]
pub mod sleep_conf0;
#[doc = "SLEEP_CONF1 (rw) register accessor: UART sleep configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep_conf1`] module"]
#[doc(alias = "SLEEP_CONF1")]
pub type SleepConf1 = crate::Reg<sleep_conf1::SleepConf1Spec>;
#[doc = "UART sleep configure register 1"]
pub mod sleep_conf1;
#[doc = "SLEEP_CONF2 (rw) register accessor: UART sleep configure register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep_conf2`] module"]
#[doc(alias = "SLEEP_CONF2")]
pub type SleepConf2 = crate::Reg<sleep_conf2::SleepConf2Spec>;
#[doc = "UART sleep configure register 2"]
pub mod sleep_conf2;
#[doc = "SWFC_CONF0 (rw) register accessor: Software flow-control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swfc_conf0`] module"]
#[doc(alias = "SWFC_CONF0")]
pub type SwfcConf0 = crate::Reg<swfc_conf0::SwfcConf0Spec>;
#[doc = "Software flow-control character configuration"]
pub mod swfc_conf0;
#[doc = "SWFC_CONF1 (rw) register accessor: Software flow-control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swfc_conf1`] module"]
#[doc(alias = "SWFC_CONF1")]
pub type SwfcConf1 = crate::Reg<swfc_conf1::SwfcConf1Spec>;
#[doc = "Software flow-control character configuration"]
pub mod swfc_conf1;
#[doc = "TXBRK_CONF (rw) register accessor: Tx Break character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbrk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrk_conf`] module"]
#[doc(alias = "TXBRK_CONF")]
pub type TxbrkConf = crate::Reg<txbrk_conf::TxbrkConfSpec>;
#[doc = "Tx Break character configuration"]
pub mod txbrk_conf;
#[doc = "IDLE_CONF (rw) register accessor: Frame-end idle configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle_conf`] module"]
#[doc(alias = "IDLE_CONF")]
pub type IdleConf = crate::Reg<idle_conf::IdleConfSpec>;
#[doc = "Frame-end idle configuration"]
pub mod idle_conf;
#[doc = "RS485_CONF (rw) register accessor: RS485 mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485_conf`] module"]
#[doc(alias = "RS485_CONF")]
pub type Rs485Conf = crate::Reg<rs485_conf::Rs485ConfSpec>;
#[doc = "RS485 mode configuration"]
pub mod rs485_conf;
#[doc = "AT_CMD_PRECNT (rw) register accessor: Pre-sequence timing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_precnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_precnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_precnt`] module"]
#[doc(alias = "AT_CMD_PRECNT")]
pub type AtCmdPrecnt = crate::Reg<at_cmd_precnt::AtCmdPrecntSpec>;
#[doc = "Pre-sequence timing configuration"]
pub mod at_cmd_precnt;
#[doc = "AT_CMD_POSTCNT (rw) register accessor: Post-sequence timing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_postcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_postcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_postcnt`] module"]
#[doc(alias = "AT_CMD_POSTCNT")]
pub type AtCmdPostcnt = crate::Reg<at_cmd_postcnt::AtCmdPostcntSpec>;
#[doc = "Post-sequence timing configuration"]
pub mod at_cmd_postcnt;
#[doc = "AT_CMD_GAPTOUT (rw) register accessor: Timeout configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_gaptout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_gaptout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_gaptout`] module"]
#[doc(alias = "AT_CMD_GAPTOUT")]
pub type AtCmdGaptout = crate::Reg<at_cmd_gaptout::AtCmdGaptoutSpec>;
#[doc = "Timeout configuration"]
pub mod at_cmd_gaptout;
#[doc = "AT_CMD_CHAR (rw) register accessor: AT escape sequence detection configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_char`] module"]
#[doc(alias = "AT_CMD_CHAR")]
pub type AtCmdChar = crate::Reg<at_cmd_char::AtCmdCharSpec>;
#[doc = "AT escape sequence detection configuration"]
pub mod at_cmd_char;
#[doc = "MEM_CONF (rw) register accessor: UART memory power configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_conf`] module"]
#[doc(alias = "MEM_CONF")]
pub type MemConf = crate::Reg<mem_conf::MemConfSpec>;
#[doc = "UART memory power configuration"]
pub mod mem_conf;
#[doc = "TOUT_CONF (rw) register accessor: UART threshold and allocation configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tout_conf`] module"]
#[doc(alias = "TOUT_CONF")]
pub type ToutConf = crate::Reg<tout_conf::ToutConfSpec>;
#[doc = "UART threshold and allocation configuration"]
pub mod tout_conf;
#[doc = "MEM_TX_STATUS (r) register accessor: Tx-SRAM write and read offset address.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_tx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_tx_status`] module"]
#[doc(alias = "MEM_TX_STATUS")]
pub type MemTxStatus = crate::Reg<mem_tx_status::MemTxStatusSpec>;
#[doc = "Tx-SRAM write and read offset address."]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS (r) register accessor: Rx-SRAM write and read offset address.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rx_status`] module"]
#[doc(alias = "MEM_RX_STATUS")]
pub type MemRxStatus = crate::Reg<mem_rx_status::MemRxStatusSpec>;
#[doc = "Rx-SRAM write and read offset address."]
pub mod mem_rx_status;
#[doc = "FSM_STATUS (r) register accessor: UART transmit and receive status.\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_status`] module"]
#[doc(alias = "FSM_STATUS")]
pub type FsmStatus = crate::Reg<fsm_status::FsmStatusSpec>;
#[doc = "UART transmit and receive status."]
pub mod fsm_status;
#[doc = "POSPULSE (r) register accessor: Autobaud high pulse register\n\nYou can [`read`](crate::Reg::read) this register and get [`pospulse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pospulse`] module"]
#[doc(alias = "POSPULSE")]
pub type Pospulse = crate::Reg<pospulse::PospulseSpec>;
#[doc = "Autobaud high pulse register"]
pub mod pospulse;
#[doc = "NEGPULSE (r) register accessor: Autobaud low pulse register\n\nYou can [`read`](crate::Reg::read) this register and get [`negpulse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@negpulse`] module"]
#[doc(alias = "NEGPULSE")]
pub type Negpulse = crate::Reg<negpulse::NegpulseSpec>;
#[doc = "Autobaud low pulse register"]
pub mod negpulse;
#[doc = "LOWPULSE (r) register accessor: Autobaud minimum low pulse duration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpulse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpulse`] module"]
#[doc(alias = "LOWPULSE")]
pub type Lowpulse = crate::Reg<lowpulse::LowpulseSpec>;
#[doc = "Autobaud minimum low pulse duration register"]
pub mod lowpulse;
#[doc = "HIGHPULSE (r) register accessor: Autobaud minimum high pulse duration register\n\nYou can [`read`](crate::Reg::read) this register and get [`highpulse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@highpulse`] module"]
#[doc(alias = "HIGHPULSE")]
pub type Highpulse = crate::Reg<highpulse::HighpulseSpec>;
#[doc = "Autobaud minimum high pulse duration register"]
pub mod highpulse;
#[doc = "RXD_CNT (r) register accessor: Autobaud edge change count register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd_cnt`] module"]
#[doc(alias = "RXD_CNT")]
pub type RxdCnt = crate::Reg<rxd_cnt::RxdCntSpec>;
#[doc = "Autobaud edge change count register"]
pub mod rxd_cnt;
#[doc = "CLK_CONF (rw) register accessor: UART core clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
#[doc(alias = "CLK_CONF")]
pub type ClkConf = crate::Reg<clk_conf::ClkConfSpec>;
#[doc = "UART core clock configuration"]
pub mod clk_conf;
#[doc = "DATE (rw) register accessor: UART Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "UART Version register"]
pub mod date;
#[doc = "AFIFO_STATUS (r) register accessor: UART AFIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`afifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afifo_status`] module"]
#[doc(alias = "AFIFO_STATUS")]
pub type AfifoStatus = crate::Reg<afifo_status::AfifoStatusSpec>;
#[doc = "UART AFIFO Status"]
pub mod afifo_status;
#[doc = "REG_UPDATE (rw) register accessor: UART Registers Configuration Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_update`] module"]
#[doc(alias = "REG_UPDATE")]
pub type RegUpdate = crate::Reg<reg_update::RegUpdateSpec>;
#[doc = "UART Registers Configuration Update register"]
pub mod reg_update;
#[doc = "ID (rw) register accessor: UART ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`] module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "UART ID register"]
pub mod id;
