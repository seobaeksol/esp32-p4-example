#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hp_active_dig_power: HpActiveDigPower,
    hp_active_icg_hp_func: HpActiveIcgHpFunc,
    hp_active_icg_hp_apb: HpActiveIcgHpApb,
    hp_active_icg_modem: HpActiveIcgModem,
    hp_active_hp_sys_cntl: HpActiveHpSysCntl,
    hp_active_hp_ck_power: HpActiveHpCkPower,
    hp_active_bias: HpActiveBias,
    hp_active_backup: HpActiveBackup,
    hp_active_backup_clk: HpActiveBackupClk,
    hp_active_sysclk: HpActiveSysclk,
    hp_active_hp_regulator0: HpActiveHpRegulator0,
    hp_active_hp_regulator1: HpActiveHpRegulator1,
    hp_active_xtal: HpActiveXtal,
    hp_modem_dig_power: HpModemDigPower,
    hp_modem_icg_hp_func: HpModemIcgHpFunc,
    hp_modem_icg_hp_apb: HpModemIcgHpApb,
    hp_modem_icg_modem: HpModemIcgModem,
    hp_modem_hp_sys_cntl: HpModemHpSysCntl,
    hp_modem_hp_ck_power: HpModemHpCkPower,
    hp_modem_bias: HpModemBias,
    hp_modem_backup: HpModemBackup,
    hp_modem_backup_clk: HpModemBackupClk,
    hp_modem_sysclk: HpModemSysclk,
    hp_modem_hp_regulator0: HpModemHpRegulator0,
    hp_modem_hp_regulator1: HpModemHpRegulator1,
    hp_modem_xtal: HpModemXtal,
    hp_sleep_dig_power: HpSleepDigPower,
    hp_sleep_icg_hp_func: HpSleepIcgHpFunc,
    hp_sleep_icg_hp_apb: HpSleepIcgHpApb,
    hp_sleep_icg_modem: HpSleepIcgModem,
    hp_sleep_hp_sys_cntl: HpSleepHpSysCntl,
    hp_sleep_hp_ck_power: HpSleepHpCkPower,
    hp_sleep_bias: HpSleepBias,
    hp_sleep_backup: HpSleepBackup,
    hp_sleep_backup_clk: HpSleepBackupClk,
    hp_sleep_sysclk: HpSleepSysclk,
    hp_sleep_hp_regulator0: HpSleepHpRegulator0,
    hp_sleep_hp_regulator1: HpSleepHpRegulator1,
    hp_sleep_xtal: HpSleepXtal,
    hp_sleep_lp_regulator0: HpSleepLpRegulator0,
    hp_sleep_lp_regulator1: HpSleepLpRegulator1,
    hp_sleep_lp_dcdc_reserve: HpSleepLpDcdcReserve,
    hp_sleep_lp_dig_power: HpSleepLpDigPower,
    hp_sleep_lp_ck_power: HpSleepLpCkPower,
    lp_sleep_lp_bias_reserve: LpSleepLpBiasReserve,
    lp_sleep_lp_regulator0: LpSleepLpRegulator0,
    lp_sleep_lp_regulator1: LpSleepLpRegulator1,
    lp_sleep_xtal: LpSleepXtal,
    lp_sleep_lp_dig_power: LpSleepLpDigPower,
    lp_sleep_lp_ck_power: LpSleepLpCkPower,
    lp_sleep_bias: LpSleepBias,
    imm_hp_ck_power: ImmHpCkPower,
    imm_sleep_sysclk: ImmSleepSysclk,
    imm_hp_func_icg: ImmHpFuncIcg,
    imm_hp_apb_icg: ImmHpApbIcg,
    imm_modem_icg: ImmModemIcg,
    imm_lp_icg: ImmLpIcg,
    imm_pad_hold_all: ImmPadHoldAll,
    imm_i2c_iso: ImmI2cIso,
    power_wait_timer0: PowerWaitTimer0,
    power_wait_timer1: PowerWaitTimer1,
    power_pd_top_cntl: PowerPdTopCntl,
    power_pd_cnnt_cntl: PowerPdCnntCntl,
    power_pd_hpmem_cntl: PowerPdHpmemCntl,
    power_pd_top_mask: PowerPdTopMask,
    power_pd_cnnt_mask: PowerPdCnntMask,
    power_pd_hpmem_mask: PowerPdHpmemMask,
    power_dcdc_switch: PowerDcdcSwitch,
    power_pd_lpperi_cntl: PowerPdLpperiCntl,
    power_pd_lpperi_mask: PowerPdLpperiMask,
    power_hp_pad: PowerHpPad,
    power_ck_wait_cntl: PowerCkWaitCntl,
    slp_wakeup_cntl0: SlpWakeupCntl0,
    slp_wakeup_cntl1: SlpWakeupCntl1,
    slp_wakeup_cntl2: SlpWakeupCntl2,
    slp_wakeup_cntl3: SlpWakeupCntl3,
    slp_wakeup_cntl4: SlpWakeupCntl4,
    slp_wakeup_cntl5: SlpWakeupCntl5,
    slp_wakeup_cntl6: SlpWakeupCntl6,
    slp_wakeup_cntl7: SlpWakeupCntl7,
    slp_wakeup_cntl8: SlpWakeupCntl8,
    slp_wakeup_status0: SlpWakeupStatus0,
    slp_wakeup_status1: SlpWakeupStatus1,
    slp_wakeup_status2: SlpWakeupStatus2,
    hp_ck_poweron: HpCkPoweron,
    hp_ck_cntl: HpCkCntl,
    por_status: PorStatus,
    rf_pwc: RfPwc,
    backup_cfg: BackupCfg,
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    lp_int_raw: LpIntRaw,
    lp_int_st: LpIntSt,
    lp_int_ena: LpIntEna,
    lp_int_clr: LpIntClr,
    lp_cpu_pwr0: LpCpuPwr0,
    lp_cpu_pwr1: LpCpuPwr1,
    lp_cpu_pwr2: LpCpuPwr2,
    lp_cpu_pwr3: LpCpuPwr3,
    lp_cpu_pwr4: LpCpuPwr4,
    lp_cpu_pwr5: LpCpuPwr5,
    hp_lp_cpu_comm: HpLpCpuComm,
    hp_regulator_cfg: HpRegulatorCfg,
    main_state: MainState,
    pwr_state: PwrState,
    clk_state0: ClkState0,
    clk_state1: ClkState1,
    clk_state2: ClkState2,
    ext_ldo_p0_0p1a: ExtLdoP0_0p1a,
    ext_ldo_p0_0p1a_ana: ExtLdoP0_0p1aAna,
    ext_ldo_p0_0p2a: ExtLdoP0_0p2a,
    ext_ldo_p0_0p2a_ana: ExtLdoP0_0p2aAna,
    ext_ldo_p0_0p3a: ExtLdoP0_0p3a,
    ext_ldo_p0_0p3a_ana: ExtLdoP0_0p3aAna,
    ext_ldo_p1_0p1a: ExtLdoP1_0p1a,
    ext_ldo_p1_0p1a_ana: ExtLdoP1_0p1aAna,
    ext_ldo_p1_0p2a: ExtLdoP1_0p2a,
    ext_ldo_p1_0p2a_ana: ExtLdoP1_0p2aAna,
    ext_ldo_p1_0p3a: ExtLdoP1_0p3a,
    ext_ldo_p1_0p3a_ana: ExtLdoP1_0p3aAna,
    ext_wakeup_lv: ExtWakeupLv,
    ext_wakeup_sel: ExtWakeupSel,
    ext_wakeup_st: ExtWakeupSt,
    ext_wakeup_cntl: ExtWakeupCntl,
    sdio_wakeup_cntl: SdioWakeupCntl,
    xtal_slp: XtalSlp,
    cpu_sw_stall: CpuSwStall,
    dcm_ctrl: DcmCtrl,
    dcm_wait_delay: DcmWaitDelay,
    vddbat_cfg: VddbatCfg,
    touch_pwr_cntl: TouchPwrCntl,
    rdn_eco: RdnEco,
    power_pd_hp_cpu_cntl: PowerPdHpCpuCntl,
    power_pd_hp_cpu_mask: PowerPdHpCpuMask,
    _reserved136: [u8; 0x01dc],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn hp_active_dig_power(&self) -> &HpActiveDigPower {
        &self.hp_active_dig_power
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn hp_active_icg_hp_func(&self) -> &HpActiveIcgHpFunc {
        &self.hp_active_icg_hp_func
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn hp_active_icg_hp_apb(&self) -> &HpActiveIcgHpApb {
        &self.hp_active_icg_hp_apb
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn hp_active_icg_modem(&self) -> &HpActiveIcgModem {
        &self.hp_active_icg_modem
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn hp_active_hp_sys_cntl(&self) -> &HpActiveHpSysCntl {
        &self.hp_active_hp_sys_cntl
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn hp_active_hp_ck_power(&self) -> &HpActiveHpCkPower {
        &self.hp_active_hp_ck_power
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn hp_active_bias(&self) -> &HpActiveBias {
        &self.hp_active_bias
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn hp_active_backup(&self) -> &HpActiveBackup {
        &self.hp_active_backup
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn hp_active_backup_clk(&self) -> &HpActiveBackupClk {
        &self.hp_active_backup_clk
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn hp_active_sysclk(&self) -> &HpActiveSysclk {
        &self.hp_active_sysclk
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn hp_active_hp_regulator0(&self) -> &HpActiveHpRegulator0 {
        &self.hp_active_hp_regulator0
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn hp_active_hp_regulator1(&self) -> &HpActiveHpRegulator1 {
        &self.hp_active_hp_regulator1
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn hp_active_xtal(&self) -> &HpActiveXtal {
        &self.hp_active_xtal
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_dig_power(&self) -> &HpModemDigPower {
        &self.hp_modem_dig_power
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_icg_hp_func(&self) -> &HpModemIcgHpFunc {
        &self.hp_modem_icg_hp_func
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn hp_modem_icg_hp_apb(&self) -> &HpModemIcgHpApb {
        &self.hp_modem_icg_hp_apb
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_icg_modem(&self) -> &HpModemIcgModem {
        &self.hp_modem_icg_modem
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_hp_sys_cntl(&self) -> &HpModemHpSysCntl {
        &self.hp_modem_hp_sys_cntl
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_hp_ck_power(&self) -> &HpModemHpCkPower {
        &self.hp_modem_hp_ck_power
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn hp_modem_bias(&self) -> &HpModemBias {
        &self.hp_modem_bias
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_backup(&self) -> &HpModemBackup {
        &self.hp_modem_backup
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_backup_clk(&self) -> &HpModemBackupClk {
        &self.hp_modem_backup_clk
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_sysclk(&self) -> &HpModemSysclk {
        &self.hp_modem_sysclk
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn hp_modem_hp_regulator0(&self) -> &HpModemHpRegulator0 {
        &self.hp_modem_hp_regulator0
    }
    #[doc = "0x60 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_hp_regulator1(&self) -> &HpModemHpRegulator1 {
        &self.hp_modem_hp_regulator1
    }
    #[doc = "0x64 - need_des"]
    #[inline(always)]
    pub const fn hp_modem_xtal(&self) -> &HpModemXtal {
        &self.hp_modem_xtal
    }
    #[doc = "0x68 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_dig_power(&self) -> &HpSleepDigPower {
        &self.hp_sleep_dig_power
    }
    #[doc = "0x6c - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_icg_hp_func(&self) -> &HpSleepIcgHpFunc {
        &self.hp_sleep_icg_hp_func
    }
    #[doc = "0x70 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_icg_hp_apb(&self) -> &HpSleepIcgHpApb {
        &self.hp_sleep_icg_hp_apb
    }
    #[doc = "0x74 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_icg_modem(&self) -> &HpSleepIcgModem {
        &self.hp_sleep_icg_modem
    }
    #[doc = "0x78 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_hp_sys_cntl(&self) -> &HpSleepHpSysCntl {
        &self.hp_sleep_hp_sys_cntl
    }
    #[doc = "0x7c - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_hp_ck_power(&self) -> &HpSleepHpCkPower {
        &self.hp_sleep_hp_ck_power
    }
    #[doc = "0x80 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_bias(&self) -> &HpSleepBias {
        &self.hp_sleep_bias
    }
    #[doc = "0x84 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_backup(&self) -> &HpSleepBackup {
        &self.hp_sleep_backup
    }
    #[doc = "0x88 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_backup_clk(&self) -> &HpSleepBackupClk {
        &self.hp_sleep_backup_clk
    }
    #[doc = "0x8c - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_sysclk(&self) -> &HpSleepSysclk {
        &self.hp_sleep_sysclk
    }
    #[doc = "0x90 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_hp_regulator0(&self) -> &HpSleepHpRegulator0 {
        &self.hp_sleep_hp_regulator0
    }
    #[doc = "0x94 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_hp_regulator1(&self) -> &HpSleepHpRegulator1 {
        &self.hp_sleep_hp_regulator1
    }
    #[doc = "0x98 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_xtal(&self) -> &HpSleepXtal {
        &self.hp_sleep_xtal
    }
    #[doc = "0x9c - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_lp_regulator0(&self) -> &HpSleepLpRegulator0 {
        &self.hp_sleep_lp_regulator0
    }
    #[doc = "0xa0 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_lp_regulator1(&self) -> &HpSleepLpRegulator1 {
        &self.hp_sleep_lp_regulator1
    }
    #[doc = "0xa4 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_lp_dcdc_reserve(&self) -> &HpSleepLpDcdcReserve {
        &self.hp_sleep_lp_dcdc_reserve
    }
    #[doc = "0xa8 - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_lp_dig_power(&self) -> &HpSleepLpDigPower {
        &self.hp_sleep_lp_dig_power
    }
    #[doc = "0xac - need_des"]
    #[inline(always)]
    pub const fn hp_sleep_lp_ck_power(&self) -> &HpSleepLpCkPower {
        &self.hp_sleep_lp_ck_power
    }
    #[doc = "0xb0 - need_des"]
    #[inline(always)]
    pub const fn lp_sleep_lp_bias_reserve(&self) -> &LpSleepLpBiasReserve {
        &self.lp_sleep_lp_bias_reserve
    }
    #[doc = "0xb4 - need_des"]
    #[inline(always)]
    pub const fn lp_sleep_lp_regulator0(&self) -> &LpSleepLpRegulator0 {
        &self.lp_sleep_lp_regulator0
    }
    #[doc = "0xb8 - need_des"]
    #[inline(always)]
    pub const fn lp_sleep_lp_regulator1(&self) -> &LpSleepLpRegulator1 {
        &self.lp_sleep_lp_regulator1
    }
    #[doc = "0xbc - need_des"]
    #[inline(always)]
    pub const fn lp_sleep_xtal(&self) -> &LpSleepXtal {
        &self.lp_sleep_xtal
    }
    #[doc = "0xc0 - need_des"]
    #[inline(always)]
    pub const fn lp_sleep_lp_dig_power(&self) -> &LpSleepLpDigPower {
        &self.lp_sleep_lp_dig_power
    }
    #[doc = "0xc4 - need_des"]
    #[inline(always)]
    pub const fn lp_sleep_lp_ck_power(&self) -> &LpSleepLpCkPower {
        &self.lp_sleep_lp_ck_power
    }
    #[doc = "0xc8 - need_des"]
    #[inline(always)]
    pub const fn lp_sleep_bias(&self) -> &LpSleepBias {
        &self.lp_sleep_bias
    }
    #[doc = "0xcc - need_des"]
    #[inline(always)]
    pub const fn imm_hp_ck_power(&self) -> &ImmHpCkPower {
        &self.imm_hp_ck_power
    }
    #[doc = "0xd0 - need_des"]
    #[inline(always)]
    pub const fn imm_sleep_sysclk(&self) -> &ImmSleepSysclk {
        &self.imm_sleep_sysclk
    }
    #[doc = "0xd4 - need_des"]
    #[inline(always)]
    pub const fn imm_hp_func_icg(&self) -> &ImmHpFuncIcg {
        &self.imm_hp_func_icg
    }
    #[doc = "0xd8 - need_des"]
    #[inline(always)]
    pub const fn imm_hp_apb_icg(&self) -> &ImmHpApbIcg {
        &self.imm_hp_apb_icg
    }
    #[doc = "0xdc - need_des"]
    #[inline(always)]
    pub const fn imm_modem_icg(&self) -> &ImmModemIcg {
        &self.imm_modem_icg
    }
    #[doc = "0xe0 - need_des"]
    #[inline(always)]
    pub const fn imm_lp_icg(&self) -> &ImmLpIcg {
        &self.imm_lp_icg
    }
    #[doc = "0xe4 - need_des"]
    #[inline(always)]
    pub const fn imm_pad_hold_all(&self) -> &ImmPadHoldAll {
        &self.imm_pad_hold_all
    }
    #[doc = "0xe8 - need_des"]
    #[inline(always)]
    pub const fn imm_i2c_iso(&self) -> &ImmI2cIso {
        &self.imm_i2c_iso
    }
    #[doc = "0xec - need_des"]
    #[inline(always)]
    pub const fn power_wait_timer0(&self) -> &PowerWaitTimer0 {
        &self.power_wait_timer0
    }
    #[doc = "0xf0 - need_des"]
    #[inline(always)]
    pub const fn power_wait_timer1(&self) -> &PowerWaitTimer1 {
        &self.power_wait_timer1
    }
    #[doc = "0xf4 - need_des"]
    #[inline(always)]
    pub const fn power_pd_top_cntl(&self) -> &PowerPdTopCntl {
        &self.power_pd_top_cntl
    }
    #[doc = "0xf8 - need_des"]
    #[inline(always)]
    pub const fn power_pd_cnnt_cntl(&self) -> &PowerPdCnntCntl {
        &self.power_pd_cnnt_cntl
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn power_pd_hpmem_cntl(&self) -> &PowerPdHpmemCntl {
        &self.power_pd_hpmem_cntl
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn power_pd_top_mask(&self) -> &PowerPdTopMask {
        &self.power_pd_top_mask
    }
    #[doc = "0x104 - need_des"]
    #[inline(always)]
    pub const fn power_pd_cnnt_mask(&self) -> &PowerPdCnntMask {
        &self.power_pd_cnnt_mask
    }
    #[doc = "0x108 - need_des"]
    #[inline(always)]
    pub const fn power_pd_hpmem_mask(&self) -> &PowerPdHpmemMask {
        &self.power_pd_hpmem_mask
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn power_dcdc_switch(&self) -> &PowerDcdcSwitch {
        &self.power_dcdc_switch
    }
    #[doc = "0x110 - need_des"]
    #[inline(always)]
    pub const fn power_pd_lpperi_cntl(&self) -> &PowerPdLpperiCntl {
        &self.power_pd_lpperi_cntl
    }
    #[doc = "0x114 - need_des"]
    #[inline(always)]
    pub const fn power_pd_lpperi_mask(&self) -> &PowerPdLpperiMask {
        &self.power_pd_lpperi_mask
    }
    #[doc = "0x118 - need_des"]
    #[inline(always)]
    pub const fn power_hp_pad(&self) -> &PowerHpPad {
        &self.power_hp_pad
    }
    #[doc = "0x11c - need_des"]
    #[inline(always)]
    pub const fn power_ck_wait_cntl(&self) -> &PowerCkWaitCntl {
        &self.power_ck_wait_cntl
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl0(&self) -> &SlpWakeupCntl0 {
        &self.slp_wakeup_cntl0
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl1(&self) -> &SlpWakeupCntl1 {
        &self.slp_wakeup_cntl1
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl2(&self) -> &SlpWakeupCntl2 {
        &self.slp_wakeup_cntl2
    }
    #[doc = "0x12c - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl3(&self) -> &SlpWakeupCntl3 {
        &self.slp_wakeup_cntl3
    }
    #[doc = "0x130 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl4(&self) -> &SlpWakeupCntl4 {
        &self.slp_wakeup_cntl4
    }
    #[doc = "0x134 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl5(&self) -> &SlpWakeupCntl5 {
        &self.slp_wakeup_cntl5
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl6(&self) -> &SlpWakeupCntl6 {
        &self.slp_wakeup_cntl6
    }
    #[doc = "0x13c - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl7(&self) -> &SlpWakeupCntl7 {
        &self.slp_wakeup_cntl7
    }
    #[doc = "0x140 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl8(&self) -> &SlpWakeupCntl8 {
        &self.slp_wakeup_cntl8
    }
    #[doc = "0x144 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_status0(&self) -> &SlpWakeupStatus0 {
        &self.slp_wakeup_status0
    }
    #[doc = "0x148 - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_status1(&self) -> &SlpWakeupStatus1 {
        &self.slp_wakeup_status1
    }
    #[doc = "0x14c - need_des"]
    #[inline(always)]
    pub const fn slp_wakeup_status2(&self) -> &SlpWakeupStatus2 {
        &self.slp_wakeup_status2
    }
    #[doc = "0x150 - need_des"]
    #[inline(always)]
    pub const fn hp_ck_poweron(&self) -> &HpCkPoweron {
        &self.hp_ck_poweron
    }
    #[doc = "0x154 - need_des"]
    #[inline(always)]
    pub const fn hp_ck_cntl(&self) -> &HpCkCntl {
        &self.hp_ck_cntl
    }
    #[doc = "0x158 - need_des"]
    #[inline(always)]
    pub const fn por_status(&self) -> &PorStatus {
        &self.por_status
    }
    #[doc = "0x15c - need_des"]
    #[inline(always)]
    pub const fn rf_pwc(&self) -> &RfPwc {
        &self.rf_pwc
    }
    #[doc = "0x160 - need_des"]
    #[inline(always)]
    pub const fn backup_cfg(&self) -> &BackupCfg {
        &self.backup_cfg
    }
    #[doc = "0x164 - need_des"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x168 - need_des"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x16c - need_des"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x170 - need_des"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x174 - need_des"]
    #[inline(always)]
    pub const fn lp_int_raw(&self) -> &LpIntRaw {
        &self.lp_int_raw
    }
    #[doc = "0x178 - need_des"]
    #[inline(always)]
    pub const fn lp_int_st(&self) -> &LpIntSt {
        &self.lp_int_st
    }
    #[doc = "0x17c - need_des"]
    #[inline(always)]
    pub const fn lp_int_ena(&self) -> &LpIntEna {
        &self.lp_int_ena
    }
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn lp_int_clr(&self) -> &LpIntClr {
        &self.lp_int_clr
    }
    #[doc = "0x184 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_pwr0(&self) -> &LpCpuPwr0 {
        &self.lp_cpu_pwr0
    }
    #[doc = "0x188 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_pwr1(&self) -> &LpCpuPwr1 {
        &self.lp_cpu_pwr1
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_pwr2(&self) -> &LpCpuPwr2 {
        &self.lp_cpu_pwr2
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_pwr3(&self) -> &LpCpuPwr3 {
        &self.lp_cpu_pwr3
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_pwr4(&self) -> &LpCpuPwr4 {
        &self.lp_cpu_pwr4
    }
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_pwr5(&self) -> &LpCpuPwr5 {
        &self.lp_cpu_pwr5
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn hp_lp_cpu_comm(&self) -> &HpLpCpuComm {
        &self.hp_lp_cpu_comm
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn hp_regulator_cfg(&self) -> &HpRegulatorCfg {
        &self.hp_regulator_cfg
    }
    #[doc = "0x1a4 - need_des"]
    #[inline(always)]
    pub const fn main_state(&self) -> &MainState {
        &self.main_state
    }
    #[doc = "0x1a8 - need_des"]
    #[inline(always)]
    pub const fn pwr_state(&self) -> &PwrState {
        &self.pwr_state
    }
    #[doc = "0x1ac - need_des"]
    #[inline(always)]
    pub const fn clk_state0(&self) -> &ClkState0 {
        &self.clk_state0
    }
    #[doc = "0x1b0 - need_des"]
    #[inline(always)]
    pub const fn clk_state1(&self) -> &ClkState1 {
        &self.clk_state1
    }
    #[doc = "0x1b4 - need_des"]
    #[inline(always)]
    pub const fn clk_state2(&self) -> &ClkState2 {
        &self.clk_state2
    }
    #[doc = "0x1b8 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p1a(&self) -> &ExtLdoP0_0p1a {
        &self.ext_ldo_p0_0p1a
    }
    #[doc = "0x1bc - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p1a_ana(&self) -> &ExtLdoP0_0p1aAna {
        &self.ext_ldo_p0_0p1a_ana
    }
    #[doc = "0x1c0 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p2a(&self) -> &ExtLdoP0_0p2a {
        &self.ext_ldo_p0_0p2a
    }
    #[doc = "0x1c4 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p2a_ana(&self) -> &ExtLdoP0_0p2aAna {
        &self.ext_ldo_p0_0p2a_ana
    }
    #[doc = "0x1c8 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p3a(&self) -> &ExtLdoP0_0p3a {
        &self.ext_ldo_p0_0p3a
    }
    #[doc = "0x1cc - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p3a_ana(&self) -> &ExtLdoP0_0p3aAna {
        &self.ext_ldo_p0_0p3a_ana
    }
    #[doc = "0x1d0 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p1a(&self) -> &ExtLdoP1_0p1a {
        &self.ext_ldo_p1_0p1a
    }
    #[doc = "0x1d4 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p1a_ana(&self) -> &ExtLdoP1_0p1aAna {
        &self.ext_ldo_p1_0p1a_ana
    }
    #[doc = "0x1d8 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p2a(&self) -> &ExtLdoP1_0p2a {
        &self.ext_ldo_p1_0p2a
    }
    #[doc = "0x1dc - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p2a_ana(&self) -> &ExtLdoP1_0p2aAna {
        &self.ext_ldo_p1_0p2a_ana
    }
    #[doc = "0x1e0 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p3a(&self) -> &ExtLdoP1_0p3a {
        &self.ext_ldo_p1_0p3a
    }
    #[doc = "0x1e4 - need_des"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p3a_ana(&self) -> &ExtLdoP1_0p3aAna {
        &self.ext_ldo_p1_0p3a_ana
    }
    #[doc = "0x1e8 - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup_lv(&self) -> &ExtWakeupLv {
        &self.ext_wakeup_lv
    }
    #[doc = "0x1ec - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup_sel(&self) -> &ExtWakeupSel {
        &self.ext_wakeup_sel
    }
    #[doc = "0x1f0 - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup_st(&self) -> &ExtWakeupSt {
        &self.ext_wakeup_st
    }
    #[doc = "0x1f4 - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup_cntl(&self) -> &ExtWakeupCntl {
        &self.ext_wakeup_cntl
    }
    #[doc = "0x1f8 - need_des"]
    #[inline(always)]
    pub const fn sdio_wakeup_cntl(&self) -> &SdioWakeupCntl {
        &self.sdio_wakeup_cntl
    }
    #[doc = "0x1fc - need_des"]
    #[inline(always)]
    pub const fn xtal_slp(&self) -> &XtalSlp {
        &self.xtal_slp
    }
    #[doc = "0x200 - need_des"]
    #[inline(always)]
    pub const fn cpu_sw_stall(&self) -> &CpuSwStall {
        &self.cpu_sw_stall
    }
    #[doc = "0x204 - need_des"]
    #[inline(always)]
    pub const fn dcm_ctrl(&self) -> &DcmCtrl {
        &self.dcm_ctrl
    }
    #[doc = "0x208 - need_des"]
    #[inline(always)]
    pub const fn dcm_wait_delay(&self) -> &DcmWaitDelay {
        &self.dcm_wait_delay
    }
    #[doc = "0x20c - need_des"]
    #[inline(always)]
    pub const fn vddbat_cfg(&self) -> &VddbatCfg {
        &self.vddbat_cfg
    }
    #[doc = "0x210 - need_des"]
    #[inline(always)]
    pub const fn touch_pwr_cntl(&self) -> &TouchPwrCntl {
        &self.touch_pwr_cntl
    }
    #[doc = "0x214 - need_des"]
    #[inline(always)]
    pub const fn rdn_eco(&self) -> &RdnEco {
        &self.rdn_eco
    }
    #[doc = "0x218 - need_des"]
    #[inline(always)]
    pub const fn power_pd_hp_cpu_cntl(&self) -> &PowerPdHpCpuCntl {
        &self.power_pd_hp_cpu_cntl
    }
    #[doc = "0x21c - need_des"]
    #[inline(always)]
    pub const fn power_pd_hp_cpu_mask(&self) -> &PowerPdHpCpuMask {
        &self.power_pd_hp_cpu_mask
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "HP_ACTIVE_DIG_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_dig_power`] module"]
#[doc(alias = "HP_ACTIVE_DIG_POWER")]
pub type HpActiveDigPower = crate::Reg<hp_active_dig_power::HpActiveDigPowerSpec>;
#[doc = "need_des"]
pub mod hp_active_dig_power;
#[doc = "HP_ACTIVE_ICG_HP_FUNC (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_icg_hp_func::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_icg_hp_func::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_icg_hp_func`] module"]
#[doc(alias = "HP_ACTIVE_ICG_HP_FUNC")]
pub type HpActiveIcgHpFunc = crate::Reg<hp_active_icg_hp_func::HpActiveIcgHpFuncSpec>;
#[doc = "need_des"]
pub mod hp_active_icg_hp_func;
#[doc = "HP_ACTIVE_ICG_HP_APB (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_icg_hp_apb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_icg_hp_apb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_icg_hp_apb`] module"]
#[doc(alias = "HP_ACTIVE_ICG_HP_APB")]
pub type HpActiveIcgHpApb = crate::Reg<hp_active_icg_hp_apb::HpActiveIcgHpApbSpec>;
#[doc = "need_des"]
pub mod hp_active_icg_hp_apb;
#[doc = "HP_ACTIVE_ICG_MODEM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_icg_modem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_icg_modem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_icg_modem`] module"]
#[doc(alias = "HP_ACTIVE_ICG_MODEM")]
pub type HpActiveIcgModem = crate::Reg<hp_active_icg_modem::HpActiveIcgModemSpec>;
#[doc = "need_des"]
pub mod hp_active_icg_modem;
#[doc = "HP_ACTIVE_HP_SYS_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_sys_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_sys_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_sys_cntl`] module"]
#[doc(alias = "HP_ACTIVE_HP_SYS_CNTL")]
pub type HpActiveHpSysCntl = crate::Reg<hp_active_hp_sys_cntl::HpActiveHpSysCntlSpec>;
#[doc = "need_des"]
pub mod hp_active_hp_sys_cntl;
#[doc = "HP_ACTIVE_HP_CK_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_ck_power`] module"]
#[doc(alias = "HP_ACTIVE_HP_CK_POWER")]
pub type HpActiveHpCkPower = crate::Reg<hp_active_hp_ck_power::HpActiveHpCkPowerSpec>;
#[doc = "need_des"]
pub mod hp_active_hp_ck_power;
#[doc = "HP_ACTIVE_BIAS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_bias`] module"]
#[doc(alias = "HP_ACTIVE_BIAS")]
pub type HpActiveBias = crate::Reg<hp_active_bias::HpActiveBiasSpec>;
#[doc = "need_des"]
pub mod hp_active_bias;
#[doc = "HP_ACTIVE_BACKUP (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_backup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_backup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_backup`] module"]
#[doc(alias = "HP_ACTIVE_BACKUP")]
pub type HpActiveBackup = crate::Reg<hp_active_backup::HpActiveBackupSpec>;
#[doc = "need_des"]
pub mod hp_active_backup;
#[doc = "HP_ACTIVE_BACKUP_CLK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_backup_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_backup_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_backup_clk`] module"]
#[doc(alias = "HP_ACTIVE_BACKUP_CLK")]
pub type HpActiveBackupClk = crate::Reg<hp_active_backup_clk::HpActiveBackupClkSpec>;
#[doc = "need_des"]
pub mod hp_active_backup_clk;
#[doc = "HP_ACTIVE_SYSCLK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_sysclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_sysclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_sysclk`] module"]
#[doc(alias = "HP_ACTIVE_SYSCLK")]
pub type HpActiveSysclk = crate::Reg<hp_active_sysclk::HpActiveSysclkSpec>;
#[doc = "need_des"]
pub mod hp_active_sysclk;
#[doc = "HP_ACTIVE_HP_REGULATOR0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_regulator0`] module"]
#[doc(alias = "HP_ACTIVE_HP_REGULATOR0")]
pub type HpActiveHpRegulator0 = crate::Reg<hp_active_hp_regulator0::HpActiveHpRegulator0Spec>;
#[doc = "need_des"]
pub mod hp_active_hp_regulator0;
#[doc = "HP_ACTIVE_HP_REGULATOR1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_regulator1`] module"]
#[doc(alias = "HP_ACTIVE_HP_REGULATOR1")]
pub type HpActiveHpRegulator1 = crate::Reg<hp_active_hp_regulator1::HpActiveHpRegulator1Spec>;
#[doc = "need_des"]
pub mod hp_active_hp_regulator1;
#[doc = "HP_ACTIVE_XTAL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_xtal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_xtal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_xtal`] module"]
#[doc(alias = "HP_ACTIVE_XTAL")]
pub type HpActiveXtal = crate::Reg<hp_active_xtal::HpActiveXtalSpec>;
#[doc = "need_des"]
pub mod hp_active_xtal;
#[doc = "HP_MODEM_DIG_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_modem_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_dig_power`] module"]
#[doc(alias = "HP_MODEM_DIG_POWER")]
pub type HpModemDigPower = crate::Reg<hp_modem_dig_power::HpModemDigPowerSpec>;
#[doc = "need_des"]
pub mod hp_modem_dig_power;
#[doc = "HP_MODEM_ICG_HP_FUNC (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_hp_func::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_icg_hp_func`] module"]
#[doc(alias = "HP_MODEM_ICG_HP_FUNC")]
pub type HpModemIcgHpFunc = crate::Reg<hp_modem_icg_hp_func::HpModemIcgHpFuncSpec>;
#[doc = "need_des"]
pub mod hp_modem_icg_hp_func;
#[doc = "HP_MODEM_ICG_HP_APB (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_hp_apb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_icg_hp_apb`] module"]
#[doc(alias = "HP_MODEM_ICG_HP_APB")]
pub type HpModemIcgHpApb = crate::Reg<hp_modem_icg_hp_apb::HpModemIcgHpApbSpec>;
#[doc = "need_des"]
pub mod hp_modem_icg_hp_apb;
#[doc = "HP_MODEM_ICG_MODEM (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_modem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_icg_modem`] module"]
#[doc(alias = "HP_MODEM_ICG_MODEM")]
pub type HpModemIcgModem = crate::Reg<hp_modem_icg_modem::HpModemIcgModemSpec>;
#[doc = "need_des"]
pub mod hp_modem_icg_modem;
#[doc = "HP_MODEM_HP_SYS_CNTL (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_sys_cntl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_sys_cntl`] module"]
#[doc(alias = "HP_MODEM_HP_SYS_CNTL")]
pub type HpModemHpSysCntl = crate::Reg<hp_modem_hp_sys_cntl::HpModemHpSysCntlSpec>;
#[doc = "need_des"]
pub mod hp_modem_hp_sys_cntl;
#[doc = "HP_MODEM_HP_CK_POWER (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_ck_power::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_ck_power`] module"]
#[doc(alias = "HP_MODEM_HP_CK_POWER")]
pub type HpModemHpCkPower = crate::Reg<hp_modem_hp_ck_power::HpModemHpCkPowerSpec>;
#[doc = "need_des"]
pub mod hp_modem_hp_ck_power;
#[doc = "HP_MODEM_BIAS (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_bias::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_bias`] module"]
#[doc(alias = "HP_MODEM_BIAS")]
pub type HpModemBias = crate::Reg<hp_modem_bias::HpModemBiasSpec>;
#[doc = "need_des"]
pub mod hp_modem_bias;
#[doc = "HP_MODEM_BACKUP (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_backup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_backup`] module"]
#[doc(alias = "HP_MODEM_BACKUP")]
pub type HpModemBackup = crate::Reg<hp_modem_backup::HpModemBackupSpec>;
#[doc = "need_des"]
pub mod hp_modem_backup;
#[doc = "HP_MODEM_BACKUP_CLK (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_backup_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_backup_clk`] module"]
#[doc(alias = "HP_MODEM_BACKUP_CLK")]
pub type HpModemBackupClk = crate::Reg<hp_modem_backup_clk::HpModemBackupClkSpec>;
#[doc = "need_des"]
pub mod hp_modem_backup_clk;
#[doc = "HP_MODEM_SYSCLK (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_sysclk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_sysclk`] module"]
#[doc(alias = "HP_MODEM_SYSCLK")]
pub type HpModemSysclk = crate::Reg<hp_modem_sysclk::HpModemSysclkSpec>;
#[doc = "need_des"]
pub mod hp_modem_sysclk;
#[doc = "HP_MODEM_HP_REGULATOR0 (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_regulator0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_regulator0`] module"]
#[doc(alias = "HP_MODEM_HP_REGULATOR0")]
pub type HpModemHpRegulator0 = crate::Reg<hp_modem_hp_regulator0::HpModemHpRegulator0Spec>;
#[doc = "need_des"]
pub mod hp_modem_hp_regulator0;
#[doc = "HP_MODEM_HP_REGULATOR1 (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_regulator1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_regulator1`] module"]
#[doc(alias = "HP_MODEM_HP_REGULATOR1")]
pub type HpModemHpRegulator1 = crate::Reg<hp_modem_hp_regulator1::HpModemHpRegulator1Spec>;
#[doc = "need_des"]
pub mod hp_modem_hp_regulator1;
#[doc = "HP_MODEM_XTAL (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_xtal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_xtal`] module"]
#[doc(alias = "HP_MODEM_XTAL")]
pub type HpModemXtal = crate::Reg<hp_modem_xtal::HpModemXtalSpec>;
#[doc = "need_des"]
pub mod hp_modem_xtal;
#[doc = "HP_SLEEP_DIG_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_dig_power`] module"]
#[doc(alias = "HP_SLEEP_DIG_POWER")]
pub type HpSleepDigPower = crate::Reg<hp_sleep_dig_power::HpSleepDigPowerSpec>;
#[doc = "need_des"]
pub mod hp_sleep_dig_power;
#[doc = "HP_SLEEP_ICG_HP_FUNC (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_icg_hp_func::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_icg_hp_func::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_icg_hp_func`] module"]
#[doc(alias = "HP_SLEEP_ICG_HP_FUNC")]
pub type HpSleepIcgHpFunc = crate::Reg<hp_sleep_icg_hp_func::HpSleepIcgHpFuncSpec>;
#[doc = "need_des"]
pub mod hp_sleep_icg_hp_func;
#[doc = "HP_SLEEP_ICG_HP_APB (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_icg_hp_apb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_icg_hp_apb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_icg_hp_apb`] module"]
#[doc(alias = "HP_SLEEP_ICG_HP_APB")]
pub type HpSleepIcgHpApb = crate::Reg<hp_sleep_icg_hp_apb::HpSleepIcgHpApbSpec>;
#[doc = "need_des"]
pub mod hp_sleep_icg_hp_apb;
#[doc = "HP_SLEEP_ICG_MODEM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_icg_modem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_icg_modem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_icg_modem`] module"]
#[doc(alias = "HP_SLEEP_ICG_MODEM")]
pub type HpSleepIcgModem = crate::Reg<hp_sleep_icg_modem::HpSleepIcgModemSpec>;
#[doc = "need_des"]
pub mod hp_sleep_icg_modem;
#[doc = "HP_SLEEP_HP_SYS_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_sys_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_sys_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_sys_cntl`] module"]
#[doc(alias = "HP_SLEEP_HP_SYS_CNTL")]
pub type HpSleepHpSysCntl = crate::Reg<hp_sleep_hp_sys_cntl::HpSleepHpSysCntlSpec>;
#[doc = "need_des"]
pub mod hp_sleep_hp_sys_cntl;
#[doc = "HP_SLEEP_HP_CK_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_ck_power`] module"]
#[doc(alias = "HP_SLEEP_HP_CK_POWER")]
pub type HpSleepHpCkPower = crate::Reg<hp_sleep_hp_ck_power::HpSleepHpCkPowerSpec>;
#[doc = "need_des"]
pub mod hp_sleep_hp_ck_power;
#[doc = "HP_SLEEP_BIAS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_bias`] module"]
#[doc(alias = "HP_SLEEP_BIAS")]
pub type HpSleepBias = crate::Reg<hp_sleep_bias::HpSleepBiasSpec>;
#[doc = "need_des"]
pub mod hp_sleep_bias;
#[doc = "HP_SLEEP_BACKUP (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_backup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_backup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_backup`] module"]
#[doc(alias = "HP_SLEEP_BACKUP")]
pub type HpSleepBackup = crate::Reg<hp_sleep_backup::HpSleepBackupSpec>;
#[doc = "need_des"]
pub mod hp_sleep_backup;
#[doc = "HP_SLEEP_BACKUP_CLK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_backup_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_backup_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_backup_clk`] module"]
#[doc(alias = "HP_SLEEP_BACKUP_CLK")]
pub type HpSleepBackupClk = crate::Reg<hp_sleep_backup_clk::HpSleepBackupClkSpec>;
#[doc = "need_des"]
pub mod hp_sleep_backup_clk;
#[doc = "HP_SLEEP_SYSCLK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_sysclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_sysclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_sysclk`] module"]
#[doc(alias = "HP_SLEEP_SYSCLK")]
pub type HpSleepSysclk = crate::Reg<hp_sleep_sysclk::HpSleepSysclkSpec>;
#[doc = "need_des"]
pub mod hp_sleep_sysclk;
#[doc = "HP_SLEEP_HP_REGULATOR0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_regulator0`] module"]
#[doc(alias = "HP_SLEEP_HP_REGULATOR0")]
pub type HpSleepHpRegulator0 = crate::Reg<hp_sleep_hp_regulator0::HpSleepHpRegulator0Spec>;
#[doc = "need_des"]
pub mod hp_sleep_hp_regulator0;
#[doc = "HP_SLEEP_HP_REGULATOR1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_regulator1`] module"]
#[doc(alias = "HP_SLEEP_HP_REGULATOR1")]
pub type HpSleepHpRegulator1 = crate::Reg<hp_sleep_hp_regulator1::HpSleepHpRegulator1Spec>;
#[doc = "need_des"]
pub mod hp_sleep_hp_regulator1;
#[doc = "HP_SLEEP_XTAL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_xtal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_xtal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_xtal`] module"]
#[doc(alias = "HP_SLEEP_XTAL")]
pub type HpSleepXtal = crate::Reg<hp_sleep_xtal::HpSleepXtalSpec>;
#[doc = "need_des"]
pub mod hp_sleep_xtal;
#[doc = "HP_SLEEP_LP_REGULATOR0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_regulator0`] module"]
#[doc(alias = "HP_SLEEP_LP_REGULATOR0")]
pub type HpSleepLpRegulator0 = crate::Reg<hp_sleep_lp_regulator0::HpSleepLpRegulator0Spec>;
#[doc = "need_des"]
pub mod hp_sleep_lp_regulator0;
#[doc = "HP_SLEEP_LP_REGULATOR1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_regulator1`] module"]
#[doc(alias = "HP_SLEEP_LP_REGULATOR1")]
pub type HpSleepLpRegulator1 = crate::Reg<hp_sleep_lp_regulator1::HpSleepLpRegulator1Spec>;
#[doc = "need_des"]
pub mod hp_sleep_lp_regulator1;
#[doc = "HP_SLEEP_LP_DCDC_RESERVE (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_dcdc_reserve::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_dcdc_reserve`] module"]
#[doc(alias = "HP_SLEEP_LP_DCDC_RESERVE")]
pub type HpSleepLpDcdcReserve = crate::Reg<hp_sleep_lp_dcdc_reserve::HpSleepLpDcdcReserveSpec>;
#[doc = "need_des"]
pub mod hp_sleep_lp_dcdc_reserve;
#[doc = "HP_SLEEP_LP_DIG_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_dig_power`] module"]
#[doc(alias = "HP_SLEEP_LP_DIG_POWER")]
pub type HpSleepLpDigPower = crate::Reg<hp_sleep_lp_dig_power::HpSleepLpDigPowerSpec>;
#[doc = "need_des"]
pub mod hp_sleep_lp_dig_power;
#[doc = "HP_SLEEP_LP_CK_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_ck_power`] module"]
#[doc(alias = "HP_SLEEP_LP_CK_POWER")]
pub type HpSleepLpCkPower = crate::Reg<hp_sleep_lp_ck_power::HpSleepLpCkPowerSpec>;
#[doc = "need_des"]
pub mod hp_sleep_lp_ck_power;
#[doc = "LP_SLEEP_LP_BIAS_RESERVE (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_bias_reserve::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_bias_reserve`] module"]
#[doc(alias = "LP_SLEEP_LP_BIAS_RESERVE")]
pub type LpSleepLpBiasReserve = crate::Reg<lp_sleep_lp_bias_reserve::LpSleepLpBiasReserveSpec>;
#[doc = "need_des"]
pub mod lp_sleep_lp_bias_reserve;
#[doc = "LP_SLEEP_LP_REGULATOR0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_regulator0`] module"]
#[doc(alias = "LP_SLEEP_LP_REGULATOR0")]
pub type LpSleepLpRegulator0 = crate::Reg<lp_sleep_lp_regulator0::LpSleepLpRegulator0Spec>;
#[doc = "need_des"]
pub mod lp_sleep_lp_regulator0;
#[doc = "LP_SLEEP_LP_REGULATOR1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_regulator1`] module"]
#[doc(alias = "LP_SLEEP_LP_REGULATOR1")]
pub type LpSleepLpRegulator1 = crate::Reg<lp_sleep_lp_regulator1::LpSleepLpRegulator1Spec>;
#[doc = "need_des"]
pub mod lp_sleep_lp_regulator1;
#[doc = "LP_SLEEP_XTAL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_xtal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_xtal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_xtal`] module"]
#[doc(alias = "LP_SLEEP_XTAL")]
pub type LpSleepXtal = crate::Reg<lp_sleep_xtal::LpSleepXtalSpec>;
#[doc = "need_des"]
pub mod lp_sleep_xtal;
#[doc = "LP_SLEEP_LP_DIG_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_dig_power`] module"]
#[doc(alias = "LP_SLEEP_LP_DIG_POWER")]
pub type LpSleepLpDigPower = crate::Reg<lp_sleep_lp_dig_power::LpSleepLpDigPowerSpec>;
#[doc = "need_des"]
pub mod lp_sleep_lp_dig_power;
#[doc = "LP_SLEEP_LP_CK_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_ck_power`] module"]
#[doc(alias = "LP_SLEEP_LP_CK_POWER")]
pub type LpSleepLpCkPower = crate::Reg<lp_sleep_lp_ck_power::LpSleepLpCkPowerSpec>;
#[doc = "need_des"]
pub mod lp_sleep_lp_ck_power;
#[doc = "LP_SLEEP_BIAS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_bias`] module"]
#[doc(alias = "LP_SLEEP_BIAS")]
pub type LpSleepBias = crate::Reg<lp_sleep_bias::LpSleepBiasSpec>;
#[doc = "need_des"]
pub mod lp_sleep_bias;
#[doc = "IMM_HP_CK_POWER (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_hp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_hp_ck_power`] module"]
#[doc(alias = "IMM_HP_CK_POWER")]
pub type ImmHpCkPower = crate::Reg<imm_hp_ck_power::ImmHpCkPowerSpec>;
#[doc = "need_des"]
pub mod imm_hp_ck_power;
#[doc = "IMM_SLEEP_SYSCLK (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_sleep_sysclk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_sleep_sysclk`] module"]
#[doc(alias = "IMM_SLEEP_SYSCLK")]
pub type ImmSleepSysclk = crate::Reg<imm_sleep_sysclk::ImmSleepSysclkSpec>;
#[doc = "need_des"]
pub mod imm_sleep_sysclk;
#[doc = "IMM_HP_FUNC_ICG (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_func_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_hp_func_icg`] module"]
#[doc(alias = "IMM_HP_FUNC_ICG")]
pub type ImmHpFuncIcg = crate::Reg<imm_hp_func_icg::ImmHpFuncIcgSpec>;
#[doc = "need_des"]
pub mod imm_hp_func_icg;
#[doc = "IMM_HP_APB_ICG (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_apb_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_hp_apb_icg`] module"]
#[doc(alias = "IMM_HP_APB_ICG")]
pub type ImmHpApbIcg = crate::Reg<imm_hp_apb_icg::ImmHpApbIcgSpec>;
#[doc = "need_des"]
pub mod imm_hp_apb_icg;
#[doc = "IMM_MODEM_ICG (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_modem_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_modem_icg`] module"]
#[doc(alias = "IMM_MODEM_ICG")]
pub type ImmModemIcg = crate::Reg<imm_modem_icg::ImmModemIcgSpec>;
#[doc = "need_des"]
pub mod imm_modem_icg;
#[doc = "IMM_LP_ICG (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_lp_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_lp_icg`] module"]
#[doc(alias = "IMM_LP_ICG")]
pub type ImmLpIcg = crate::Reg<imm_lp_icg::ImmLpIcgSpec>;
#[doc = "need_des"]
pub mod imm_lp_icg;
#[doc = "IMM_PAD_HOLD_ALL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_pad_hold_all::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_pad_hold_all::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_pad_hold_all`] module"]
#[doc(alias = "IMM_PAD_HOLD_ALL")]
pub type ImmPadHoldAll = crate::Reg<imm_pad_hold_all::ImmPadHoldAllSpec>;
#[doc = "need_des"]
pub mod imm_pad_hold_all;
#[doc = "IMM_I2C_ISO (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_i2c_iso::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_i2c_iso`] module"]
#[doc(alias = "IMM_I2C_ISO")]
pub type ImmI2cIso = crate::Reg<imm_i2c_iso::ImmI2cIsoSpec>;
#[doc = "need_des"]
pub mod imm_i2c_iso;
#[doc = "POWER_WAIT_TIMER0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_wait_timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_wait_timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_wait_timer0`] module"]
#[doc(alias = "POWER_WAIT_TIMER0")]
pub type PowerWaitTimer0 = crate::Reg<power_wait_timer0::PowerWaitTimer0Spec>;
#[doc = "need_des"]
pub mod power_wait_timer0;
#[doc = "POWER_WAIT_TIMER1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_wait_timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_wait_timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_wait_timer1`] module"]
#[doc(alias = "POWER_WAIT_TIMER1")]
pub type PowerWaitTimer1 = crate::Reg<power_wait_timer1::PowerWaitTimer1Spec>;
#[doc = "need_des"]
pub mod power_wait_timer1;
#[doc = "POWER_PD_TOP_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_top_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_top_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_top_cntl`] module"]
#[doc(alias = "POWER_PD_TOP_CNTL")]
pub type PowerPdTopCntl = crate::Reg<power_pd_top_cntl::PowerPdTopCntlSpec>;
#[doc = "need_des"]
pub mod power_pd_top_cntl;
#[doc = "POWER_PD_CNNT_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_cnnt_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_cnnt_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_cnnt_cntl`] module"]
#[doc(alias = "POWER_PD_CNNT_CNTL")]
pub type PowerPdCnntCntl = crate::Reg<power_pd_cnnt_cntl::PowerPdCnntCntlSpec>;
#[doc = "need_des"]
pub mod power_pd_cnnt_cntl;
#[doc = "POWER_PD_HPMEM_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hpmem_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hpmem_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hpmem_cntl`] module"]
#[doc(alias = "POWER_PD_HPMEM_CNTL")]
pub type PowerPdHpmemCntl = crate::Reg<power_pd_hpmem_cntl::PowerPdHpmemCntlSpec>;
#[doc = "need_des"]
pub mod power_pd_hpmem_cntl;
#[doc = "POWER_PD_TOP_MASK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_top_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_top_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_top_mask`] module"]
#[doc(alias = "POWER_PD_TOP_MASK")]
pub type PowerPdTopMask = crate::Reg<power_pd_top_mask::PowerPdTopMaskSpec>;
#[doc = "need_des"]
pub mod power_pd_top_mask;
#[doc = "POWER_PD_CNNT_MASK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_cnnt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_cnnt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_cnnt_mask`] module"]
#[doc(alias = "POWER_PD_CNNT_MASK")]
pub type PowerPdCnntMask = crate::Reg<power_pd_cnnt_mask::PowerPdCnntMaskSpec>;
#[doc = "need_des"]
pub mod power_pd_cnnt_mask;
#[doc = "POWER_PD_HPMEM_MASK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hpmem_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hpmem_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hpmem_mask`] module"]
#[doc(alias = "POWER_PD_HPMEM_MASK")]
pub type PowerPdHpmemMask = crate::Reg<power_pd_hpmem_mask::PowerPdHpmemMaskSpec>;
#[doc = "need_des"]
pub mod power_pd_hpmem_mask;
#[doc = "POWER_DCDC_SWITCH (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_dcdc_switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_dcdc_switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_dcdc_switch`] module"]
#[doc(alias = "POWER_DCDC_SWITCH")]
pub type PowerDcdcSwitch = crate::Reg<power_dcdc_switch::PowerDcdcSwitchSpec>;
#[doc = "need_des"]
pub mod power_dcdc_switch;
#[doc = "POWER_PD_LPPERI_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_lpperi_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_lpperi_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_lpperi_cntl`] module"]
#[doc(alias = "POWER_PD_LPPERI_CNTL")]
pub type PowerPdLpperiCntl = crate::Reg<power_pd_lpperi_cntl::PowerPdLpperiCntlSpec>;
#[doc = "need_des"]
pub mod power_pd_lpperi_cntl;
#[doc = "POWER_PD_LPPERI_MASK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_lpperi_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_lpperi_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_lpperi_mask`] module"]
#[doc(alias = "POWER_PD_LPPERI_MASK")]
pub type PowerPdLpperiMask = crate::Reg<power_pd_lpperi_mask::PowerPdLpperiMaskSpec>;
#[doc = "need_des"]
pub mod power_pd_lpperi_mask;
#[doc = "POWER_HP_PAD (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_hp_pad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_hp_pad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_hp_pad`] module"]
#[doc(alias = "POWER_HP_PAD")]
pub type PowerHpPad = crate::Reg<power_hp_pad::PowerHpPadSpec>;
#[doc = "need_des"]
pub mod power_hp_pad;
#[doc = "POWER_CK_WAIT_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_ck_wait_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_ck_wait_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ck_wait_cntl`] module"]
#[doc(alias = "POWER_CK_WAIT_CNTL")]
pub type PowerCkWaitCntl = crate::Reg<power_ck_wait_cntl::PowerCkWaitCntlSpec>;
#[doc = "need_des"]
pub mod power_ck_wait_cntl;
#[doc = "SLP_WAKEUP_CNTL0 (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl0`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL0")]
pub type SlpWakeupCntl0 = crate::Reg<slp_wakeup_cntl0::SlpWakeupCntl0Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl0;
#[doc = "SLP_WAKEUP_CNTL1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl1`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL1")]
pub type SlpWakeupCntl1 = crate::Reg<slp_wakeup_cntl1::SlpWakeupCntl1Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl1;
#[doc = "SLP_WAKEUP_CNTL2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl2`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL2")]
pub type SlpWakeupCntl2 = crate::Reg<slp_wakeup_cntl2::SlpWakeupCntl2Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl2;
#[doc = "SLP_WAKEUP_CNTL3 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl3`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL3")]
pub type SlpWakeupCntl3 = crate::Reg<slp_wakeup_cntl3::SlpWakeupCntl3Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl3;
#[doc = "SLP_WAKEUP_CNTL4 (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl4`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL4")]
pub type SlpWakeupCntl4 = crate::Reg<slp_wakeup_cntl4::SlpWakeupCntl4Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl4;
#[doc = "SLP_WAKEUP_CNTL5 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl5`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL5")]
pub type SlpWakeupCntl5 = crate::Reg<slp_wakeup_cntl5::SlpWakeupCntl5Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl5;
#[doc = "SLP_WAKEUP_CNTL6 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl6`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL6")]
pub type SlpWakeupCntl6 = crate::Reg<slp_wakeup_cntl6::SlpWakeupCntl6Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl6;
#[doc = "SLP_WAKEUP_CNTL7 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl7`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL7")]
pub type SlpWakeupCntl7 = crate::Reg<slp_wakeup_cntl7::SlpWakeupCntl7Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl7;
#[doc = "SLP_WAKEUP_CNTL8 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl8`] module"]
#[doc(alias = "SLP_WAKEUP_CNTL8")]
pub type SlpWakeupCntl8 = crate::Reg<slp_wakeup_cntl8::SlpWakeupCntl8Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl8;
#[doc = "SLP_WAKEUP_STATUS0 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_status0`] module"]
#[doc(alias = "SLP_WAKEUP_STATUS0")]
pub type SlpWakeupStatus0 = crate::Reg<slp_wakeup_status0::SlpWakeupStatus0Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_status0;
#[doc = "SLP_WAKEUP_STATUS1 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_status1`] module"]
#[doc(alias = "SLP_WAKEUP_STATUS1")]
pub type SlpWakeupStatus1 = crate::Reg<slp_wakeup_status1::SlpWakeupStatus1Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_status1;
#[doc = "SLP_WAKEUP_STATUS2 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_status2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_status2`] module"]
#[doc(alias = "SLP_WAKEUP_STATUS2")]
pub type SlpWakeupStatus2 = crate::Reg<slp_wakeup_status2::SlpWakeupStatus2Spec>;
#[doc = "need_des"]
pub mod slp_wakeup_status2;
#[doc = "HP_CK_POWERON (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ck_poweron::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ck_poweron::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ck_poweron`] module"]
#[doc(alias = "HP_CK_POWERON")]
pub type HpCkPoweron = crate::Reg<hp_ck_poweron::HpCkPoweronSpec>;
#[doc = "need_des"]
pub mod hp_ck_poweron;
#[doc = "HP_CK_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ck_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ck_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ck_cntl`] module"]
#[doc(alias = "HP_CK_CNTL")]
pub type HpCkCntl = crate::Reg<hp_ck_cntl::HpCkCntlSpec>;
#[doc = "need_des"]
pub mod hp_ck_cntl;
#[doc = "POR_STATUS (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`por_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@por_status`] module"]
#[doc(alias = "POR_STATUS")]
pub type PorStatus = crate::Reg<por_status::PorStatusSpec>;
#[doc = "need_des"]
pub mod por_status;
#[doc = "RF_PWC (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_pwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_pwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_pwc`] module"]
#[doc(alias = "RF_PWC")]
pub type RfPwc = crate::Reg<rf_pwc::RfPwcSpec>;
#[doc = "need_des"]
pub mod rf_pwc;
#[doc = "BACKUP_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_cfg`] module"]
#[doc(alias = "BACKUP_CFG")]
pub type BackupCfg = crate::Reg<backup_cfg::BackupCfgSpec>;
#[doc = "need_des"]
pub mod backup_cfg;
#[doc = "INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "need_des"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "need_des"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "need_des"]
pub mod int_clr;
#[doc = "LP_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_raw`] module"]
#[doc(alias = "LP_INT_RAW")]
pub type LpIntRaw = crate::Reg<lp_int_raw::LpIntRawSpec>;
#[doc = "need_des"]
pub mod lp_int_raw;
#[doc = "LP_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_st`] module"]
#[doc(alias = "LP_INT_ST")]
pub type LpIntSt = crate::Reg<lp_int_st::LpIntStSpec>;
#[doc = "need_des"]
pub mod lp_int_st;
#[doc = "LP_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_ena`] module"]
#[doc(alias = "LP_INT_ENA")]
pub type LpIntEna = crate::Reg<lp_int_ena::LpIntEnaSpec>;
#[doc = "need_des"]
pub mod lp_int_ena;
#[doc = "LP_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_clr`] module"]
#[doc(alias = "LP_INT_CLR")]
pub type LpIntClr = crate::Reg<lp_int_clr::LpIntClrSpec>;
#[doc = "need_des"]
pub mod lp_int_clr;
#[doc = "LP_CPU_PWR0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr0`] module"]
#[doc(alias = "LP_CPU_PWR0")]
pub type LpCpuPwr0 = crate::Reg<lp_cpu_pwr0::LpCpuPwr0Spec>;
#[doc = "need_des"]
pub mod lp_cpu_pwr0;
#[doc = "LP_CPU_PWR1 (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr1`] module"]
#[doc(alias = "LP_CPU_PWR1")]
pub type LpCpuPwr1 = crate::Reg<lp_cpu_pwr1::LpCpuPwr1Spec>;
#[doc = "need_des"]
pub mod lp_cpu_pwr1;
#[doc = "LP_CPU_PWR2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr2`] module"]
#[doc(alias = "LP_CPU_PWR2")]
pub type LpCpuPwr2 = crate::Reg<lp_cpu_pwr2::LpCpuPwr2Spec>;
#[doc = "need_des"]
pub mod lp_cpu_pwr2;
#[doc = "LP_CPU_PWR3 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr3`] module"]
#[doc(alias = "LP_CPU_PWR3")]
pub type LpCpuPwr3 = crate::Reg<lp_cpu_pwr3::LpCpuPwr3Spec>;
#[doc = "need_des"]
pub mod lp_cpu_pwr3;
#[doc = "LP_CPU_PWR4 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr4`] module"]
#[doc(alias = "LP_CPU_PWR4")]
pub type LpCpuPwr4 = crate::Reg<lp_cpu_pwr4::LpCpuPwr4Spec>;
#[doc = "need_des"]
pub mod lp_cpu_pwr4;
#[doc = "LP_CPU_PWR5 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr5`] module"]
#[doc(alias = "LP_CPU_PWR5")]
pub type LpCpuPwr5 = crate::Reg<lp_cpu_pwr5::LpCpuPwr5Spec>;
#[doc = "need_des"]
pub mod lp_cpu_pwr5;
#[doc = "HP_LP_CPU_COMM (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_lp_cpu_comm::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_lp_cpu_comm`] module"]
#[doc(alias = "HP_LP_CPU_COMM")]
pub type HpLpCpuComm = crate::Reg<hp_lp_cpu_comm::HpLpCpuCommSpec>;
#[doc = "need_des"]
pub mod hp_lp_cpu_comm;
#[doc = "HP_REGULATOR_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_regulator_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_regulator_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_regulator_cfg`] module"]
#[doc(alias = "HP_REGULATOR_CFG")]
pub type HpRegulatorCfg = crate::Reg<hp_regulator_cfg::HpRegulatorCfgSpec>;
#[doc = "need_des"]
pub mod hp_regulator_cfg;
#[doc = "MAIN_STATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`main_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_state`] module"]
#[doc(alias = "MAIN_STATE")]
pub type MainState = crate::Reg<main_state::MainStateSpec>;
#[doc = "need_des"]
pub mod main_state;
#[doc = "PWR_STATE (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_state`] module"]
#[doc(alias = "PWR_STATE")]
pub type PwrState = crate::Reg<pwr_state::PwrStateSpec>;
#[doc = "need_des"]
pub mod pwr_state;
#[doc = "CLK_STATE0 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_state0`] module"]
#[doc(alias = "CLK_STATE0")]
pub type ClkState0 = crate::Reg<clk_state0::ClkState0Spec>;
#[doc = "need_des"]
pub mod clk_state0;
#[doc = "CLK_STATE1 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_state1`] module"]
#[doc(alias = "CLK_STATE1")]
pub type ClkState1 = crate::Reg<clk_state1::ClkState1Spec>;
#[doc = "need_des"]
pub mod clk_state1;
#[doc = "CLK_STATE2 (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_state2`] module"]
#[doc(alias = "CLK_STATE2")]
pub type ClkState2 = crate::Reg<clk_state2::ClkState2Spec>;
#[doc = "need_des"]
pub mod clk_state2;
#[doc = "EXT_LDO_P0_0P1A (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p1a`] module"]
#[doc(alias = "EXT_LDO_P0_0P1A")]
pub type ExtLdoP0_0p1a = crate::Reg<ext_ldo_p0_0p1a::ExtLdoP0_0p1aSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p0_0p1a;
#[doc = "EXT_LDO_P0_0P1A_ANA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p1a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p1a_ana`] module"]
#[doc(alias = "EXT_LDO_P0_0P1A_ANA")]
pub type ExtLdoP0_0p1aAna = crate::Reg<ext_ldo_p0_0p1a_ana::ExtLdoP0_0p1aAnaSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p0_0p1a_ana;
#[doc = "EXT_LDO_P0_0P2A (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p2a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p2a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p2a`] module"]
#[doc(alias = "EXT_LDO_P0_0P2A")]
pub type ExtLdoP0_0p2a = crate::Reg<ext_ldo_p0_0p2a::ExtLdoP0_0p2aSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p0_0p2a;
#[doc = "EXT_LDO_P0_0P2A_ANA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p2a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p2a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p2a_ana`] module"]
#[doc(alias = "EXT_LDO_P0_0P2A_ANA")]
pub type ExtLdoP0_0p2aAna = crate::Reg<ext_ldo_p0_0p2a_ana::ExtLdoP0_0p2aAnaSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p0_0p2a_ana;
#[doc = "EXT_LDO_P0_0P3A (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p3a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p3a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p3a`] module"]
#[doc(alias = "EXT_LDO_P0_0P3A")]
pub type ExtLdoP0_0p3a = crate::Reg<ext_ldo_p0_0p3a::ExtLdoP0_0p3aSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p0_0p3a;
#[doc = "EXT_LDO_P0_0P3A_ANA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p3a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p3a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p3a_ana`] module"]
#[doc(alias = "EXT_LDO_P0_0P3A_ANA")]
pub type ExtLdoP0_0p3aAna = crate::Reg<ext_ldo_p0_0p3a_ana::ExtLdoP0_0p3aAnaSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p0_0p3a_ana;
#[doc = "EXT_LDO_P1_0P1A (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p1a`] module"]
#[doc(alias = "EXT_LDO_P1_0P1A")]
pub type ExtLdoP1_0p1a = crate::Reg<ext_ldo_p1_0p1a::ExtLdoP1_0p1aSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p1_0p1a;
#[doc = "EXT_LDO_P1_0P1A_ANA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p1a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p1a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p1a_ana`] module"]
#[doc(alias = "EXT_LDO_P1_0P1A_ANA")]
pub type ExtLdoP1_0p1aAna = crate::Reg<ext_ldo_p1_0p1a_ana::ExtLdoP1_0p1aAnaSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p1_0p1a_ana;
#[doc = "EXT_LDO_P1_0P2A (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p2a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p2a`] module"]
#[doc(alias = "EXT_LDO_P1_0P2A")]
pub type ExtLdoP1_0p2a = crate::Reg<ext_ldo_p1_0p2a::ExtLdoP1_0p2aSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p1_0p2a;
#[doc = "EXT_LDO_P1_0P2A_ANA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p2a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p2a_ana`] module"]
#[doc(alias = "EXT_LDO_P1_0P2A_ANA")]
pub type ExtLdoP1_0p2aAna = crate::Reg<ext_ldo_p1_0p2a_ana::ExtLdoP1_0p2aAnaSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p1_0p2a_ana;
#[doc = "EXT_LDO_P1_0P3A (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p3a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p3a`] module"]
#[doc(alias = "EXT_LDO_P1_0P3A")]
pub type ExtLdoP1_0p3a = crate::Reg<ext_ldo_p1_0p3a::ExtLdoP1_0p3aSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p1_0p3a;
#[doc = "EXT_LDO_P1_0P3A_ANA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p3a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p3a_ana`] module"]
#[doc(alias = "EXT_LDO_P1_0P3A_ANA")]
pub type ExtLdoP1_0p3aAna = crate::Reg<ext_ldo_p1_0p3a_ana::ExtLdoP1_0p3aAnaSpec>;
#[doc = "need_des"]
pub mod ext_ldo_p1_0p3a_ana;
#[doc = "EXT_WAKEUP_LV (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_lv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_lv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_lv`] module"]
#[doc(alias = "EXT_WAKEUP_LV")]
pub type ExtWakeupLv = crate::Reg<ext_wakeup_lv::ExtWakeupLvSpec>;
#[doc = "need_des"]
pub mod ext_wakeup_lv;
#[doc = "EXT_WAKEUP_SEL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_sel`] module"]
#[doc(alias = "EXT_WAKEUP_SEL")]
pub type ExtWakeupSel = crate::Reg<ext_wakeup_sel::ExtWakeupSelSpec>;
#[doc = "need_des"]
pub mod ext_wakeup_sel;
#[doc = "EXT_WAKEUP_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_st`] module"]
#[doc(alias = "EXT_WAKEUP_ST")]
pub type ExtWakeupSt = crate::Reg<ext_wakeup_st::ExtWakeupStSpec>;
#[doc = "need_des"]
pub mod ext_wakeup_st;
#[doc = "EXT_WAKEUP_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_cntl`] module"]
#[doc(alias = "EXT_WAKEUP_CNTL")]
pub type ExtWakeupCntl = crate::Reg<ext_wakeup_cntl::ExtWakeupCntlSpec>;
#[doc = "need_des"]
pub mod ext_wakeup_cntl;
#[doc = "SDIO_WAKEUP_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_wakeup_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_wakeup_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_wakeup_cntl`] module"]
#[doc(alias = "SDIO_WAKEUP_CNTL")]
pub type SdioWakeupCntl = crate::Reg<sdio_wakeup_cntl::SdioWakeupCntlSpec>;
#[doc = "need_des"]
pub mod sdio_wakeup_cntl;
#[doc = "XTAL_SLP (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_slp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_slp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_slp`] module"]
#[doc(alias = "XTAL_SLP")]
pub type XtalSlp = crate::Reg<xtal_slp::XtalSlpSpec>;
#[doc = "need_des"]
pub mod xtal_slp;
#[doc = "CPU_SW_STALL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sw_stall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_sw_stall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_sw_stall`] module"]
#[doc(alias = "CPU_SW_STALL")]
pub type CpuSwStall = crate::Reg<cpu_sw_stall::CpuSwStallSpec>;
#[doc = "need_des"]
pub mod cpu_sw_stall;
#[doc = "DCM_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dcm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcm_ctrl`] module"]
#[doc(alias = "DCM_CTRL")]
pub type DcmCtrl = crate::Reg<dcm_ctrl::DcmCtrlSpec>;
#[doc = "need_des"]
pub mod dcm_ctrl;
#[doc = "DCM_WAIT_DELAY (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dcm_wait_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcm_wait_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcm_wait_delay`] module"]
#[doc(alias = "DCM_WAIT_DELAY")]
pub type DcmWaitDelay = crate::Reg<dcm_wait_delay::DcmWaitDelaySpec>;
#[doc = "need_des"]
pub mod dcm_wait_delay;
#[doc = "VDDBAT_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vddbat_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddbat_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vddbat_cfg`] module"]
#[doc(alias = "VDDBAT_CFG")]
pub type VddbatCfg = crate::Reg<vddbat_cfg::VddbatCfgSpec>;
#[doc = "need_des"]
pub mod vddbat_cfg;
#[doc = "TOUCH_PWR_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pwr_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pwr_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pwr_cntl`] module"]
#[doc(alias = "TOUCH_PWR_CNTL")]
pub type TouchPwrCntl = crate::Reg<touch_pwr_cntl::TouchPwrCntlSpec>;
#[doc = "need_des"]
pub mod touch_pwr_cntl;
#[doc = "RDN_ECO (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco`] module"]
#[doc(alias = "RDN_ECO")]
pub type RdnEco = crate::Reg<rdn_eco::RdnEcoSpec>;
#[doc = "need_des"]
pub mod rdn_eco;
#[doc = "POWER_PD_HP_CPU_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hp_cpu_cntl`] module"]
#[doc(alias = "POWER_PD_HP_CPU_CNTL")]
pub type PowerPdHpCpuCntl = crate::Reg<power_pd_hp_cpu_cntl::PowerPdHpCpuCntlSpec>;
#[doc = "need_des"]
pub mod power_pd_hp_cpu_cntl;
#[doc = "POWER_PD_HP_CPU_MASK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hp_cpu_mask`] module"]
#[doc(alias = "POWER_PD_HP_CPU_MASK")]
pub type PowerPdHpCpuMask = crate::Reg<power_pd_hp_cpu_mask::PowerPdHpCpuMaskSpec>;
#[doc = "need_des"]
pub mod power_pd_hp_cpu_mask;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "need_des"]
pub mod date;
