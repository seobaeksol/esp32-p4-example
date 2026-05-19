#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_aonclkrst_lp_clk_conf: LpAonclkrstLpClkConf,
    lp_aonclkrst_lp_clk_po_en: LpAonclkrstLpClkPoEn,
    lp_aonclkrst_lp_clk_en: LpAonclkrstLpClkEn,
    lp_aonclkrst_lp_rst_en: LpAonclkrstLpRstEn,
    lp_aonclkrst_reset_cause: LpAonclkrstResetCause,
    lp_aonclkrst_hpcpu_reset_ctrl0: LpAonclkrstHpcpuResetCtrl0,
    lp_aonclkrst_hpcpu_reset_ctrl1: LpAonclkrstHpcpuResetCtrl1,
    lp_aonclkrst_fosc_cntl: LpAonclkrstFoscCntl,
    lp_aonclkrst_rc32k_cntl: LpAonclkrstRc32kCntl,
    lp_aonclkrst_sosc_cntl: LpAonclkrstSoscCntl,
    lp_aonclkrst_clk_to_hp: LpAonclkrstClkToHp,
    lp_aonclkrst_lpmem_force: LpAonclkrstLpmemForce,
    lp_aonclkrst_xtal32k: LpAonclkrstXtal32k,
    lp_aonclkrst_mux_hpsys_reset_bypass: LpAonclkrstMuxHpsysResetBypass,
    lp_aonclkrst_hpsys_0_reset_bypass: LpAonclkrstHpsys0ResetBypass,
    lp_aonclkrst_hpsys_apm_reset_bypass: LpAonclkrstHpsysApmResetBypass,
    lp_aonclkrst_hp_clk_ctrl: LpAonclkrstHpClkCtrl,
    lp_aonclkrst_hp_usb_clkrst_ctrl0: LpAonclkrstHpUsbClkrstCtrl0,
    lp_aonclkrst_hp_usb_clkrst_ctrl1: LpAonclkrstHpUsbClkrstCtrl1,
    lp_aonclkrst_hp_sdmmc_emac_rst_ctrl: LpAonclkrstHpSdmmcEmacRstCtrl,
    _reserved20: [u8; 0x03ac],
    lp_aonclkrst_date: LpAonclkrstDate,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_clk_conf(&self) -> &LpAonclkrstLpClkConf {
        &self.lp_aonclkrst_lp_clk_conf
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_clk_po_en(&self) -> &LpAonclkrstLpClkPoEn {
        &self.lp_aonclkrst_lp_clk_po_en
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_clk_en(&self) -> &LpAonclkrstLpClkEn {
        &self.lp_aonclkrst_lp_clk_en
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_rst_en(&self) -> &LpAonclkrstLpRstEn {
        &self.lp_aonclkrst_lp_rst_en
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_reset_cause(&self) -> &LpAonclkrstResetCause {
        &self.lp_aonclkrst_reset_cause
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcpu_reset_ctrl0(&self) -> &LpAonclkrstHpcpuResetCtrl0 {
        &self.lp_aonclkrst_hpcpu_reset_ctrl0
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcpu_reset_ctrl1(&self) -> &LpAonclkrstHpcpuResetCtrl1 {
        &self.lp_aonclkrst_hpcpu_reset_ctrl1
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_fosc_cntl(&self) -> &LpAonclkrstFoscCntl {
        &self.lp_aonclkrst_fosc_cntl
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_rc32k_cntl(&self) -> &LpAonclkrstRc32kCntl {
        &self.lp_aonclkrst_rc32k_cntl
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_sosc_cntl(&self) -> &LpAonclkrstSoscCntl {
        &self.lp_aonclkrst_sosc_cntl
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_to_hp(&self) -> &LpAonclkrstClkToHp {
        &self.lp_aonclkrst_clk_to_hp
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_lpmem_force(&self) -> &LpAonclkrstLpmemForce {
        &self.lp_aonclkrst_lpmem_force
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_xtal32k(&self) -> &LpAonclkrstXtal32k {
        &self.lp_aonclkrst_xtal32k
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_mux_hpsys_reset_bypass(&self) -> &LpAonclkrstMuxHpsysResetBypass {
        &self.lp_aonclkrst_mux_hpsys_reset_bypass
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpsys_0_reset_bypass(&self) -> &LpAonclkrstHpsys0ResetBypass {
        &self.lp_aonclkrst_hpsys_0_reset_bypass
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hpsys_apm_reset_bypass(&self) -> &LpAonclkrstHpsysApmResetBypass {
        &self.lp_aonclkrst_hpsys_apm_reset_bypass
    }
    #[doc = "0x40 - HP Clock Control Register."]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_clk_ctrl(&self) -> &LpAonclkrstHpClkCtrl {
        &self.lp_aonclkrst_hp_clk_ctrl
    }
    #[doc = "0x44 - HP USB Clock Reset Control Register."]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_usb_clkrst_ctrl0(&self) -> &LpAonclkrstHpUsbClkrstCtrl0 {
        &self.lp_aonclkrst_hp_usb_clkrst_ctrl0
    }
    #[doc = "0x48 - HP USB Clock Reset Control Register."]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_usb_clkrst_ctrl1(&self) -> &LpAonclkrstHpUsbClkrstCtrl1 {
        &self.lp_aonclkrst_hp_usb_clkrst_ctrl1
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_sdmmc_emac_rst_ctrl(&self) -> &LpAonclkrstHpSdmmcEmacRstCtrl {
        &self.lp_aonclkrst_hp_sdmmc_emac_rst_ctrl
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn lp_aonclkrst_date(&self) -> &LpAonclkrstDate {
        &self.lp_aonclkrst_date
    }
}
#[doc = "LP_AONCLKRST_LP_CLK_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_clk_conf`] module"]
#[doc(alias = "LP_AONCLKRST_LP_CLK_CONF")]
pub type LpAonclkrstLpClkConf = crate::Reg<lp_aonclkrst_lp_clk_conf::LpAonclkrstLpClkConfSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_clk_conf;
#[doc = "LP_AONCLKRST_LP_CLK_PO_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_clk_po_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_po_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_clk_po_en`] module"]
#[doc(alias = "LP_AONCLKRST_LP_CLK_PO_EN")]
pub type LpAonclkrstLpClkPoEn = crate::Reg<lp_aonclkrst_lp_clk_po_en::LpAonclkrstLpClkPoEnSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_clk_po_en;
#[doc = "LP_AONCLKRST_LP_CLK_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_clk_en`] module"]
#[doc(alias = "LP_AONCLKRST_LP_CLK_EN")]
pub type LpAonclkrstLpClkEn = crate::Reg<lp_aonclkrst_lp_clk_en::LpAonclkrstLpClkEnSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_clk_en;
#[doc = "LP_AONCLKRST_LP_RST_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_rst_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_rst_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lp_rst_en`] module"]
#[doc(alias = "LP_AONCLKRST_LP_RST_EN")]
pub type LpAonclkrstLpRstEn = crate::Reg<lp_aonclkrst_lp_rst_en::LpAonclkrstLpRstEnSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lp_rst_en;
#[doc = "LP_AONCLKRST_RESET_CAUSE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_reset_cause`] module"]
#[doc(alias = "LP_AONCLKRST_RESET_CAUSE")]
pub type LpAonclkrstResetCause = crate::Reg<lp_aonclkrst_reset_cause::LpAonclkrstResetCauseSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_reset_cause;
#[doc = "LP_AONCLKRST_HPCPU_RESET_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcpu_reset_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcpu_reset_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcpu_reset_ctrl0`] module"]
#[doc(alias = "LP_AONCLKRST_HPCPU_RESET_CTRL0")]
pub type LpAonclkrstHpcpuResetCtrl0 =
    crate::Reg<lp_aonclkrst_hpcpu_reset_ctrl0::LpAonclkrstHpcpuResetCtrl0Spec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcpu_reset_ctrl0;
#[doc = "LP_AONCLKRST_HPCPU_RESET_CTRL1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcpu_reset_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcpu_reset_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpcpu_reset_ctrl1`] module"]
#[doc(alias = "LP_AONCLKRST_HPCPU_RESET_CTRL1")]
pub type LpAonclkrstHpcpuResetCtrl1 =
    crate::Reg<lp_aonclkrst_hpcpu_reset_ctrl1::LpAonclkrstHpcpuResetCtrl1Spec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpcpu_reset_ctrl1;
#[doc = "LP_AONCLKRST_FOSC_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_fosc_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_fosc_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_fosc_cntl`] module"]
#[doc(alias = "LP_AONCLKRST_FOSC_CNTL")]
pub type LpAonclkrstFoscCntl = crate::Reg<lp_aonclkrst_fosc_cntl::LpAonclkrstFoscCntlSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_fosc_cntl;
#[doc = "LP_AONCLKRST_RC32K_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_rc32k_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_rc32k_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_rc32k_cntl`] module"]
#[doc(alias = "LP_AONCLKRST_RC32K_CNTL")]
pub type LpAonclkrstRc32kCntl = crate::Reg<lp_aonclkrst_rc32k_cntl::LpAonclkrstRc32kCntlSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_rc32k_cntl;
#[doc = "LP_AONCLKRST_SOSC_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_sosc_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_sosc_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_sosc_cntl`] module"]
#[doc(alias = "LP_AONCLKRST_SOSC_CNTL")]
pub type LpAonclkrstSoscCntl = crate::Reg<lp_aonclkrst_sosc_cntl::LpAonclkrstSoscCntlSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_sosc_cntl;
#[doc = "LP_AONCLKRST_CLK_TO_HP (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_clk_to_hp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_clk_to_hp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_clk_to_hp`] module"]
#[doc(alias = "LP_AONCLKRST_CLK_TO_HP")]
pub type LpAonclkrstClkToHp = crate::Reg<lp_aonclkrst_clk_to_hp::LpAonclkrstClkToHpSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_clk_to_hp;
#[doc = "LP_AONCLKRST_LPMEM_FORCE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lpmem_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lpmem_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_lpmem_force`] module"]
#[doc(alias = "LP_AONCLKRST_LPMEM_FORCE")]
pub type LpAonclkrstLpmemForce = crate::Reg<lp_aonclkrst_lpmem_force::LpAonclkrstLpmemForceSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_lpmem_force;
#[doc = "LP_AONCLKRST_XTAL32K (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_xtal32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_xtal32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_xtal32k`] module"]
#[doc(alias = "LP_AONCLKRST_XTAL32K")]
pub type LpAonclkrstXtal32k = crate::Reg<lp_aonclkrst_xtal32k::LpAonclkrstXtal32kSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_xtal32k;
#[doc = "LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_mux_hpsys_reset_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_mux_hpsys_reset_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_mux_hpsys_reset_bypass`] module"]
#[doc(alias = "LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS")]
pub type LpAonclkrstMuxHpsysResetBypass =
    crate::Reg<lp_aonclkrst_mux_hpsys_reset_bypass::LpAonclkrstMuxHpsysResetBypassSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_mux_hpsys_reset_bypass;
#[doc = "LP_AONCLKRST_HPSYS_0_RESET_BYPASS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpsys_0_reset_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpsys_0_reset_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpsys_0_reset_bypass`] module"]
#[doc(alias = "LP_AONCLKRST_HPSYS_0_RESET_BYPASS")]
pub type LpAonclkrstHpsys0ResetBypass =
    crate::Reg<lp_aonclkrst_hpsys_0_reset_bypass::LpAonclkrstHpsys0ResetBypassSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpsys_0_reset_bypass;
#[doc = "LP_AONCLKRST_HPSYS_APM_RESET_BYPASS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpsys_apm_reset_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpsys_apm_reset_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hpsys_apm_reset_bypass`] module"]
#[doc(alias = "LP_AONCLKRST_HPSYS_APM_RESET_BYPASS")]
pub type LpAonclkrstHpsysApmResetBypass =
    crate::Reg<lp_aonclkrst_hpsys_apm_reset_bypass::LpAonclkrstHpsysApmResetBypassSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hpsys_apm_reset_bypass;
#[doc = "LP_AONCLKRST_HP_CLK_CTRL (rw) register accessor: HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_clk_ctrl`] module"]
#[doc(alias = "LP_AONCLKRST_HP_CLK_CTRL")]
pub type LpAonclkrstHpClkCtrl = crate::Reg<lp_aonclkrst_hp_clk_ctrl::LpAonclkrstHpClkCtrlSpec>;
#[doc = "HP Clock Control Register."]
pub mod lp_aonclkrst_hp_clk_ctrl;
#[doc = "LP_AONCLKRST_HP_USB_CLKRST_CTRL0 (rw) register accessor: HP USB Clock Reset Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_usb_clkrst_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_usb_clkrst_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_usb_clkrst_ctrl0`] module"]
#[doc(alias = "LP_AONCLKRST_HP_USB_CLKRST_CTRL0")]
pub type LpAonclkrstHpUsbClkrstCtrl0 =
    crate::Reg<lp_aonclkrst_hp_usb_clkrst_ctrl0::LpAonclkrstHpUsbClkrstCtrl0Spec>;
#[doc = "HP USB Clock Reset Control Register."]
pub mod lp_aonclkrst_hp_usb_clkrst_ctrl0;
#[doc = "LP_AONCLKRST_HP_USB_CLKRST_CTRL1 (rw) register accessor: HP USB Clock Reset Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_usb_clkrst_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_usb_clkrst_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_usb_clkrst_ctrl1`] module"]
#[doc(alias = "LP_AONCLKRST_HP_USB_CLKRST_CTRL1")]
pub type LpAonclkrstHpUsbClkrstCtrl1 =
    crate::Reg<lp_aonclkrst_hp_usb_clkrst_ctrl1::LpAonclkrstHpUsbClkrstCtrl1Spec>;
#[doc = "HP USB Clock Reset Control Register."]
pub mod lp_aonclkrst_hp_usb_clkrst_ctrl1;
#[doc = "LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_hp_sdmmc_emac_rst_ctrl`] module"]
#[doc(alias = "LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL")]
pub type LpAonclkrstHpSdmmcEmacRstCtrl =
    crate::Reg<lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::LpAonclkrstHpSdmmcEmacRstCtrlSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_hp_sdmmc_emac_rst_ctrl;
#[doc = "LP_AONCLKRST_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_aonclkrst_date`] module"]
#[doc(alias = "LP_AONCLKRST_DATE")]
pub type LpAonclkrstDate = crate::Reg<lp_aonclkrst_date::LpAonclkrstDateSpec>;
#[doc = "need_des"]
pub mod lp_aonclkrst_date;
