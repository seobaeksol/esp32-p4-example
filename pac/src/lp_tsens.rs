#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    ctrl2: Ctrl2,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    clk_conf: ClkConf,
    int_ena_w1ts: IntEnaW1ts,
    int_ena_w1tc: IntEnaW1tc,
    wakeup_ctrl: WakeupCtrl,
    sample_rate: SampleRate,
    rnd_eco_low: RndEcoLow,
    rnd_eco_high: RndEcoHigh,
    rnd_eco_cs: RndEcoCs,
}
impl RegisterBlock {
    #[doc = "0x00 - Tsens configuration."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Tsens configuration."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x08 - Tsens interrupt raw registers."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x0c - Tsens interrupt status registers."]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x10 - Tsens interrupt enable registers."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x14 - Tsens interrupt clear registers."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x18 - Tsens regbank configuration registers."]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &ClkConf {
        &self.clk_conf
    }
    #[doc = "0x1c - Tsens wakeup interrupt enable assert."]
    #[inline(always)]
    pub const fn int_ena_w1ts(&self) -> &IntEnaW1ts {
        &self.int_ena_w1ts
    }
    #[doc = "0x20 - Tsens wakeup interrupt enable deassert."]
    #[inline(always)]
    pub const fn int_ena_w1tc(&self) -> &IntEnaW1tc {
        &self.int_ena_w1tc
    }
    #[doc = "0x24 - Tsens wakeup control registers."]
    #[inline(always)]
    pub const fn wakeup_ctrl(&self) -> &WakeupCtrl {
        &self.wakeup_ctrl
    }
    #[doc = "0x28 - Hardware automatic sampling control registers."]
    #[inline(always)]
    pub const fn sample_rate(&self) -> &SampleRate {
        &self.sample_rate
    }
    #[doc = "0x2c - N/A"]
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RndEcoLow {
        &self.rnd_eco_low
    }
    #[doc = "0x30 - N/A"]
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RndEcoHigh {
        &self.rnd_eco_high
    }
    #[doc = "0x34 - N/A"]
    #[inline(always)]
    pub const fn rnd_eco_cs(&self) -> &RndEcoCs {
        &self.rnd_eco_cs
    }
}
#[doc = "CTRL (rw) register accessor: Tsens configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Tsens configuration."]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: Tsens configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Tsens configuration."]
pub mod ctrl2;
#[doc = "INT_RAW (rw) register accessor: Tsens interrupt raw registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "Tsens interrupt raw registers."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Tsens interrupt status registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "Tsens interrupt status registers."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Tsens interrupt enable registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "Tsens interrupt enable registers."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Tsens interrupt clear registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "Tsens interrupt clear registers."]
pub mod int_clr;
#[doc = "CLK_CONF (rw) register accessor: Tsens regbank configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
#[doc(alias = "CLK_CONF")]
pub type ClkConf = crate::Reg<clk_conf::ClkConfSpec>;
#[doc = "Tsens regbank configuration registers."]
pub mod clk_conf;
#[doc = "INT_ENA_W1TS (w) register accessor: Tsens wakeup interrupt enable assert.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_w1ts`] module"]
#[doc(alias = "INT_ENA_W1TS")]
pub type IntEnaW1ts = crate::Reg<int_ena_w1ts::IntEnaW1tsSpec>;
#[doc = "Tsens wakeup interrupt enable assert."]
pub mod int_ena_w1ts;
#[doc = "INT_ENA_W1TC (w) register accessor: Tsens wakeup interrupt enable deassert.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_w1tc`] module"]
#[doc(alias = "INT_ENA_W1TC")]
pub type IntEnaW1tc = crate::Reg<int_ena_w1tc::IntEnaW1tcSpec>;
#[doc = "Tsens wakeup interrupt enable deassert."]
pub mod int_ena_w1tc;
#[doc = "WAKEUP_CTRL (rw) register accessor: Tsens wakeup control registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_ctrl`] module"]
#[doc(alias = "WAKEUP_CTRL")]
pub type WakeupCtrl = crate::Reg<wakeup_ctrl::WakeupCtrlSpec>;
#[doc = "Tsens wakeup control registers."]
pub mod wakeup_ctrl;
#[doc = "SAMPLE_RATE (rw) register accessor: Hardware automatic sampling control registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_rate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_rate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_rate`] module"]
#[doc(alias = "SAMPLE_RATE")]
pub type SampleRate = crate::Reg<sample_rate::SampleRateSpec>;
#[doc = "Hardware automatic sampling control registers."]
pub mod sample_rate;
#[doc = "RND_ECO_LOW (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_low`] module"]
#[doc(alias = "RND_ECO_LOW")]
pub type RndEcoLow = crate::Reg<rnd_eco_low::RndEcoLowSpec>;
#[doc = "N/A"]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_high`] module"]
#[doc(alias = "RND_ECO_HIGH")]
pub type RndEcoHigh = crate::Reg<rnd_eco_high::RndEcoHighSpec>;
#[doc = "N/A"]
pub mod rnd_eco_high;
#[doc = "RND_ECO_CS (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_eco_cs`] module"]
#[doc(alias = "RND_ECO_CS")]
pub type RndEcoCs = crate::Reg<rnd_eco_cs::RndEcoCsSpec>;
#[doc = "N/A"]
pub mod rnd_eco_cs;
