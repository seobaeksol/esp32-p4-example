#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    _reserved4: [u8; 0x04],
    rx_conf: RxConf,
    tx_conf: TxConf,
    rx_conf1: RxConf1,
    tx_conf1: TxConf1,
    _reserved8: [u8; 0x10],
    tx_pcm2pdm_conf: TxPcm2pdmConf,
    tx_pcm2pdm_conf1: TxPcm2pdmConf1,
    rx_pdm2pcm_conf: RxPdm2pcmConf,
    _reserved11: [u8; 0x04],
    rx_tdm_ctrl: RxTdmCtrl,
    tx_tdm_ctrl: TxTdmCtrl,
    rx_timing: RxTiming,
    tx_timing: TxTiming,
    lc_hung_conf: LcHungConf,
    rxeof_num: RxeofNum,
    conf_sigle_data: ConfSigleData,
    state: State,
    etm_conf: EtmConf,
    fifo_cnt: FifoCnt,
    bck_cnt: BckCnt,
    clk_gate: ClkGate,
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x0c - I2S interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x10 - I2S interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x14 - I2S interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x18 - I2S interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x20 - I2S RX configure register"]
    #[inline(always)]
    pub const fn rx_conf(&self) -> &RxConf {
        &self.rx_conf
    }
    #[doc = "0x24 - I2S TX configure register"]
    #[inline(always)]
    pub const fn tx_conf(&self) -> &TxConf {
        &self.tx_conf
    }
    #[doc = "0x28 - I2S RX configure register 1"]
    #[inline(always)]
    pub const fn rx_conf1(&self) -> &RxConf1 {
        &self.rx_conf1
    }
    #[doc = "0x2c - I2S TX configure register 1"]
    #[inline(always)]
    pub const fn tx_conf1(&self) -> &TxConf1 {
        &self.tx_conf1
    }
    #[doc = "0x40 - I2S TX PCM2PDM configuration register"]
    #[inline(always)]
    pub const fn tx_pcm2pdm_conf(&self) -> &TxPcm2pdmConf {
        &self.tx_pcm2pdm_conf
    }
    #[doc = "0x44 - I2S TX PCM2PDM configuration register"]
    #[inline(always)]
    pub const fn tx_pcm2pdm_conf1(&self) -> &TxPcm2pdmConf1 {
        &self.tx_pcm2pdm_conf1
    }
    #[doc = "0x48 - I2S RX configure register"]
    #[inline(always)]
    pub const fn rx_pdm2pcm_conf(&self) -> &RxPdm2pcmConf {
        &self.rx_pdm2pcm_conf
    }
    #[doc = "0x50 - I2S TX TDM mode control register"]
    #[inline(always)]
    pub const fn rx_tdm_ctrl(&self) -> &RxTdmCtrl {
        &self.rx_tdm_ctrl
    }
    #[doc = "0x54 - I2S TX TDM mode control register"]
    #[inline(always)]
    pub const fn tx_tdm_ctrl(&self) -> &TxTdmCtrl {
        &self.tx_tdm_ctrl
    }
    #[doc = "0x58 - I2S RX timing control register"]
    #[inline(always)]
    pub const fn rx_timing(&self) -> &RxTiming {
        &self.rx_timing
    }
    #[doc = "0x5c - I2S TX timing control register"]
    #[inline(always)]
    pub const fn tx_timing(&self) -> &TxTiming {
        &self.tx_timing
    }
    #[doc = "0x60 - I2S HUNG configure register."]
    #[inline(always)]
    pub const fn lc_hung_conf(&self) -> &LcHungConf {
        &self.lc_hung_conf
    }
    #[doc = "0x64 - I2S RX data number control register."]
    #[inline(always)]
    pub const fn rxeof_num(&self) -> &RxeofNum {
        &self.rxeof_num
    }
    #[doc = "0x68 - I2S signal data register"]
    #[inline(always)]
    pub const fn conf_sigle_data(&self) -> &ConfSigleData {
        &self.conf_sigle_data
    }
    #[doc = "0x6c - I2S TX status register"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x70 - I2S ETM configure register"]
    #[inline(always)]
    pub const fn etm_conf(&self) -> &EtmConf {
        &self.etm_conf
    }
    #[doc = "0x74 - I2S sync counter register"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> &FifoCnt {
        &self.fifo_cnt
    }
    #[doc = "0x78 - I2S sync counter register"]
    #[inline(always)]
    pub const fn bck_cnt(&self) -> &BckCnt {
        &self.bck_cnt
    }
    #[doc = "0x7c - Clock gate register"]
    #[inline(always)]
    pub const fn clk_gate(&self) -> &ClkGate {
        &self.clk_gate
    }
    #[doc = "0x80 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "INT_RAW (r) register accessor: I2S interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "I2S interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: I2S interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "I2S interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: I2S interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "I2S interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: I2S interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "I2S interrupt clear register."]
pub mod int_clr;
#[doc = "RX_CONF (rw) register accessor: I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_conf`] module"]
#[doc(alias = "RX_CONF")]
pub type RxConf = crate::Reg<rx_conf::RxConfSpec>;
#[doc = "I2S RX configure register"]
pub mod rx_conf;
#[doc = "TX_CONF (rw) register accessor: I2S TX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_conf`] module"]
#[doc(alias = "TX_CONF")]
pub type TxConf = crate::Reg<tx_conf::TxConfSpec>;
#[doc = "I2S TX configure register"]
pub mod tx_conf;
#[doc = "RX_CONF1 (rw) register accessor: I2S RX configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_conf1`] module"]
#[doc(alias = "RX_CONF1")]
pub type RxConf1 = crate::Reg<rx_conf1::RxConf1Spec>;
#[doc = "I2S RX configure register 1"]
pub mod rx_conf1;
#[doc = "TX_CONF1 (rw) register accessor: I2S TX configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_conf1`] module"]
#[doc(alias = "TX_CONF1")]
pub type TxConf1 = crate::Reg<tx_conf1::TxConf1Spec>;
#[doc = "I2S TX configure register 1"]
pub mod tx_conf1;
#[doc = "TX_PCM2PDM_CONF (rw) register accessor: I2S TX PCM2PDM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pcm2pdm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm2pdm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_pcm2pdm_conf`] module"]
#[doc(alias = "TX_PCM2PDM_CONF")]
pub type TxPcm2pdmConf = crate::Reg<tx_pcm2pdm_conf::TxPcm2pdmConfSpec>;
#[doc = "I2S TX PCM2PDM configuration register"]
pub mod tx_pcm2pdm_conf;
#[doc = "TX_PCM2PDM_CONF1 (rw) register accessor: I2S TX PCM2PDM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pcm2pdm_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm2pdm_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_pcm2pdm_conf1`] module"]
#[doc(alias = "TX_PCM2PDM_CONF1")]
pub type TxPcm2pdmConf1 = crate::Reg<tx_pcm2pdm_conf1::TxPcm2pdmConf1Spec>;
#[doc = "I2S TX PCM2PDM configuration register"]
pub mod tx_pcm2pdm_conf1;
#[doc = "RX_PDM2PCM_CONF (rw) register accessor: I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pdm2pcm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_pdm2pcm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_pdm2pcm_conf`] module"]
#[doc(alias = "RX_PDM2PCM_CONF")]
pub type RxPdm2pcmConf = crate::Reg<rx_pdm2pcm_conf::RxPdm2pcmConfSpec>;
#[doc = "I2S RX configure register"]
pub mod rx_pdm2pcm_conf;
#[doc = "RX_TDM_CTRL (rw) register accessor: I2S TX TDM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_tdm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_tdm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_tdm_ctrl`] module"]
#[doc(alias = "RX_TDM_CTRL")]
pub type RxTdmCtrl = crate::Reg<rx_tdm_ctrl::RxTdmCtrlSpec>;
#[doc = "I2S TX TDM mode control register"]
pub mod rx_tdm_ctrl;
#[doc = "TX_TDM_CTRL (rw) register accessor: I2S TX TDM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_tdm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_tdm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_tdm_ctrl`] module"]
#[doc(alias = "TX_TDM_CTRL")]
pub type TxTdmCtrl = crate::Reg<tx_tdm_ctrl::TxTdmCtrlSpec>;
#[doc = "I2S TX TDM mode control register"]
pub mod tx_tdm_ctrl;
#[doc = "RX_TIMING (rw) register accessor: I2S RX timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_timing`] module"]
#[doc(alias = "RX_TIMING")]
pub type RxTiming = crate::Reg<rx_timing::RxTimingSpec>;
#[doc = "I2S RX timing control register"]
pub mod rx_timing;
#[doc = "TX_TIMING (rw) register accessor: I2S TX timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_timing`] module"]
#[doc(alias = "TX_TIMING")]
pub type TxTiming = crate::Reg<tx_timing::TxTimingSpec>;
#[doc = "I2S TX timing control register"]
pub mod tx_timing;
#[doc = "LC_HUNG_CONF (rw) register accessor: I2S HUNG configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_hung_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc_hung_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_hung_conf`] module"]
#[doc(alias = "LC_HUNG_CONF")]
pub type LcHungConf = crate::Reg<lc_hung_conf::LcHungConfSpec>;
#[doc = "I2S HUNG configure register."]
pub mod lc_hung_conf;
#[doc = "RXEOF_NUM (rw) register accessor: I2S RX data number control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxeof_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxeof_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxeof_num`] module"]
#[doc(alias = "RXEOF_NUM")]
pub type RxeofNum = crate::Reg<rxeof_num::RxeofNumSpec>;
#[doc = "I2S RX data number control register."]
pub mod rxeof_num;
#[doc = "CONF_SIGLE_DATA (rw) register accessor: I2S signal data register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf_sigle_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf_sigle_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf_sigle_data`] module"]
#[doc(alias = "CONF_SIGLE_DATA")]
pub type ConfSigleData = crate::Reg<conf_sigle_data::ConfSigleDataSpec>;
#[doc = "I2S signal data register"]
pub mod conf_sigle_data;
#[doc = "STATE (r) register accessor: I2S TX status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "I2S TX status register"]
pub mod state;
#[doc = "ETM_CONF (rw) register accessor: I2S ETM configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_conf`] module"]
#[doc(alias = "ETM_CONF")]
pub type EtmConf = crate::Reg<etm_conf::EtmConfSpec>;
#[doc = "I2S ETM configure register"]
pub mod etm_conf;
#[doc = "FIFO_CNT (rw) register accessor: I2S sync counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_cnt`] module"]
#[doc(alias = "FIFO_CNT")]
pub type FifoCnt = crate::Reg<fifo_cnt::FifoCntSpec>;
#[doc = "I2S sync counter register"]
pub mod fifo_cnt;
#[doc = "BCK_CNT (rw) register accessor: I2S sync counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bck_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bck_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bck_cnt`] module"]
#[doc(alias = "BCK_CNT")]
pub type BckCnt = crate::Reg<bck_cnt::BckCntSpec>;
#[doc = "I2S sync counter register"]
pub mod bck_cnt;
#[doc = "CLK_GATE (rw) register accessor: Clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gate`] module"]
#[doc(alias = "CLK_GATE")]
pub type ClkGate = crate::Reg<clk_gate::ClkGateSpec>;
#[doc = "Clock gate register"]
pub mod clk_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control register"]
pub mod date;
