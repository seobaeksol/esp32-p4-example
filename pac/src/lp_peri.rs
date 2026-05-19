#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en: ClkEn,
    core_clk_sel: CoreClkSel,
    reset_en: ResetEn,
    cpu: Cpu,
    _reserved4: [u8; 0x18],
    mem_ctrl: MemCtrl,
    adc_ctrl: AdcCtrl,
    lp_i2s_rxclk_div_num: LpI2sRxclkDivNum,
    lp_i2s_rxclk_div_xyz: LpI2sRxclkDivXyz,
    lp_i2s_txclk_div_num: LpI2sTxclkDivNum,
    lp_i2s_txclk_div_xyz: LpI2sTxclkDivXyz,
    _reserved10: [u8; 0x03bc],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &ClkEn {
        &self.clk_en
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn core_clk_sel(&self) -> &CoreClkSel {
        &self.core_clk_sel
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn reset_en(&self) -> &ResetEn {
        &self.reset_en
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn cpu(&self) -> &Cpu {
        &self.cpu
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn mem_ctrl(&self) -> &MemCtrl {
        &self.mem_ctrl
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn adc_ctrl(&self) -> &AdcCtrl {
        &self.adc_ctrl
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_rxclk_div_num(&self) -> &LpI2sRxclkDivNum {
        &self.lp_i2s_rxclk_div_num
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_rxclk_div_xyz(&self) -> &LpI2sRxclkDivXyz {
        &self.lp_i2s_rxclk_div_xyz
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_txclk_div_num(&self) -> &LpI2sTxclkDivNum {
        &self.lp_i2s_txclk_div_num
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_i2s_txclk_div_xyz(&self) -> &LpI2sTxclkDivXyz {
        &self.lp_i2s_txclk_div_xyz
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "CLK_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
#[doc(alias = "CLK_EN")]
pub type ClkEn = crate::Reg<clk_en::ClkEnSpec>;
#[doc = "need_des"]
pub mod clk_en;
#[doc = "CORE_CLK_SEL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_clk_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_clk_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_clk_sel`] module"]
#[doc(alias = "CORE_CLK_SEL")]
pub type CoreClkSel = crate::Reg<core_clk_sel::CoreClkSelSpec>;
#[doc = "need_des"]
pub mod core_clk_sel;
#[doc = "RESET_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_en`] module"]
#[doc(alias = "RESET_EN")]
pub type ResetEn = crate::Reg<reset_en::ResetEnSpec>;
#[doc = "need_des"]
pub mod reset_en;
#[doc = "CPU (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu`] module"]
#[doc(alias = "CPU")]
pub type Cpu = crate::Reg<cpu::CpuSpec>;
#[doc = "need_des"]
pub mod cpu;
#[doc = "MEM_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ctrl`] module"]
#[doc(alias = "MEM_CTRL")]
pub type MemCtrl = crate::Reg<mem_ctrl::MemCtrlSpec>;
#[doc = "need_des"]
pub mod mem_ctrl;
#[doc = "ADC_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ctrl`] module"]
#[doc(alias = "ADC_CTRL")]
pub type AdcCtrl = crate::Reg<adc_ctrl::AdcCtrlSpec>;
#[doc = "need_des"]
pub mod adc_ctrl;
#[doc = "LP_I2S_RXCLK_DIV_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2s_rxclk_div_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2s_rxclk_div_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_rxclk_div_num`] module"]
#[doc(alias = "LP_I2S_RXCLK_DIV_NUM")]
pub type LpI2sRxclkDivNum = crate::Reg<lp_i2s_rxclk_div_num::LpI2sRxclkDivNumSpec>;
#[doc = "need_des"]
pub mod lp_i2s_rxclk_div_num;
#[doc = "LP_I2S_RXCLK_DIV_XYZ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2s_rxclk_div_xyz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2s_rxclk_div_xyz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_rxclk_div_xyz`] module"]
#[doc(alias = "LP_I2S_RXCLK_DIV_XYZ")]
pub type LpI2sRxclkDivXyz = crate::Reg<lp_i2s_rxclk_div_xyz::LpI2sRxclkDivXyzSpec>;
#[doc = "need_des"]
pub mod lp_i2s_rxclk_div_xyz;
#[doc = "LP_I2S_TXCLK_DIV_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2s_txclk_div_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2s_txclk_div_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_txclk_div_num`] module"]
#[doc(alias = "LP_I2S_TXCLK_DIV_NUM")]
pub type LpI2sTxclkDivNum = crate::Reg<lp_i2s_txclk_div_num::LpI2sTxclkDivNumSpec>;
#[doc = "need_des"]
pub mod lp_i2s_txclk_div_num;
#[doc = "LP_I2S_TXCLK_DIV_XYZ (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2s_txclk_div_xyz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2s_txclk_div_xyz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_txclk_div_xyz`] module"]
#[doc(alias = "LP_I2S_TXCLK_DIV_XYZ")]
pub type LpI2sTxclkDivXyz = crate::Reg<lp_i2s_txclk_div_xyz::LpI2sTxclkDivXyzSpec>;
#[doc = "need_des"]
pub mod lp_i2s_txclk_div_xyz;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "need_des"]
pub mod date;
