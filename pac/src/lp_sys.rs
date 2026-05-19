#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_sys_ver_date: LpSysVerDate,
    clk_sel_ctrl: ClkSelCtrl,
    sys_ctrl: SysCtrl,
    lp_clk_ctrl: LpClkCtrl,
    lp_rst_ctrl: LpRstCtrl,
    _reserved5: [u8; 0x04],
    lp_core_boot_addr: LpCoreBootAddr,
    ext_wakeup1: ExtWakeup1,
    ext_wakeup1_status: ExtWakeup1Status,
    lp_tcm_pwr_ctrl: LpTcmPwrCtrl,
    boot_addr_hp_lp: BootAddrHpLp,
    lp_store0: LpStore0,
    lp_store1: LpStore1,
    lp_store2: LpStore2,
    lp_store3: LpStore3,
    lp_store4: LpStore4,
    lp_store5: LpStore5,
    lp_store6: LpStore6,
    lp_store7: LpStore7,
    lp_store8: LpStore8,
    lp_store9: LpStore9,
    lp_store10: LpStore10,
    lp_store11: LpStore11,
    lp_store12: LpStore12,
    lp_store13: LpStore13,
    lp_store14: LpStore14,
    lp_store15: LpStore15,
    lp_probea_ctrl: LpProbeaCtrl,
    lp_probeb_ctrl: LpProbebCtrl,
    lp_probe_out: LpProbeOut,
    _reserved29: [u8; 0x24],
    f2s_apb_brg_cntl: F2sApbBrgCntl,
    _reserved30: [u8; 0x60],
    usb_ctrl: UsbCtrl,
    _reserved31: [u8; 0x08],
    ana_xpd_pad_group: AnaXpdPadGroup,
    lp_tcm_ram_rdn_eco_cs: LpTcmRamRdnEcoCs,
    lp_tcm_ram_rdn_eco_low: LpTcmRamRdnEcoLow,
    lp_tcm_ram_rdn_eco_high: LpTcmRamRdnEcoHigh,
    lp_tcm_rom_rdn_eco_cs: LpTcmRomRdnEcoCs,
    lp_tcm_rom_rdn_eco_low: LpTcmRomRdnEcoLow,
    lp_tcm_rom_rdn_eco_high: LpTcmRomRdnEcoHigh,
    _reserved38: [u8; 0x08],
    hp_root_clk_ctrl: HpRootClkCtrl,
    _reserved39: [u8; 0x04],
    lp_pmu_rdn_eco_low: LpPmuRdnEcoLow,
    lp_pmu_rdn_eco_high: LpPmuRdnEcoHigh,
    _reserved41: [u8; 0x08],
    pad_comp0: PadComp0,
    pad_comp1: PadComp1,
    _reserved43: [u8; 0x04],
    backup_dma_cfg0: BackupDmaCfg0,
    backup_dma_cfg1: BackupDmaCfg1,
    backup_dma_cfg2: BackupDmaCfg2,
    _reserved46: [u8; 0x04],
    boot_addr_hp_core1: BootAddrHpCore1,
    lp_addrhole_addr: LpAddrholeAddr,
    lp_addrhole_info: LpAddrholeInfo,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    hp_mem_aux_ctrl: HpMemAuxCtrl,
    lp_mem_aux_ctrl: LpMemAuxCtrl,
    hp_rom_aux_ctrl: HpRomAuxCtrl,
    lp_rom_aux_ctrl: LpRomAuxCtrl,
    lp_cpu_dbg_pc: LpCpuDbgPc,
    lp_cpu_exc_pc: LpCpuExcPc,
    idbus_addrhole_addr: IdbusAddrholeAddr,
    idbus_addrhole_info: IdbusAddrholeInfo,
    hp_por_rst_bypass_ctrl: HpPorRstBypassCtrl,
    rng_data: RngData,
    _reserved63: [u8; 0x08],
    lp_core_ahb_timeout: LpCoreAhbTimeout,
    lp_core_ibus_timeout: LpCoreIbusTimeout,
    lp_core_dbus_timeout: LpCoreDbusTimeout,
    lp_core_err_resp_dis: LpCoreErrRespDis,
    rng_cfg: RngCfg,
    pad_rtc_hold_ctrl0: PadRtcHoldCtrl0,
    pad_rtc_hold_ctrl1: PadRtcHoldCtrl1,
    ded_pad_rtc_hold_ctrl: DedPadRtcHoldCtrl,
    _reserved71: [u8; 0x30],
    discharge: Discharge,
    hp_usb_otghs_phy_ctrl: HpUsbOtghsPhyCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn lp_sys_ver_date(&self) -> &LpSysVerDate {
        &self.lp_sys_ver_date
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn clk_sel_ctrl(&self) -> &ClkSelCtrl {
        &self.clk_sel_ctrl
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SysCtrl {
        &self.sys_ctrl
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn lp_clk_ctrl(&self) -> &LpClkCtrl {
        &self.lp_clk_ctrl
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn lp_rst_ctrl(&self) -> &LpRstCtrl {
        &self.lp_rst_ctrl
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn lp_core_boot_addr(&self) -> &LpCoreBootAddr {
        &self.lp_core_boot_addr
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup1(&self) -> &ExtWakeup1 {
        &self.ext_wakeup1
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup1_status(&self) -> &ExtWakeup1Status {
        &self.ext_wakeup1_status
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_pwr_ctrl(&self) -> &LpTcmPwrCtrl {
        &self.lp_tcm_pwr_ctrl
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn boot_addr_hp_lp(&self) -> &BootAddrHpLp {
        &self.boot_addr_hp_lp
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn lp_store0(&self) -> &LpStore0 {
        &self.lp_store0
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_store1(&self) -> &LpStore1 {
        &self.lp_store1
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_store2(&self) -> &LpStore2 {
        &self.lp_store2
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_store3(&self) -> &LpStore3 {
        &self.lp_store3
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_store4(&self) -> &LpStore4 {
        &self.lp_store4
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn lp_store5(&self) -> &LpStore5 {
        &self.lp_store5
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn lp_store6(&self) -> &LpStore6 {
        &self.lp_store6
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn lp_store7(&self) -> &LpStore7 {
        &self.lp_store7
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn lp_store8(&self) -> &LpStore8 {
        &self.lp_store8
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn lp_store9(&self) -> &LpStore9 {
        &self.lp_store9
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn lp_store10(&self) -> &LpStore10 {
        &self.lp_store10
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn lp_store11(&self) -> &LpStore11 {
        &self.lp_store11
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn lp_store12(&self) -> &LpStore12 {
        &self.lp_store12
    }
    #[doc = "0x60 - need_des"]
    #[inline(always)]
    pub const fn lp_store13(&self) -> &LpStore13 {
        &self.lp_store13
    }
    #[doc = "0x64 - need_des"]
    #[inline(always)]
    pub const fn lp_store14(&self) -> &LpStore14 {
        &self.lp_store14
    }
    #[doc = "0x68 - need_des"]
    #[inline(always)]
    pub const fn lp_store15(&self) -> &LpStore15 {
        &self.lp_store15
    }
    #[doc = "0x6c - need_des"]
    #[inline(always)]
    pub const fn lp_probea_ctrl(&self) -> &LpProbeaCtrl {
        &self.lp_probea_ctrl
    }
    #[doc = "0x70 - need_des"]
    #[inline(always)]
    pub const fn lp_probeb_ctrl(&self) -> &LpProbebCtrl {
        &self.lp_probeb_ctrl
    }
    #[doc = "0x74 - need_des"]
    #[inline(always)]
    pub const fn lp_probe_out(&self) -> &LpProbeOut {
        &self.lp_probe_out
    }
    #[doc = "0x9c - need_des"]
    #[inline(always)]
    pub const fn f2s_apb_brg_cntl(&self) -> &F2sApbBrgCntl {
        &self.f2s_apb_brg_cntl
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn usb_ctrl(&self) -> &UsbCtrl {
        &self.usb_ctrl
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn ana_xpd_pad_group(&self) -> &AnaXpdPadGroup {
        &self.ana_xpd_pad_group
    }
    #[doc = "0x110 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_cs(&self) -> &LpTcmRamRdnEcoCs {
        &self.lp_tcm_ram_rdn_eco_cs
    }
    #[doc = "0x114 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_low(&self) -> &LpTcmRamRdnEcoLow {
        &self.lp_tcm_ram_rdn_eco_low
    }
    #[doc = "0x118 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_high(&self) -> &LpTcmRamRdnEcoHigh {
        &self.lp_tcm_ram_rdn_eco_high
    }
    #[doc = "0x11c - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_cs(&self) -> &LpTcmRomRdnEcoCs {
        &self.lp_tcm_rom_rdn_eco_cs
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_low(&self) -> &LpTcmRomRdnEcoLow {
        &self.lp_tcm_rom_rdn_eco_low
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_high(&self) -> &LpTcmRomRdnEcoHigh {
        &self.lp_tcm_rom_rdn_eco_high
    }
    #[doc = "0x130 - need_des"]
    #[inline(always)]
    pub const fn hp_root_clk_ctrl(&self) -> &HpRootClkCtrl {
        &self.hp_root_clk_ctrl
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn lp_pmu_rdn_eco_low(&self) -> &LpPmuRdnEcoLow {
        &self.lp_pmu_rdn_eco_low
    }
    #[doc = "0x13c - need_des"]
    #[inline(always)]
    pub const fn lp_pmu_rdn_eco_high(&self) -> &LpPmuRdnEcoHigh {
        &self.lp_pmu_rdn_eco_high
    }
    #[doc = "0x148 - need_des"]
    #[inline(always)]
    pub const fn pad_comp0(&self) -> &PadComp0 {
        &self.pad_comp0
    }
    #[doc = "0x14c - need_des"]
    #[inline(always)]
    pub const fn pad_comp1(&self) -> &PadComp1 {
        &self.pad_comp1
    }
    #[doc = "0x154 - need_des"]
    #[inline(always)]
    pub const fn backup_dma_cfg0(&self) -> &BackupDmaCfg0 {
        &self.backup_dma_cfg0
    }
    #[doc = "0x158 - need_des"]
    #[inline(always)]
    pub const fn backup_dma_cfg1(&self) -> &BackupDmaCfg1 {
        &self.backup_dma_cfg1
    }
    #[doc = "0x15c - need_des"]
    #[inline(always)]
    pub const fn backup_dma_cfg2(&self) -> &BackupDmaCfg2 {
        &self.backup_dma_cfg2
    }
    #[doc = "0x164 - need_des"]
    #[inline(always)]
    pub const fn boot_addr_hp_core1(&self) -> &BootAddrHpCore1 {
        &self.boot_addr_hp_core1
    }
    #[doc = "0x168 - need_des"]
    #[inline(always)]
    pub const fn lp_addrhole_addr(&self) -> &LpAddrholeAddr {
        &self.lp_addrhole_addr
    }
    #[doc = "0x16c - need_des"]
    #[inline(always)]
    pub const fn lp_addrhole_info(&self) -> &LpAddrholeInfo {
        &self.lp_addrhole_info
    }
    #[doc = "0x170 - raw interrupt register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x174 - masked interrupt register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x178 - masked interrupt register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x17c - interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn hp_mem_aux_ctrl(&self) -> &HpMemAuxCtrl {
        &self.hp_mem_aux_ctrl
    }
    #[doc = "0x184 - need_des"]
    #[inline(always)]
    pub const fn lp_mem_aux_ctrl(&self) -> &LpMemAuxCtrl {
        &self.lp_mem_aux_ctrl
    }
    #[doc = "0x188 - need_des"]
    #[inline(always)]
    pub const fn hp_rom_aux_ctrl(&self) -> &HpRomAuxCtrl {
        &self.hp_rom_aux_ctrl
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn lp_rom_aux_ctrl(&self) -> &LpRomAuxCtrl {
        &self.lp_rom_aux_ctrl
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_dbg_pc(&self) -> &LpCpuDbgPc {
        &self.lp_cpu_dbg_pc
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_exc_pc(&self) -> &LpCpuExcPc {
        &self.lp_cpu_exc_pc
    }
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn idbus_addrhole_addr(&self) -> &IdbusAddrholeAddr {
        &self.idbus_addrhole_addr
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn idbus_addrhole_info(&self) -> &IdbusAddrholeInfo {
        &self.idbus_addrhole_info
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn hp_por_rst_bypass_ctrl(&self) -> &HpPorRstBypassCtrl {
        &self.hp_por_rst_bypass_ctrl
    }
    #[doc = "0x1a4 - rng data register"]
    #[inline(always)]
    pub const fn rng_data(&self) -> &RngData {
        &self.rng_data
    }
    #[doc = "0x1b0 - need_des"]
    #[inline(always)]
    pub const fn lp_core_ahb_timeout(&self) -> &LpCoreAhbTimeout {
        &self.lp_core_ahb_timeout
    }
    #[doc = "0x1b4 - need_des"]
    #[inline(always)]
    pub const fn lp_core_ibus_timeout(&self) -> &LpCoreIbusTimeout {
        &self.lp_core_ibus_timeout
    }
    #[doc = "0x1b8 - need_des"]
    #[inline(always)]
    pub const fn lp_core_dbus_timeout(&self) -> &LpCoreDbusTimeout {
        &self.lp_core_dbus_timeout
    }
    #[doc = "0x1bc - need_des"]
    #[inline(always)]
    pub const fn lp_core_err_resp_dis(&self) -> &LpCoreErrRespDis {
        &self.lp_core_err_resp_dis
    }
    #[doc = "0x1c0 - rng cfg register"]
    #[inline(always)]
    pub const fn rng_cfg(&self) -> &RngCfg {
        &self.rng_cfg
    }
    #[doc = "0x1c4 - enable pad hold ctrl"]
    #[inline(always)]
    pub const fn pad_rtc_hold_ctrl0(&self) -> &PadRtcHoldCtrl0 {
        &self.pad_rtc_hold_ctrl0
    }
    #[doc = "0x1c8 - enable pad hold ctrl"]
    #[inline(always)]
    pub const fn pad_rtc_hold_ctrl1(&self) -> &PadRtcHoldCtrl1 {
        &self.pad_rtc_hold_ctrl1
    }
    #[doc = "0x1cc - enable pad hold ctrl"]
    #[inline(always)]
    pub const fn ded_pad_rtc_hold_ctrl(&self) -> &DedPadRtcHoldCtrl {
        &self.ded_pad_rtc_hold_ctrl
    }
    #[doc = "0x200 - pufmem / ldo flash power discharge control"]
    #[inline(always)]
    pub const fn discharge(&self) -> &Discharge {
        &self.discharge
    }
    #[doc = "0x204 - Usb otg2.0 PHY control register"]
    #[inline(always)]
    pub const fn hp_usb_otghs_phy_ctrl(&self) -> &HpUsbOtghsPhyCtrl {
        &self.hp_usb_otghs_phy_ctrl
    }
}
#[doc = "LP_SYS_VER_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sys_ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sys_ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sys_ver_date`] module"]
#[doc(alias = "LP_SYS_VER_DATE")]
pub type LpSysVerDate = crate::Reg<lp_sys_ver_date::LpSysVerDateSpec>;
#[doc = "need_des"]
pub mod lp_sys_ver_date;
#[doc = "CLK_SEL_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_sel_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sel_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sel_ctrl`] module"]
#[doc(alias = "CLK_SEL_CTRL")]
pub type ClkSelCtrl = crate::Reg<clk_sel_ctrl::ClkSelCtrlSpec>;
#[doc = "need_des"]
pub mod clk_sel_ctrl;
#[doc = "SYS_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`] module"]
#[doc(alias = "SYS_CTRL")]
pub type SysCtrl = crate::Reg<sys_ctrl::SysCtrlSpec>;
#[doc = "need_des"]
pub mod sys_ctrl;
#[doc = "LP_CLK_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_ctrl`] module"]
#[doc(alias = "LP_CLK_CTRL")]
pub type LpClkCtrl = crate::Reg<lp_clk_ctrl::LpClkCtrlSpec>;
#[doc = "need_des"]
pub mod lp_clk_ctrl;
#[doc = "LP_RST_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rst_ctrl`] module"]
#[doc(alias = "LP_RST_CTRL")]
pub type LpRstCtrl = crate::Reg<lp_rst_ctrl::LpRstCtrlSpec>;
#[doc = "need_des"]
pub mod lp_rst_ctrl;
#[doc = "LP_CORE_BOOT_ADDR (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_boot_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_boot_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_boot_addr`] module"]
#[doc(alias = "LP_CORE_BOOT_ADDR")]
pub type LpCoreBootAddr = crate::Reg<lp_core_boot_addr::LpCoreBootAddrSpec>;
#[doc = "need_des"]
pub mod lp_core_boot_addr;
#[doc = "EXT_WAKEUP1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1`] module"]
#[doc(alias = "EXT_WAKEUP1")]
pub type ExtWakeup1 = crate::Reg<ext_wakeup1::ExtWakeup1Spec>;
#[doc = "need_des"]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1_status`] module"]
#[doc(alias = "EXT_WAKEUP1_STATUS")]
pub type ExtWakeup1Status = crate::Reg<ext_wakeup1_status::ExtWakeup1StatusSpec>;
#[doc = "need_des"]
pub mod ext_wakeup1_status;
#[doc = "LP_TCM_PWR_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_pwr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_pwr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_pwr_ctrl`] module"]
#[doc(alias = "LP_TCM_PWR_CTRL")]
pub type LpTcmPwrCtrl = crate::Reg<lp_tcm_pwr_ctrl::LpTcmPwrCtrlSpec>;
#[doc = "need_des"]
pub mod lp_tcm_pwr_ctrl;
#[doc = "BOOT_ADDR_HP_LP (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_addr_hp_lp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_addr_hp_lp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_addr_hp_lp`] module"]
#[doc(alias = "BOOT_ADDR_HP_LP")]
pub type BootAddrHpLp = crate::Reg<boot_addr_hp_lp::BootAddrHpLpSpec>;
#[doc = "need_des"]
pub mod boot_addr_hp_lp;
#[doc = "LP_STORE0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store0`] module"]
#[doc(alias = "LP_STORE0")]
pub type LpStore0 = crate::Reg<lp_store0::LpStore0Spec>;
#[doc = "need_des"]
pub mod lp_store0;
#[doc = "LP_STORE1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store1`] module"]
#[doc(alias = "LP_STORE1")]
pub type LpStore1 = crate::Reg<lp_store1::LpStore1Spec>;
#[doc = "need_des"]
pub mod lp_store1;
#[doc = "LP_STORE2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store2`] module"]
#[doc(alias = "LP_STORE2")]
pub type LpStore2 = crate::Reg<lp_store2::LpStore2Spec>;
#[doc = "need_des"]
pub mod lp_store2;
#[doc = "LP_STORE3 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store3`] module"]
#[doc(alias = "LP_STORE3")]
pub type LpStore3 = crate::Reg<lp_store3::LpStore3Spec>;
#[doc = "need_des"]
pub mod lp_store3;
#[doc = "LP_STORE4 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store4`] module"]
#[doc(alias = "LP_STORE4")]
pub type LpStore4 = crate::Reg<lp_store4::LpStore4Spec>;
#[doc = "need_des"]
pub mod lp_store4;
#[doc = "LP_STORE5 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store5`] module"]
#[doc(alias = "LP_STORE5")]
pub type LpStore5 = crate::Reg<lp_store5::LpStore5Spec>;
#[doc = "need_des"]
pub mod lp_store5;
#[doc = "LP_STORE6 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store6`] module"]
#[doc(alias = "LP_STORE6")]
pub type LpStore6 = crate::Reg<lp_store6::LpStore6Spec>;
#[doc = "need_des"]
pub mod lp_store6;
#[doc = "LP_STORE7 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store7`] module"]
#[doc(alias = "LP_STORE7")]
pub type LpStore7 = crate::Reg<lp_store7::LpStore7Spec>;
#[doc = "need_des"]
pub mod lp_store7;
#[doc = "LP_STORE8 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store8`] module"]
#[doc(alias = "LP_STORE8")]
pub type LpStore8 = crate::Reg<lp_store8::LpStore8Spec>;
#[doc = "need_des"]
pub mod lp_store8;
#[doc = "LP_STORE9 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store9`] module"]
#[doc(alias = "LP_STORE9")]
pub type LpStore9 = crate::Reg<lp_store9::LpStore9Spec>;
#[doc = "need_des"]
pub mod lp_store9;
#[doc = "LP_STORE10 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store10`] module"]
#[doc(alias = "LP_STORE10")]
pub type LpStore10 = crate::Reg<lp_store10::LpStore10Spec>;
#[doc = "need_des"]
pub mod lp_store10;
#[doc = "LP_STORE11 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store11`] module"]
#[doc(alias = "LP_STORE11")]
pub type LpStore11 = crate::Reg<lp_store11::LpStore11Spec>;
#[doc = "need_des"]
pub mod lp_store11;
#[doc = "LP_STORE12 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store12`] module"]
#[doc(alias = "LP_STORE12")]
pub type LpStore12 = crate::Reg<lp_store12::LpStore12Spec>;
#[doc = "need_des"]
pub mod lp_store12;
#[doc = "LP_STORE13 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store13`] module"]
#[doc(alias = "LP_STORE13")]
pub type LpStore13 = crate::Reg<lp_store13::LpStore13Spec>;
#[doc = "need_des"]
pub mod lp_store13;
#[doc = "LP_STORE14 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store14`] module"]
#[doc(alias = "LP_STORE14")]
pub type LpStore14 = crate::Reg<lp_store14::LpStore14Spec>;
#[doc = "need_des"]
pub mod lp_store14;
#[doc = "LP_STORE15 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store15`] module"]
#[doc(alias = "LP_STORE15")]
pub type LpStore15 = crate::Reg<lp_store15::LpStore15Spec>;
#[doc = "need_des"]
pub mod lp_store15;
#[doc = "LP_PROBEA_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_probea_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_probea_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probea_ctrl`] module"]
#[doc(alias = "LP_PROBEA_CTRL")]
pub type LpProbeaCtrl = crate::Reg<lp_probea_ctrl::LpProbeaCtrlSpec>;
#[doc = "need_des"]
pub mod lp_probea_ctrl;
#[doc = "LP_PROBEB_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_probeb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_probeb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probeb_ctrl`] module"]
#[doc(alias = "LP_PROBEB_CTRL")]
pub type LpProbebCtrl = crate::Reg<lp_probeb_ctrl::LpProbebCtrlSpec>;
#[doc = "need_des"]
pub mod lp_probeb_ctrl;
#[doc = "LP_PROBE_OUT (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_probe_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probe_out`] module"]
#[doc(alias = "LP_PROBE_OUT")]
pub type LpProbeOut = crate::Reg<lp_probe_out::LpProbeOutSpec>;
#[doc = "need_des"]
pub mod lp_probe_out;
#[doc = "F2S_APB_BRG_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`f2s_apb_brg_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2s_apb_brg_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2s_apb_brg_cntl`] module"]
#[doc(alias = "F2S_APB_BRG_CNTL")]
pub type F2sApbBrgCntl = crate::Reg<f2s_apb_brg_cntl::F2sApbBrgCntlSpec>;
#[doc = "need_des"]
pub mod f2s_apb_brg_cntl;
#[doc = "USB_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctrl`] module"]
#[doc(alias = "USB_CTRL")]
pub type UsbCtrl = crate::Reg<usb_ctrl::UsbCtrlSpec>;
#[doc = "need_des"]
pub mod usb_ctrl;
#[doc = "ANA_XPD_PAD_GROUP (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_xpd_pad_group::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_xpd_pad_group::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_xpd_pad_group`] module"]
#[doc(alias = "ANA_XPD_PAD_GROUP")]
pub type AnaXpdPadGroup = crate::Reg<ana_xpd_pad_group::AnaXpdPadGroupSpec>;
#[doc = "need_des"]
pub mod ana_xpd_pad_group;
#[doc = "LP_TCM_RAM_RDN_ECO_CS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_cs`] module"]
#[doc(alias = "LP_TCM_RAM_RDN_ECO_CS")]
pub type LpTcmRamRdnEcoCs = crate::Reg<lp_tcm_ram_rdn_eco_cs::LpTcmRamRdnEcoCsSpec>;
#[doc = "need_des"]
pub mod lp_tcm_ram_rdn_eco_cs;
#[doc = "LP_TCM_RAM_RDN_ECO_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_low`] module"]
#[doc(alias = "LP_TCM_RAM_RDN_ECO_LOW")]
pub type LpTcmRamRdnEcoLow = crate::Reg<lp_tcm_ram_rdn_eco_low::LpTcmRamRdnEcoLowSpec>;
#[doc = "need_des"]
pub mod lp_tcm_ram_rdn_eco_low;
#[doc = "LP_TCM_RAM_RDN_ECO_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_high`] module"]
#[doc(alias = "LP_TCM_RAM_RDN_ECO_HIGH")]
pub type LpTcmRamRdnEcoHigh = crate::Reg<lp_tcm_ram_rdn_eco_high::LpTcmRamRdnEcoHighSpec>;
#[doc = "need_des"]
pub mod lp_tcm_ram_rdn_eco_high;
#[doc = "LP_TCM_ROM_RDN_ECO_CS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_cs`] module"]
#[doc(alias = "LP_TCM_ROM_RDN_ECO_CS")]
pub type LpTcmRomRdnEcoCs = crate::Reg<lp_tcm_rom_rdn_eco_cs::LpTcmRomRdnEcoCsSpec>;
#[doc = "need_des"]
pub mod lp_tcm_rom_rdn_eco_cs;
#[doc = "LP_TCM_ROM_RDN_ECO_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_low`] module"]
#[doc(alias = "LP_TCM_ROM_RDN_ECO_LOW")]
pub type LpTcmRomRdnEcoLow = crate::Reg<lp_tcm_rom_rdn_eco_low::LpTcmRomRdnEcoLowSpec>;
#[doc = "need_des"]
pub mod lp_tcm_rom_rdn_eco_low;
#[doc = "LP_TCM_ROM_RDN_ECO_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_high`] module"]
#[doc(alias = "LP_TCM_ROM_RDN_ECO_HIGH")]
pub type LpTcmRomRdnEcoHigh = crate::Reg<lp_tcm_rom_rdn_eco_high::LpTcmRomRdnEcoHighSpec>;
#[doc = "need_des"]
pub mod lp_tcm_rom_rdn_eco_high;
#[doc = "HP_ROOT_CLK_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_root_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_root_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_root_clk_ctrl`] module"]
#[doc(alias = "HP_ROOT_CLK_CTRL")]
pub type HpRootClkCtrl = crate::Reg<hp_root_clk_ctrl::HpRootClkCtrlSpec>;
#[doc = "need_des"]
pub mod hp_root_clk_ctrl;
#[doc = "LP_PMU_RDN_ECO_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pmu_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pmu_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pmu_rdn_eco_low`] module"]
#[doc(alias = "LP_PMU_RDN_ECO_LOW")]
pub type LpPmuRdnEcoLow = crate::Reg<lp_pmu_rdn_eco_low::LpPmuRdnEcoLowSpec>;
#[doc = "need_des"]
pub mod lp_pmu_rdn_eco_low;
#[doc = "LP_PMU_RDN_ECO_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pmu_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pmu_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pmu_rdn_eco_high`] module"]
#[doc(alias = "LP_PMU_RDN_ECO_HIGH")]
pub type LpPmuRdnEcoHigh = crate::Reg<lp_pmu_rdn_eco_high::LpPmuRdnEcoHighSpec>;
#[doc = "need_des"]
pub mod lp_pmu_rdn_eco_high;
#[doc = "PAD_COMP0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp0`] module"]
#[doc(alias = "PAD_COMP0")]
pub type PadComp0 = crate::Reg<pad_comp0::PadComp0Spec>;
#[doc = "need_des"]
pub mod pad_comp0;
#[doc = "PAD_COMP1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp1`] module"]
#[doc(alias = "PAD_COMP1")]
pub type PadComp1 = crate::Reg<pad_comp1::PadComp1Spec>;
#[doc = "need_des"]
pub mod pad_comp1;
#[doc = "BACKUP_DMA_CFG0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg0`] module"]
#[doc(alias = "BACKUP_DMA_CFG0")]
pub type BackupDmaCfg0 = crate::Reg<backup_dma_cfg0::BackupDmaCfg0Spec>;
#[doc = "need_des"]
pub mod backup_dma_cfg0;
#[doc = "BACKUP_DMA_CFG1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg1`] module"]
#[doc(alias = "BACKUP_DMA_CFG1")]
pub type BackupDmaCfg1 = crate::Reg<backup_dma_cfg1::BackupDmaCfg1Spec>;
#[doc = "need_des"]
pub mod backup_dma_cfg1;
#[doc = "BACKUP_DMA_CFG2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg2`] module"]
#[doc(alias = "BACKUP_DMA_CFG2")]
pub type BackupDmaCfg2 = crate::Reg<backup_dma_cfg2::BackupDmaCfg2Spec>;
#[doc = "need_des"]
pub mod backup_dma_cfg2;
#[doc = "BOOT_ADDR_HP_CORE1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_addr_hp_core1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_addr_hp_core1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_addr_hp_core1`] module"]
#[doc(alias = "BOOT_ADDR_HP_CORE1")]
pub type BootAddrHpCore1 = crate::Reg<boot_addr_hp_core1::BootAddrHpCore1Spec>;
#[doc = "need_des"]
pub mod boot_addr_hp_core1;
#[doc = "LP_ADDRHOLE_ADDR (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_addrhole_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_addrhole_addr`] module"]
#[doc(alias = "LP_ADDRHOLE_ADDR")]
pub type LpAddrholeAddr = crate::Reg<lp_addrhole_addr::LpAddrholeAddrSpec>;
#[doc = "need_des"]
pub mod lp_addrhole_addr;
#[doc = "LP_ADDRHOLE_INFO (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_addrhole_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_addrhole_info`] module"]
#[doc(alias = "LP_ADDRHOLE_INFO")]
pub type LpAddrholeInfo = crate::Reg<lp_addrhole_info::LpAddrholeInfoSpec>;
#[doc = "need_des"]
pub mod lp_addrhole_info;
#[doc = "INT_RAW (r) register accessor: raw interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "raw interrupt register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "masked interrupt register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "masked interrupt register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "interrupt clear register"]
pub mod int_clr;
#[doc = "HP_MEM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_aux_ctrl`] module"]
#[doc(alias = "HP_MEM_AUX_CTRL")]
pub type HpMemAuxCtrl = crate::Reg<hp_mem_aux_ctrl::HpMemAuxCtrlSpec>;
#[doc = "need_des"]
pub mod hp_mem_aux_ctrl;
#[doc = "LP_MEM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mem_aux_ctrl`] module"]
#[doc(alias = "LP_MEM_AUX_CTRL")]
pub type LpMemAuxCtrl = crate::Reg<lp_mem_aux_ctrl::LpMemAuxCtrlSpec>;
#[doc = "need_des"]
pub mod lp_mem_aux_ctrl;
#[doc = "HP_ROM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rom_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rom_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rom_aux_ctrl`] module"]
#[doc(alias = "HP_ROM_AUX_CTRL")]
pub type HpRomAuxCtrl = crate::Reg<hp_rom_aux_ctrl::HpRomAuxCtrlSpec>;
#[doc = "need_des"]
pub mod hp_rom_aux_ctrl;
#[doc = "LP_ROM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rom_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rom_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rom_aux_ctrl`] module"]
#[doc(alias = "LP_ROM_AUX_CTRL")]
pub type LpRomAuxCtrl = crate::Reg<lp_rom_aux_ctrl::LpRomAuxCtrlSpec>;
#[doc = "need_des"]
pub mod lp_rom_aux_ctrl;
#[doc = "LP_CPU_DBG_PC (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_dbg_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_dbg_pc`] module"]
#[doc(alias = "LP_CPU_DBG_PC")]
pub type LpCpuDbgPc = crate::Reg<lp_cpu_dbg_pc::LpCpuDbgPcSpec>;
#[doc = "need_des"]
pub mod lp_cpu_dbg_pc;
#[doc = "LP_CPU_EXC_PC (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_exc_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_exc_pc`] module"]
#[doc(alias = "LP_CPU_EXC_PC")]
pub type LpCpuExcPc = crate::Reg<lp_cpu_exc_pc::LpCpuExcPcSpec>;
#[doc = "need_des"]
pub mod lp_cpu_exc_pc;
#[doc = "IDBUS_ADDRHOLE_ADDR (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`idbus_addrhole_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idbus_addrhole_addr`] module"]
#[doc(alias = "IDBUS_ADDRHOLE_ADDR")]
pub type IdbusAddrholeAddr = crate::Reg<idbus_addrhole_addr::IdbusAddrholeAddrSpec>;
#[doc = "need_des"]
pub mod idbus_addrhole_addr;
#[doc = "IDBUS_ADDRHOLE_INFO (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`idbus_addrhole_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idbus_addrhole_info`] module"]
#[doc(alias = "IDBUS_ADDRHOLE_INFO")]
pub type IdbusAddrholeInfo = crate::Reg<idbus_addrhole_info::IdbusAddrholeInfoSpec>;
#[doc = "need_des"]
pub mod idbus_addrhole_info;
#[doc = "HP_POR_RST_BYPASS_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_por_rst_bypass_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_por_rst_bypass_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_por_rst_bypass_ctrl`] module"]
#[doc(alias = "HP_POR_RST_BYPASS_CTRL")]
pub type HpPorRstBypassCtrl = crate::Reg<hp_por_rst_bypass_ctrl::HpPorRstBypassCtrlSpec>;
#[doc = "need_des"]
pub mod hp_por_rst_bypass_ctrl;
#[doc = "RNG_DATA (r) register accessor: rng data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_data`] module"]
#[doc(alias = "RNG_DATA")]
pub type RngData = crate::Reg<rng_data::RngDataSpec>;
#[doc = "rng data register"]
pub mod rng_data;
#[doc = "LP_CORE_AHB_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_ahb_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_ahb_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_ahb_timeout`] module"]
#[doc(alias = "LP_CORE_AHB_TIMEOUT")]
pub type LpCoreAhbTimeout = crate::Reg<lp_core_ahb_timeout::LpCoreAhbTimeoutSpec>;
#[doc = "need_des"]
pub mod lp_core_ahb_timeout;
#[doc = "LP_CORE_IBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_ibus_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_ibus_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_ibus_timeout`] module"]
#[doc(alias = "LP_CORE_IBUS_TIMEOUT")]
pub type LpCoreIbusTimeout = crate::Reg<lp_core_ibus_timeout::LpCoreIbusTimeoutSpec>;
#[doc = "need_des"]
pub mod lp_core_ibus_timeout;
#[doc = "LP_CORE_DBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_dbus_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_dbus_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_dbus_timeout`] module"]
#[doc(alias = "LP_CORE_DBUS_TIMEOUT")]
pub type LpCoreDbusTimeout = crate::Reg<lp_core_dbus_timeout::LpCoreDbusTimeoutSpec>;
#[doc = "need_des"]
pub mod lp_core_dbus_timeout;
#[doc = "LP_CORE_ERR_RESP_DIS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_err_resp_dis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_err_resp_dis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_err_resp_dis`] module"]
#[doc(alias = "LP_CORE_ERR_RESP_DIS")]
pub type LpCoreErrRespDis = crate::Reg<lp_core_err_resp_dis::LpCoreErrRespDisSpec>;
#[doc = "need_des"]
pub mod lp_core_err_resp_dis;
#[doc = "RNG_CFG (rw) register accessor: rng cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cfg`] module"]
#[doc(alias = "RNG_CFG")]
pub type RngCfg = crate::Reg<rng_cfg::RngCfgSpec>;
#[doc = "rng cfg register"]
pub mod rng_cfg;
#[doc = "PAD_RTC_HOLD_CTRL0 (rw) register accessor: enable pad hold ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_rtc_hold_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_rtc_hold_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rtc_hold_ctrl0`] module"]
#[doc(alias = "PAD_RTC_HOLD_CTRL0")]
pub type PadRtcHoldCtrl0 = crate::Reg<pad_rtc_hold_ctrl0::PadRtcHoldCtrl0Spec>;
#[doc = "enable pad hold ctrl"]
pub mod pad_rtc_hold_ctrl0;
#[doc = "PAD_RTC_HOLD_CTRL1 (rw) register accessor: enable pad hold ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_rtc_hold_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_rtc_hold_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rtc_hold_ctrl1`] module"]
#[doc(alias = "PAD_RTC_HOLD_CTRL1")]
pub type PadRtcHoldCtrl1 = crate::Reg<pad_rtc_hold_ctrl1::PadRtcHoldCtrl1Spec>;
#[doc = "enable pad hold ctrl"]
pub mod pad_rtc_hold_ctrl1;
#[doc = "DED_PAD_RTC_HOLD_CTRL (rw) register accessor: enable pad hold ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_pad_rtc_hold_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_pad_rtc_hold_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_pad_rtc_hold_ctrl`] module"]
#[doc(alias = "DED_PAD_RTC_HOLD_CTRL")]
pub type DedPadRtcHoldCtrl = crate::Reg<ded_pad_rtc_hold_ctrl::DedPadRtcHoldCtrlSpec>;
#[doc = "enable pad hold ctrl"]
pub mod ded_pad_rtc_hold_ctrl;
#[doc = "DISCHARGE (rw) register accessor: pufmem / ldo flash power discharge control\n\nYou can [`read`](crate::Reg::read) this register and get [`discharge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`discharge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@discharge`] module"]
#[doc(alias = "DISCHARGE")]
pub type Discharge = crate::Reg<discharge::DischargeSpec>;
#[doc = "pufmem / ldo flash power discharge control"]
pub mod discharge;
#[doc = "HP_USB_OTGHS_PHY_CTRL (rw) register accessor: Usb otg2.0 PHY control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_usb_otghs_phy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_usb_otghs_phy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_usb_otghs_phy_ctrl`] module"]
#[doc(alias = "HP_USB_OTGHS_PHY_CTRL")]
pub type HpUsbOtghsPhyCtrl = crate::Reg<hp_usb_otghs_phy_ctrl::HpUsbOtghsPhyCtrlSpec>;
#[doc = "Usb otg2.0 PHY control register"]
pub mod hp_usb_otghs_phy_ctrl;
