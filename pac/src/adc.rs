#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    ctrl2: Ctrl2,
    filter_ctrl1: FilterCtrl1,
    fsm_wait: FsmWait,
    sar1_status: Sar1Status,
    sar2_status: Sar2Status,
    sar1_patt_tab1: Sar1PattTab1,
    sar1_patt_tab2: Sar1PattTab2,
    sar1_patt_tab3: Sar1PattTab3,
    sar1_patt_tab4: Sar1PattTab4,
    sar2_patt_tab1: Sar2PattTab1,
    sar2_patt_tab2: Sar2PattTab2,
    sar2_patt_tab3: Sar2PattTab3,
    sar2_patt_tab4: Sar2PattTab4,
    arb_ctrl: ArbCtrl,
    filter_ctrl0: FilterCtrl0,
    sar1_data_status: Sar1DataStatus,
    thres0_ctrl: Thres0Ctrl,
    thres1_ctrl: Thres1Ctrl,
    thres_ctrl: ThresCtrl,
    int_ena: IntEna,
    int_raw: IntRaw,
    int_st: IntSt,
    int_clr: IntClr,
    dma_conf: DmaConf,
    sar2_data_status: Sar2DataStatus,
    cali: Cali,
    rnd_eco_low: RndEcoLow,
    rnd_eco_high: RndEcoHigh,
    rnd_eco_cs: RndEcoCs,
    _reserved30: [u8; 0x0384],
    ctrl_date: CtrlDate,
}
impl RegisterBlock {
    #[doc = "0x00 - Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Register"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x08 - Register"]
    #[inline(always)]
    pub const fn filter_ctrl1(&self) -> &FilterCtrl1 {
        &self.filter_ctrl1
    }
    #[doc = "0x0c - Register"]
    #[inline(always)]
    pub const fn fsm_wait(&self) -> &FsmWait {
        &self.fsm_wait
    }
    #[doc = "0x10 - Register"]
    #[inline(always)]
    pub const fn sar1_status(&self) -> &Sar1Status {
        &self.sar1_status
    }
    #[doc = "0x14 - Register"]
    #[inline(always)]
    pub const fn sar2_status(&self) -> &Sar2Status {
        &self.sar2_status
    }
    #[doc = "0x18 - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab1(&self) -> &Sar1PattTab1 {
        &self.sar1_patt_tab1
    }
    #[doc = "0x1c - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab2(&self) -> &Sar1PattTab2 {
        &self.sar1_patt_tab2
    }
    #[doc = "0x20 - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab3(&self) -> &Sar1PattTab3 {
        &self.sar1_patt_tab3
    }
    #[doc = "0x24 - Register"]
    #[inline(always)]
    pub const fn sar1_patt_tab4(&self) -> &Sar1PattTab4 {
        &self.sar1_patt_tab4
    }
    #[doc = "0x28 - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab1(&self) -> &Sar2PattTab1 {
        &self.sar2_patt_tab1
    }
    #[doc = "0x2c - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab2(&self) -> &Sar2PattTab2 {
        &self.sar2_patt_tab2
    }
    #[doc = "0x30 - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab3(&self) -> &Sar2PattTab3 {
        &self.sar2_patt_tab3
    }
    #[doc = "0x34 - Register"]
    #[inline(always)]
    pub const fn sar2_patt_tab4(&self) -> &Sar2PattTab4 {
        &self.sar2_patt_tab4
    }
    #[doc = "0x38 - Register"]
    #[inline(always)]
    pub const fn arb_ctrl(&self) -> &ArbCtrl {
        &self.arb_ctrl
    }
    #[doc = "0x3c - Register"]
    #[inline(always)]
    pub const fn filter_ctrl0(&self) -> &FilterCtrl0 {
        &self.filter_ctrl0
    }
    #[doc = "0x40 - Register"]
    #[inline(always)]
    pub const fn sar1_data_status(&self) -> &Sar1DataStatus {
        &self.sar1_data_status
    }
    #[doc = "0x44 - Register"]
    #[inline(always)]
    pub const fn thres0_ctrl(&self) -> &Thres0Ctrl {
        &self.thres0_ctrl
    }
    #[doc = "0x48 - Register"]
    #[inline(always)]
    pub const fn thres1_ctrl(&self) -> &Thres1Ctrl {
        &self.thres1_ctrl
    }
    #[doc = "0x4c - Register"]
    #[inline(always)]
    pub const fn thres_ctrl(&self) -> &ThresCtrl {
        &self.thres_ctrl
    }
    #[doc = "0x50 - Register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x54 - Register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x58 - Register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x5c - Register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x60 - Register"]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DmaConf {
        &self.dma_conf
    }
    #[doc = "0x64 - Register"]
    #[inline(always)]
    pub const fn sar2_data_status(&self) -> &Sar2DataStatus {
        &self.sar2_data_status
    }
    #[doc = "0x68 - Register"]
    #[inline(always)]
    pub const fn cali(&self) -> &Cali {
        &self.cali
    }
    #[doc = "0x6c - Register"]
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RndEcoLow {
        &self.rnd_eco_low
    }
    #[doc = "0x70 - Register"]
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RndEcoHigh {
        &self.rnd_eco_high
    }
    #[doc = "0x74 - Register"]
    #[inline(always)]
    pub const fn rnd_eco_cs(&self) -> &RndEcoCs {
        &self.rnd_eco_cs
    }
    #[doc = "0x3fc - Register"]
    #[inline(always)]
    pub const fn ctrl_date(&self) -> &CtrlDate {
        &self.ctrl_date
    }
}
#[doc = "CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Register"]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Register"]
pub mod ctrl2;
#[doc = "FILTER_CTRL1 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ctrl1`] module"]
#[doc(alias = "FILTER_CTRL1")]
pub type FilterCtrl1 = crate::Reg<filter_ctrl1::FilterCtrl1Spec>;
#[doc = "Register"]
pub mod filter_ctrl1;
#[doc = "FSM_WAIT (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_wait`] module"]
#[doc(alias = "FSM_WAIT")]
pub type FsmWait = crate::Reg<fsm_wait::FsmWaitSpec>;
#[doc = "Register"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_status`] module"]
#[doc(alias = "SAR1_STATUS")]
pub type Sar1Status = crate::Reg<sar1_status::Sar1StatusSpec>;
#[doc = "Register"]
pub mod sar1_status;
#[doc = "SAR2_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_status`] module"]
#[doc(alias = "SAR2_STATUS")]
pub type Sar2Status = crate::Reg<sar2_status::Sar2StatusSpec>;
#[doc = "Register"]
pub mod sar2_status;
#[doc = "SAR1_PATT_TAB1 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab1`] module"]
#[doc(alias = "SAR1_PATT_TAB1")]
pub type Sar1PattTab1 = crate::Reg<sar1_patt_tab1::Sar1PattTab1Spec>;
#[doc = "Register"]
pub mod sar1_patt_tab1;
#[doc = "SAR1_PATT_TAB2 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab2`] module"]
#[doc(alias = "SAR1_PATT_TAB2")]
pub type Sar1PattTab2 = crate::Reg<sar1_patt_tab2::Sar1PattTab2Spec>;
#[doc = "Register"]
pub mod sar1_patt_tab2;
#[doc = "SAR1_PATT_TAB3 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab3`] module"]
#[doc(alias = "SAR1_PATT_TAB3")]
pub type Sar1PattTab3 = crate::Reg<sar1_patt_tab3::Sar1PattTab3Spec>;
#[doc = "Register"]
pub mod sar1_patt_tab3;
#[doc = "SAR1_PATT_TAB4 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_patt_tab4`] module"]
#[doc(alias = "SAR1_PATT_TAB4")]
pub type Sar1PattTab4 = crate::Reg<sar1_patt_tab4::Sar1PattTab4Spec>;
#[doc = "Register"]
pub mod sar1_patt_tab4;
#[doc = "SAR2_PATT_TAB1 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab1`] module"]
#[doc(alias = "SAR2_PATT_TAB1")]
pub type Sar2PattTab1 = crate::Reg<sar2_patt_tab1::Sar2PattTab1Spec>;
#[doc = "Register"]
pub mod sar2_patt_tab1;
#[doc = "SAR2_PATT_TAB2 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab2`] module"]
#[doc(alias = "SAR2_PATT_TAB2")]
pub type Sar2PattTab2 = crate::Reg<sar2_patt_tab2::Sar2PattTab2Spec>;
#[doc = "Register"]
pub mod sar2_patt_tab2;
#[doc = "SAR2_PATT_TAB3 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab3`] module"]
#[doc(alias = "SAR2_PATT_TAB3")]
pub type Sar2PattTab3 = crate::Reg<sar2_patt_tab3::Sar2PattTab3Spec>;
#[doc = "Register"]
pub mod sar2_patt_tab3;
#[doc = "SAR2_PATT_TAB4 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_patt_tab4`] module"]
#[doc(alias = "SAR2_PATT_TAB4")]
pub type Sar2PattTab4 = crate::Reg<sar2_patt_tab4::Sar2PattTab4Spec>;
#[doc = "Register"]
pub mod sar2_patt_tab4;
#[doc = "ARB_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ctrl`] module"]
#[doc(alias = "ARB_CTRL")]
pub type ArbCtrl = crate::Reg<arb_ctrl::ArbCtrlSpec>;
#[doc = "Register"]
pub mod arb_ctrl;
#[doc = "FILTER_CTRL0 (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_ctrl0`] module"]
#[doc(alias = "FILTER_CTRL0")]
pub type FilterCtrl0 = crate::Reg<filter_ctrl0::FilterCtrl0Spec>;
#[doc = "Register"]
pub mod filter_ctrl0;
#[doc = "SAR1_DATA_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_data_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1_data_status`] module"]
#[doc(alias = "SAR1_DATA_STATUS")]
pub type Sar1DataStatus = crate::Reg<sar1_data_status::Sar1DataStatusSpec>;
#[doc = "Register"]
pub mod sar1_data_status;
#[doc = "THRES0_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres0_ctrl`] module"]
#[doc(alias = "THRES0_CTRL")]
pub type Thres0Ctrl = crate::Reg<thres0_ctrl::Thres0CtrlSpec>;
#[doc = "Register"]
pub mod thres0_ctrl;
#[doc = "THRES1_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres1_ctrl`] module"]
#[doc(alias = "THRES1_CTRL")]
pub type Thres1Ctrl = crate::Reg<thres1_ctrl::Thres1CtrlSpec>;
#[doc = "Register"]
pub mod thres1_ctrl;
#[doc = "THRES_CTRL (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres_ctrl`] module"]
#[doc(alias = "THRES_CTRL")]
pub type ThresCtrl = crate::Reg<thres_ctrl::ThresCtrlSpec>;
#[doc = "Register"]
pub mod thres_ctrl;
#[doc = "INT_ENA (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "Register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "Register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "Register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "Register"]
pub mod int_clr;
#[doc = "DMA_CONF (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
#[doc(alias = "DMA_CONF")]
pub type DmaConf = crate::Reg<dma_conf::DmaConfSpec>;
#[doc = "Register"]
pub mod dma_conf;
#[doc = "SAR2_DATA_STATUS (r) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_data_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2_data_status`] module"]
#[doc(alias = "SAR2_DATA_STATUS")]
pub type Sar2DataStatus = crate::Reg<sar2_data_status::Sar2DataStatusSpec>;
#[doc = "Register"]
pub mod sar2_data_status;
#[doc = "CALI (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali`] module"]
#[doc(alias = "CALI")]
pub type Cali = crate::Reg<cali::CaliSpec>;
#[doc = "Register"]
pub mod cali;
#[doc = "RND_ECO_LOW (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_low`] module"]
#[doc(alias = "RND_ECO_LOW")]
pub type RndEcoLow = crate::Reg<rnd_eco_low::RndEcoLowSpec>;
#[doc = "Register"]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_high`] module"]
#[doc(alias = "RND_ECO_HIGH")]
pub type RndEcoHigh = crate::Reg<rnd_eco_high::RndEcoHighSpec>;
#[doc = "Register"]
pub mod rnd_eco_high;
#[doc = "RND_ECO_CS (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_cs`] module"]
#[doc(alias = "RND_ECO_CS")]
pub type RndEcoCs = crate::Reg<rnd_eco_cs::RndEcoCsSpec>;
#[doc = "Register"]
pub mod rnd_eco_cs;
#[doc = "CTRL_DATE (rw) register accessor: Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_date`] module"]
#[doc(alias = "CTRL_DATE")]
pub type CtrlDate = crate::Reg<ctrl_date::CtrlDateSpec>;
#[doc = "Register"]
pub mod ctrl_date;
