#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ver_date: VerDate,
    clk_en: ClkEn,
    _reserved2: [u8; 0x08],
    cpu_intr_from_cpu: [CpuIntrFromCpu; 4],
    cache_clk_config: CacheClkConfig,
    cache_reset_config: CacheResetConfig,
    _reserved5: [u8; 0x04],
    dma_addr_ctrl: DmaAddrCtrl,
    _reserved6: [u8; 0x04],
    tcm_ram_wrr_config: TcmRamWrrConfig,
    tcm_sw_parity_bwe_mask: TcmSwParityBweMask,
    tcm_ram_pwr_ctrl0: TcmRamPwrCtrl0,
    l2_rom_pwr_ctrl0: L2RomPwrCtrl0,
    _reserved10: [u8; 0x0c],
    probea_ctrl: ProbeaCtrl,
    probeb_ctrl: ProbebCtrl,
    _reserved12: [u8; 0x04],
    probe_out: ProbeOut,
    l2_mem_ram_pwr_ctrl0: L2MemRamPwrCtrl0,
    cpu_corestalled_st: CpuCorestalledSt,
    _reserved15: [u8; 0x08],
    crypto_ctrl: CryptoCtrl,
    gpio_o_hold_ctrl0: GpioOHoldCtrl0,
    gpio_o_hold_ctrl1: GpioOHoldCtrl1,
    rdn_eco_cs: RdnEcoCs,
    cache_apb_postw_en: CacheApbPostwEn,
    l2_mem_subsize: L2MemSubsize,
    _reserved21: [u8; 0x14],
    l2_mem_int_raw: L2MemIntRaw,
    l2_mem_int_st: L2MemIntSt,
    l2_mem_int_ena: L2MemIntEna,
    l2_mem_int_clr: L2MemIntClr,
    l2_mem_l2_ram_ecc: L2MemL2RamEcc,
    l2_mem_int_record0: L2MemIntRecord0,
    l2_mem_int_record1: L2MemIntRecord1,
    _reserved28: [u8; 0x0c],
    l2_mem_l2_cache_ecc: L2MemL2CacheEcc,
    l1cache_bus0_id: L1cacheBus0Id,
    l1cache_bus1_id: L1cacheBus1Id,
    _reserved31: [u8; 0x08],
    l2_mem_rdn_eco_cs: L2MemRdnEcoCs,
    l2_mem_rdn_eco_low: L2MemRdnEcoLow,
    l2_mem_rdn_eco_high: L2MemRdnEcoHigh,
    tcm_rdn_eco_cs: TcmRdnEcoCs,
    tcm_rdn_eco_low: TcmRdnEcoLow,
    tcm_rdn_eco_high: TcmRdnEcoHigh,
    gpio_ded_hold_ctrl: GpioDedHoldCtrl,
    l2_mem_sw_ecc_bwe_mask: L2MemSwEccBweMask,
    usb20otg_mem_ctrl: Usb20otgMemCtrl,
    tcm_int_raw: TcmIntRaw,
    tcm_int_st: TcmIntSt,
    tcm_int_ena: TcmIntEna,
    tcm_int_clr: TcmIntClr,
    tcm_parity_int_record: TcmParityIntRecord,
    l1_cache_pwr_ctrl: L1CachePwrCtrl,
    l2_cache_pwr_ctrl: L2CachePwrCtrl,
    cpu_waiti_conf: CpuWaitiConf,
    core_debug_runstall_conf: CoreDebugRunstallConf,
    core_ahb_timeout: CoreAhbTimeout,
    core_ibus_timeout: CoreIbusTimeout,
    core_dbus_timeout: CoreDbusTimeout,
    _reserved52: [u8; 0x0c],
    icm_cpu_h2x_cfg: IcmCpuH2xCfg,
    peri1_apb_postw_en: Peri1ApbPostwEn,
    bitscrambler_peri_sel: BitscramblerPeriSel,
    apb_sync_postw_en: ApbSyncPostwEn,
    gdma_ctrl: GdmaCtrl,
    gmac_ctrl0: GmacCtrl0,
    gmac_ctrl1: GmacCtrl1,
    gmac_ctrl2: GmacCtrl2,
    vpu_ctrl: VpuCtrl,
    usbotg20_ctrl: Usbotg20Ctrl,
    tcm_err_resp_ctrl: TcmErrRespCtrl,
    l2_mem_refresh: L2MemRefresh,
    tcm_init: TcmInit,
    tcm_parity_check_ctrl: TcmParityCheckCtrl,
    design_for_verification0: DesignForVerification0,
    design_for_verification1: DesignForVerification1,
    _reserved68: [u8; 0x08],
    psram_flash_addr_interchange: PsramFlashAddrInterchange,
    _reserved69: [u8; 0x04],
    ahb2axi_bresp_err_int_raw: Ahb2axiBrespErrIntRaw,
    ahb2axi_bresp_err_int_st: Ahb2axiBrespErrIntSt,
    ahb2axi_bresp_err_int_ena: Ahb2axiBrespErrIntEna,
    ahb2axi_bresp_err_int_clr: Ahb2axiBrespErrIntClr,
    l2_mem_err_resp_ctrl: L2MemErrRespCtrl,
    l2_mem_ahb_buffer_ctrl: L2MemAhbBufferCtrl,
    core_dmactive_lpcore: CoreDmactiveLpcore,
    core_err_resp_dis: CoreErrRespDis,
    core_timeout_int_raw: CoreTimeoutIntRaw,
    core_timeout_int_st: CoreTimeoutIntSt,
    core_timeout_int_ena: CoreTimeoutIntEna,
    core_timeout_int_clr: CoreTimeoutIntClr,
    _reserved81: [u8; 0x08],
    gpio_o_hys_ctrl0: GpioOHysCtrl0,
    gpio_o_hys_ctrl1: GpioOHysCtrl1,
    _reserved83: [u8; 0x08],
    rsa_pd_ctrl: RsaPdCtrl,
    ecc_pd_ctrl: EccPdCtrl,
    rng_cfg: RngCfg,
    uart_pd_ctrl: UartPdCtrl,
    peri_mem_clk_force_on: PeriMemClkForceOn,
    _reserved88: [u8; 0x04],
    usb_otghs_phy_st: UsbOtghsPhySt,
    cpu_wakeup_event: CpuWakeupEvent,
    hp2lp_intr_group0_en: Hp2lpIntrGroup0En,
    hp2lp_intr_group1_en: Hp2lpIntrGroup1En,
    hp2lp_intr_group2_en: Hp2lpIntrGroup2En,
    hp2lp_intr_group3_en: Hp2lpIntrGroup3En,
    hp2lp_intr_group0_st: Hp2lpIntrGroup0St,
    hp2lp_intr_group1_st: Hp2lpIntrGroup1St,
    hp2lp_intr_group2_st: Hp2lpIntrGroup2St,
    hp2lp_intr_group3_st: Hp2lpIntrGroup3St,
    hp2lp_wakeup_group0_en: Hp2lpWakeupGroup0En,
    hp2lp_wakeup_group1_en: Hp2lpWakeupGroup1En,
    hp2lp_wakeup_group2_en: Hp2lpWakeupGroup2En,
    hp2lp_wakeup_group3_en: Hp2lpWakeupGroup3En,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VerDate {
        &self.ver_date
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &ClkEn {
        &self.clk_en
    }
    #[doc = "0x10..0x20 - NA"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu(&self, n: usize) -> &CpuIntrFromCpu {
        &self.cpu_intr_from_cpu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - NA"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_iter(&self) -> impl Iterator<Item = &CpuIntrFromCpu> {
        self.cpu_intr_from_cpu.iter()
    }
    #[doc = "0x20 - NA"]
    #[inline(always)]
    pub const fn cache_clk_config(&self) -> &CacheClkConfig {
        &self.cache_clk_config
    }
    #[doc = "0x24 - NA"]
    #[inline(always)]
    pub const fn cache_reset_config(&self) -> &CacheResetConfig {
        &self.cache_reset_config
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn dma_addr_ctrl(&self) -> &DmaAddrCtrl {
        &self.dma_addr_ctrl
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn tcm_ram_wrr_config(&self) -> &TcmRamWrrConfig {
        &self.tcm_ram_wrr_config
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn tcm_sw_parity_bwe_mask(&self) -> &TcmSwParityBweMask {
        &self.tcm_sw_parity_bwe_mask
    }
    #[doc = "0x3c - NA"]
    #[inline(always)]
    pub const fn tcm_ram_pwr_ctrl0(&self) -> &TcmRamPwrCtrl0 {
        &self.tcm_ram_pwr_ctrl0
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn l2_rom_pwr_ctrl0(&self) -> &L2RomPwrCtrl0 {
        &self.l2_rom_pwr_ctrl0
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn probea_ctrl(&self) -> &ProbeaCtrl {
        &self.probea_ctrl
    }
    #[doc = "0x54 - NA"]
    #[inline(always)]
    pub const fn probeb_ctrl(&self) -> &ProbebCtrl {
        &self.probeb_ctrl
    }
    #[doc = "0x5c - NA"]
    #[inline(always)]
    pub const fn probe_out(&self) -> &ProbeOut {
        &self.probe_out
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn l2_mem_ram_pwr_ctrl0(&self) -> &L2MemRamPwrCtrl0 {
        &self.l2_mem_ram_pwr_ctrl0
    }
    #[doc = "0x64 - NA"]
    #[inline(always)]
    pub const fn cpu_corestalled_st(&self) -> &CpuCorestalledSt {
        &self.cpu_corestalled_st
    }
    #[doc = "0x70 - NA"]
    #[inline(always)]
    pub const fn crypto_ctrl(&self) -> &CryptoCtrl {
        &self.crypto_ctrl
    }
    #[doc = "0x74 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hold_ctrl0(&self) -> &GpioOHoldCtrl0 {
        &self.gpio_o_hold_ctrl0
    }
    #[doc = "0x78 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hold_ctrl1(&self) -> &GpioOHoldCtrl1 {
        &self.gpio_o_hold_ctrl1
    }
    #[doc = "0x7c - NA"]
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RdnEcoCs {
        &self.rdn_eco_cs
    }
    #[doc = "0x80 - NA"]
    #[inline(always)]
    pub const fn cache_apb_postw_en(&self) -> &CacheApbPostwEn {
        &self.cache_apb_postw_en
    }
    #[doc = "0x84 - NA"]
    #[inline(always)]
    pub const fn l2_mem_subsize(&self) -> &L2MemSubsize {
        &self.l2_mem_subsize
    }
    #[doc = "0x9c - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_raw(&self) -> &L2MemIntRaw {
        &self.l2_mem_int_raw
    }
    #[doc = "0xa0 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_st(&self) -> &L2MemIntSt {
        &self.l2_mem_int_st
    }
    #[doc = "0xa4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_ena(&self) -> &L2MemIntEna {
        &self.l2_mem_int_ena
    }
    #[doc = "0xa8 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_clr(&self) -> &L2MemIntClr {
        &self.l2_mem_int_clr
    }
    #[doc = "0xac - NA"]
    #[inline(always)]
    pub const fn l2_mem_l2_ram_ecc(&self) -> &L2MemL2RamEcc {
        &self.l2_mem_l2_ram_ecc
    }
    #[doc = "0xb0 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_record0(&self) -> &L2MemIntRecord0 {
        &self.l2_mem_int_record0
    }
    #[doc = "0xb4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_record1(&self) -> &L2MemIntRecord1 {
        &self.l2_mem_int_record1
    }
    #[doc = "0xc4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_l2_cache_ecc(&self) -> &L2MemL2CacheEcc {
        &self.l2_mem_l2_cache_ecc
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn l1cache_bus0_id(&self) -> &L1cacheBus0Id {
        &self.l1cache_bus0_id
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn l1cache_bus1_id(&self) -> &L1cacheBus1Id {
        &self.l1cache_bus1_id
    }
    #[doc = "0xd8 - NA"]
    #[inline(always)]
    pub const fn l2_mem_rdn_eco_cs(&self) -> &L2MemRdnEcoCs {
        &self.l2_mem_rdn_eco_cs
    }
    #[doc = "0xdc - NA"]
    #[inline(always)]
    pub const fn l2_mem_rdn_eco_low(&self) -> &L2MemRdnEcoLow {
        &self.l2_mem_rdn_eco_low
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn l2_mem_rdn_eco_high(&self) -> &L2MemRdnEcoHigh {
        &self.l2_mem_rdn_eco_high
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn tcm_rdn_eco_cs(&self) -> &TcmRdnEcoCs {
        &self.tcm_rdn_eco_cs
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn tcm_rdn_eco_low(&self) -> &TcmRdnEcoLow {
        &self.tcm_rdn_eco_low
    }
    #[doc = "0xec - NA"]
    #[inline(always)]
    pub const fn tcm_rdn_eco_high(&self) -> &TcmRdnEcoHigh {
        &self.tcm_rdn_eco_high
    }
    #[doc = "0xf0 - NA"]
    #[inline(always)]
    pub const fn gpio_ded_hold_ctrl(&self) -> &GpioDedHoldCtrl {
        &self.gpio_ded_hold_ctrl
    }
    #[doc = "0xf4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_sw_ecc_bwe_mask(&self) -> &L2MemSwEccBweMask {
        &self.l2_mem_sw_ecc_bwe_mask
    }
    #[doc = "0xf8 - NA"]
    #[inline(always)]
    pub const fn usb20otg_mem_ctrl(&self) -> &Usb20otgMemCtrl {
        &self.usb20otg_mem_ctrl
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn tcm_int_raw(&self) -> &TcmIntRaw {
        &self.tcm_int_raw
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn tcm_int_st(&self) -> &TcmIntSt {
        &self.tcm_int_st
    }
    #[doc = "0x104 - need_des"]
    #[inline(always)]
    pub const fn tcm_int_ena(&self) -> &TcmIntEna {
        &self.tcm_int_ena
    }
    #[doc = "0x108 - need_des"]
    #[inline(always)]
    pub const fn tcm_int_clr(&self) -> &TcmIntClr {
        &self.tcm_int_clr
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn tcm_parity_int_record(&self) -> &TcmParityIntRecord {
        &self.tcm_parity_int_record
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn l1_cache_pwr_ctrl(&self) -> &L1CachePwrCtrl {
        &self.l1_cache_pwr_ctrl
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn l2_cache_pwr_ctrl(&self) -> &L2CachePwrCtrl {
        &self.l2_cache_pwr_ctrl
    }
    #[doc = "0x118 - CPU_WAITI configuration register"]
    #[inline(always)]
    pub const fn cpu_waiti_conf(&self) -> &CpuWaitiConf {
        &self.cpu_waiti_conf
    }
    #[doc = "0x11c - Core Debug runstall configure register"]
    #[inline(always)]
    pub const fn core_debug_runstall_conf(&self) -> &CoreDebugRunstallConf {
        &self.core_debug_runstall_conf
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn core_ahb_timeout(&self) -> &CoreAhbTimeout {
        &self.core_ahb_timeout
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn core_ibus_timeout(&self) -> &CoreIbusTimeout {
        &self.core_ibus_timeout
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn core_dbus_timeout(&self) -> &CoreDbusTimeout {
        &self.core_dbus_timeout
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn icm_cpu_h2x_cfg(&self) -> &IcmCpuH2xCfg {
        &self.icm_cpu_h2x_cfg
    }
    #[doc = "0x13c - NA"]
    #[inline(always)]
    pub const fn peri1_apb_postw_en(&self) -> &Peri1ApbPostwEn {
        &self.peri1_apb_postw_en
    }
    #[doc = "0x140 - Bitscrambler Peri Sel"]
    #[inline(always)]
    pub const fn bitscrambler_peri_sel(&self) -> &BitscramblerPeriSel {
        &self.bitscrambler_peri_sel
    }
    #[doc = "0x144 - N/A"]
    #[inline(always)]
    pub const fn apb_sync_postw_en(&self) -> &ApbSyncPostwEn {
        &self.apb_sync_postw_en
    }
    #[doc = "0x148 - N/A"]
    #[inline(always)]
    pub const fn gdma_ctrl(&self) -> &GdmaCtrl {
        &self.gdma_ctrl
    }
    #[doc = "0x14c - N/A"]
    #[inline(always)]
    pub const fn gmac_ctrl0(&self) -> &GmacCtrl0 {
        &self.gmac_ctrl0
    }
    #[doc = "0x150 - N/A"]
    #[inline(always)]
    pub const fn gmac_ctrl1(&self) -> &GmacCtrl1 {
        &self.gmac_ctrl1
    }
    #[doc = "0x154 - N/A"]
    #[inline(always)]
    pub const fn gmac_ctrl2(&self) -> &GmacCtrl2 {
        &self.gmac_ctrl2
    }
    #[doc = "0x158 - N/A"]
    #[inline(always)]
    pub const fn vpu_ctrl(&self) -> &VpuCtrl {
        &self.vpu_ctrl
    }
    #[doc = "0x15c - N/A"]
    #[inline(always)]
    pub const fn usbotg20_ctrl(&self) -> &Usbotg20Ctrl {
        &self.usbotg20_ctrl
    }
    #[doc = "0x160 - need_des"]
    #[inline(always)]
    pub const fn tcm_err_resp_ctrl(&self) -> &TcmErrRespCtrl {
        &self.tcm_err_resp_ctrl
    }
    #[doc = "0x164 - NA"]
    #[inline(always)]
    pub const fn l2_mem_refresh(&self) -> &L2MemRefresh {
        &self.l2_mem_refresh
    }
    #[doc = "0x168 - NA"]
    #[inline(always)]
    pub const fn tcm_init(&self) -> &TcmInit {
        &self.tcm_init
    }
    #[doc = "0x16c - need_des"]
    #[inline(always)]
    pub const fn tcm_parity_check_ctrl(&self) -> &TcmParityCheckCtrl {
        &self.tcm_parity_check_ctrl
    }
    #[doc = "0x170 - need_des"]
    #[inline(always)]
    pub const fn design_for_verification0(&self) -> &DesignForVerification0 {
        &self.design_for_verification0
    }
    #[doc = "0x174 - need_des"]
    #[inline(always)]
    pub const fn design_for_verification1(&self) -> &DesignForVerification1 {
        &self.design_for_verification1
    }
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn psram_flash_addr_interchange(&self) -> &PsramFlashAddrInterchange {
        &self.psram_flash_addr_interchange
    }
    #[doc = "0x188 - NA"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_raw(&self) -> &Ahb2axiBrespErrIntRaw {
        &self.ahb2axi_bresp_err_int_raw
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_st(&self) -> &Ahb2axiBrespErrIntSt {
        &self.ahb2axi_bresp_err_int_st
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_ena(&self) -> &Ahb2axiBrespErrIntEna {
        &self.ahb2axi_bresp_err_int_ena
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_clr(&self) -> &Ahb2axiBrespErrIntClr {
        &self.ahb2axi_bresp_err_int_clr
    }
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn l2_mem_err_resp_ctrl(&self) -> &L2MemErrRespCtrl {
        &self.l2_mem_err_resp_ctrl
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn l2_mem_ahb_buffer_ctrl(&self) -> &L2MemAhbBufferCtrl {
        &self.l2_mem_ahb_buffer_ctrl
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn core_dmactive_lpcore(&self) -> &CoreDmactiveLpcore {
        &self.core_dmactive_lpcore
    }
    #[doc = "0x1a4 - need_des"]
    #[inline(always)]
    pub const fn core_err_resp_dis(&self) -> &CoreErrRespDis {
        &self.core_err_resp_dis
    }
    #[doc = "0x1a8 - Hp core bus timeout interrupt raw register"]
    #[inline(always)]
    pub const fn core_timeout_int_raw(&self) -> &CoreTimeoutIntRaw {
        &self.core_timeout_int_raw
    }
    #[doc = "0x1ac - masked interrupt register"]
    #[inline(always)]
    pub const fn core_timeout_int_st(&self) -> &CoreTimeoutIntSt {
        &self.core_timeout_int_st
    }
    #[doc = "0x1b0 - masked interrupt register"]
    #[inline(always)]
    pub const fn core_timeout_int_ena(&self) -> &CoreTimeoutIntEna {
        &self.core_timeout_int_ena
    }
    #[doc = "0x1b4 - interrupt clear register"]
    #[inline(always)]
    pub const fn core_timeout_int_clr(&self) -> &CoreTimeoutIntClr {
        &self.core_timeout_int_clr
    }
    #[doc = "0x1c0 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hys_ctrl0(&self) -> &GpioOHysCtrl0 {
        &self.gpio_o_hys_ctrl0
    }
    #[doc = "0x1c4 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hys_ctrl1(&self) -> &GpioOHysCtrl1 {
        &self.gpio_o_hys_ctrl1
    }
    #[doc = "0x1d0 - rsa pd ctrl register"]
    #[inline(always)]
    pub const fn rsa_pd_ctrl(&self) -> &RsaPdCtrl {
        &self.rsa_pd_ctrl
    }
    #[doc = "0x1d4 - ecc pd ctrl register"]
    #[inline(always)]
    pub const fn ecc_pd_ctrl(&self) -> &EccPdCtrl {
        &self.ecc_pd_ctrl
    }
    #[doc = "0x1d8 - rng cfg register"]
    #[inline(always)]
    pub const fn rng_cfg(&self) -> &RngCfg {
        &self.rng_cfg
    }
    #[doc = "0x1dc - ecc pd ctrl register"]
    #[inline(always)]
    pub const fn uart_pd_ctrl(&self) -> &UartPdCtrl {
        &self.uart_pd_ctrl
    }
    #[doc = "0x1e0 - hp peri mem clk force on regpster"]
    #[inline(always)]
    pub const fn peri_mem_clk_force_on(&self) -> &PeriMemClkForceOn {
        &self.peri_mem_clk_force_on
    }
    #[doc = "0x1e8 - Usb otg2.0 PHY status register"]
    #[inline(always)]
    pub const fn usb_otghs_phy_st(&self) -> &UsbOtghsPhySt {
        &self.usb_otghs_phy_st
    }
    #[doc = "0x1ec - cpu wakeup event ctrl register"]
    #[inline(always)]
    pub const fn cpu_wakeup_event(&self) -> &CpuWakeupEvent {
        &self.cpu_wakeup_event
    }
    #[doc = "0x1f0 - HpP2LP Interrupt Enable Register Group0"]
    #[inline(always)]
    pub const fn hp2lp_intr_group0_en(&self) -> &Hp2lpIntrGroup0En {
        &self.hp2lp_intr_group0_en
    }
    #[doc = "0x1f4 - HpP2LP Interrupt Enable Register Group1"]
    #[inline(always)]
    pub const fn hp2lp_intr_group1_en(&self) -> &Hp2lpIntrGroup1En {
        &self.hp2lp_intr_group1_en
    }
    #[doc = "0x1f8 - HpP2LP Interrupt Enable Register Group2"]
    #[inline(always)]
    pub const fn hp2lp_intr_group2_en(&self) -> &Hp2lpIntrGroup2En {
        &self.hp2lp_intr_group2_en
    }
    #[doc = "0x1fc - HpP2LP Interrupt Enable Register Group3"]
    #[inline(always)]
    pub const fn hp2lp_intr_group3_en(&self) -> &Hp2lpIntrGroup3En {
        &self.hp2lp_intr_group3_en
    }
    #[doc = "0x200 - HpP2LP Interrupt Status Register Group0"]
    #[inline(always)]
    pub const fn hp2lp_intr_group0_st(&self) -> &Hp2lpIntrGroup0St {
        &self.hp2lp_intr_group0_st
    }
    #[doc = "0x204 - HpP2LP Interrupt Enable Register Group1"]
    #[inline(always)]
    pub const fn hp2lp_intr_group1_st(&self) -> &Hp2lpIntrGroup1St {
        &self.hp2lp_intr_group1_st
    }
    #[doc = "0x208 - HpP2LP Interrupt Enable Register Group2"]
    #[inline(always)]
    pub const fn hp2lp_intr_group2_st(&self) -> &Hp2lpIntrGroup2St {
        &self.hp2lp_intr_group2_st
    }
    #[doc = "0x20c - HpP2LP Interrupt Enable Register Group3"]
    #[inline(always)]
    pub const fn hp2lp_intr_group3_st(&self) -> &Hp2lpIntrGroup3St {
        &self.hp2lp_intr_group3_st
    }
    #[doc = "0x210 - HpP2LP Wakeup Enable Register Group0"]
    #[inline(always)]
    pub const fn hp2lp_wakeup_group0_en(&self) -> &Hp2lpWakeupGroup0En {
        &self.hp2lp_wakeup_group0_en
    }
    #[doc = "0x214 - HpP2LP Wakeup Enable Register Group1"]
    #[inline(always)]
    pub const fn hp2lp_wakeup_group1_en(&self) -> &Hp2lpWakeupGroup1En {
        &self.hp2lp_wakeup_group1_en
    }
    #[doc = "0x218 - HpP2LP Wakeup Enable Register Group2"]
    #[inline(always)]
    pub const fn hp2lp_wakeup_group2_en(&self) -> &Hp2lpWakeupGroup2En {
        &self.hp2lp_wakeup_group2_en
    }
    #[doc = "0x21c - HpP2LP Wakeup Enable Register Group3"]
    #[inline(always)]
    pub const fn hp2lp_wakeup_group3_en(&self) -> &Hp2lpWakeupGroup3En {
        &self.hp2lp_wakeup_group3_en
    }
}
#[doc = "VER_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
#[doc(alias = "VER_DATE")]
pub type VerDate = crate::Reg<ver_date::VerDateSpec>;
#[doc = "NA"]
pub mod ver_date;
#[doc = "CLK_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
#[doc(alias = "CLK_EN")]
pub type ClkEn = crate::Reg<clk_en::ClkEnSpec>;
#[doc = "NA"]
pub mod clk_en;
#[doc = "CPU_INTR_FROM_CPU (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu`] module"]
#[doc(alias = "CPU_INTR_FROM_CPU")]
pub type CpuIntrFromCpu = crate::Reg<cpu_intr_from_cpu::CpuIntrFromCpuSpec>;
#[doc = "NA"]
pub mod cpu_intr_from_cpu;
#[doc = "CACHE_CLK_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_clk_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_clk_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_clk_config`] module"]
#[doc(alias = "CACHE_CLK_CONFIG")]
pub type CacheClkConfig = crate::Reg<cache_clk_config::CacheClkConfigSpec>;
#[doc = "NA"]
pub mod cache_clk_config;
#[doc = "CACHE_RESET_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_reset_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_reset_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_reset_config`] module"]
#[doc(alias = "CACHE_RESET_CONFIG")]
pub type CacheResetConfig = crate::Reg<cache_reset_config::CacheResetConfigSpec>;
#[doc = "NA"]
pub mod cache_reset_config;
#[doc = "DMA_ADDR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_addr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_addr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_addr_ctrl`] module"]
#[doc(alias = "DMA_ADDR_CTRL")]
pub type DmaAddrCtrl = crate::Reg<dma_addr_ctrl::DmaAddrCtrlSpec>;
#[doc = "NA"]
pub mod dma_addr_ctrl;
#[doc = "TCM_RAM_WRR_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_ram_wrr_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_ram_wrr_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_ram_wrr_config`] module"]
#[doc(alias = "TCM_RAM_WRR_CONFIG")]
pub type TcmRamWrrConfig = crate::Reg<tcm_ram_wrr_config::TcmRamWrrConfigSpec>;
#[doc = "NA"]
pub mod tcm_ram_wrr_config;
#[doc = "TCM_SW_PARITY_BWE_MASK (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_sw_parity_bwe_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_sw_parity_bwe_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_sw_parity_bwe_mask`] module"]
#[doc(alias = "TCM_SW_PARITY_BWE_MASK")]
pub type TcmSwParityBweMask = crate::Reg<tcm_sw_parity_bwe_mask::TcmSwParityBweMaskSpec>;
#[doc = "NA"]
pub mod tcm_sw_parity_bwe_mask;
#[doc = "TCM_RAM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_ram_pwr_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_ram_pwr_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_ram_pwr_ctrl0`] module"]
#[doc(alias = "TCM_RAM_PWR_CTRL0")]
pub type TcmRamPwrCtrl0 = crate::Reg<tcm_ram_pwr_ctrl0::TcmRamPwrCtrl0Spec>;
#[doc = "NA"]
pub mod tcm_ram_pwr_ctrl0;
#[doc = "L2_ROM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_rom_pwr_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rom_pwr_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_rom_pwr_ctrl0`] module"]
#[doc(alias = "L2_ROM_PWR_CTRL0")]
pub type L2RomPwrCtrl0 = crate::Reg<l2_rom_pwr_ctrl0::L2RomPwrCtrl0Spec>;
#[doc = "NA"]
pub mod l2_rom_pwr_ctrl0;
#[doc = "PROBEA_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`probea_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`probea_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probea_ctrl`] module"]
#[doc(alias = "PROBEA_CTRL")]
pub type ProbeaCtrl = crate::Reg<probea_ctrl::ProbeaCtrlSpec>;
#[doc = "NA"]
pub mod probea_ctrl;
#[doc = "PROBEB_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`probeb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`probeb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probeb_ctrl`] module"]
#[doc(alias = "PROBEB_CTRL")]
pub type ProbebCtrl = crate::Reg<probeb_ctrl::ProbebCtrlSpec>;
#[doc = "NA"]
pub mod probeb_ctrl;
#[doc = "PROBE_OUT (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`probe_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_out`] module"]
#[doc(alias = "PROBE_OUT")]
pub type ProbeOut = crate::Reg<probe_out::ProbeOutSpec>;
#[doc = "NA"]
pub mod probe_out;
#[doc = "L2_MEM_RAM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_ram_pwr_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_ram_pwr_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_ram_pwr_ctrl0`] module"]
#[doc(alias = "L2_MEM_RAM_PWR_CTRL0")]
pub type L2MemRamPwrCtrl0 = crate::Reg<l2_mem_ram_pwr_ctrl0::L2MemRamPwrCtrl0Spec>;
#[doc = "NA"]
pub mod l2_mem_ram_pwr_ctrl0;
#[doc = "CPU_CORESTALLED_ST (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_corestalled_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_corestalled_st`] module"]
#[doc(alias = "CPU_CORESTALLED_ST")]
pub type CpuCorestalledSt = crate::Reg<cpu_corestalled_st::CpuCorestalledStSpec>;
#[doc = "NA"]
pub mod cpu_corestalled_st;
#[doc = "CRYPTO_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_ctrl`] module"]
#[doc(alias = "CRYPTO_CTRL")]
pub type CryptoCtrl = crate::Reg<crypto_ctrl::CryptoCtrlSpec>;
#[doc = "NA"]
pub mod crypto_ctrl;
#[doc = "GPIO_O_HOLD_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_o_hold_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_o_hold_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hold_ctrl0`] module"]
#[doc(alias = "GPIO_O_HOLD_CTRL0")]
pub type GpioOHoldCtrl0 = crate::Reg<gpio_o_hold_ctrl0::GpioOHoldCtrl0Spec>;
#[doc = "NA"]
pub mod gpio_o_hold_ctrl0;
#[doc = "GPIO_O_HOLD_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_o_hold_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_o_hold_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hold_ctrl1`] module"]
#[doc(alias = "GPIO_O_HOLD_CTRL1")]
pub type GpioOHoldCtrl1 = crate::Reg<gpio_o_hold_ctrl1::GpioOHoldCtrl1Spec>;
#[doc = "NA"]
pub mod gpio_o_hold_ctrl1;
#[doc = "RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_cs`] module"]
#[doc(alias = "RDN_ECO_CS")]
pub type RdnEcoCs = crate::Reg<rdn_eco_cs::RdnEcoCsSpec>;
#[doc = "NA"]
pub mod rdn_eco_cs;
#[doc = "CACHE_APB_POSTW_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_apb_postw_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_apb_postw_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_apb_postw_en`] module"]
#[doc(alias = "CACHE_APB_POSTW_EN")]
pub type CacheApbPostwEn = crate::Reg<cache_apb_postw_en::CacheApbPostwEnSpec>;
#[doc = "NA"]
pub mod cache_apb_postw_en;
#[doc = "L2_MEM_SUBSIZE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_subsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_subsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_subsize`] module"]
#[doc(alias = "L2_MEM_SUBSIZE")]
pub type L2MemSubsize = crate::Reg<l2_mem_subsize::L2MemSubsizeSpec>;
#[doc = "NA"]
pub mod l2_mem_subsize;
#[doc = "L2_MEM_INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_raw`] module"]
#[doc(alias = "L2_MEM_INT_RAW")]
pub type L2MemIntRaw = crate::Reg<l2_mem_int_raw::L2MemIntRawSpec>;
#[doc = "NA"]
pub mod l2_mem_int_raw;
#[doc = "L2_MEM_INT_ST (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_st`] module"]
#[doc(alias = "L2_MEM_INT_ST")]
pub type L2MemIntSt = crate::Reg<l2_mem_int_st::L2MemIntStSpec>;
#[doc = "NA"]
pub mod l2_mem_int_st;
#[doc = "L2_MEM_INT_ENA (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_ena`] module"]
#[doc(alias = "L2_MEM_INT_ENA")]
pub type L2MemIntEna = crate::Reg<l2_mem_int_ena::L2MemIntEnaSpec>;
#[doc = "NA"]
pub mod l2_mem_int_ena;
#[doc = "L2_MEM_INT_CLR (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_clr`] module"]
#[doc(alias = "L2_MEM_INT_CLR")]
pub type L2MemIntClr = crate::Reg<l2_mem_int_clr::L2MemIntClrSpec>;
#[doc = "NA"]
pub mod l2_mem_int_clr;
#[doc = "L2_MEM_L2_RAM_ECC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_l2_ram_ecc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_l2_ram_ecc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_l2_ram_ecc`] module"]
#[doc(alias = "L2_MEM_L2_RAM_ECC")]
pub type L2MemL2RamEcc = crate::Reg<l2_mem_l2_ram_ecc::L2MemL2RamEccSpec>;
#[doc = "NA"]
pub mod l2_mem_l2_ram_ecc;
#[doc = "L2_MEM_INT_RECORD0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_record0`] module"]
#[doc(alias = "L2_MEM_INT_RECORD0")]
pub type L2MemIntRecord0 = crate::Reg<l2_mem_int_record0::L2MemIntRecord0Spec>;
#[doc = "NA"]
pub mod l2_mem_int_record0;
#[doc = "L2_MEM_INT_RECORD1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_record1`] module"]
#[doc(alias = "L2_MEM_INT_RECORD1")]
pub type L2MemIntRecord1 = crate::Reg<l2_mem_int_record1::L2MemIntRecord1Spec>;
#[doc = "NA"]
pub mod l2_mem_int_record1;
#[doc = "L2_MEM_L2_CACHE_ECC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_l2_cache_ecc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_l2_cache_ecc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_l2_cache_ecc`] module"]
#[doc(alias = "L2_MEM_L2_CACHE_ECC")]
pub type L2MemL2CacheEcc = crate::Reg<l2_mem_l2_cache_ecc::L2MemL2CacheEccSpec>;
#[doc = "NA"]
pub mod l2_mem_l2_cache_ecc;
#[doc = "L1CACHE_BUS0_ID (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l1cache_bus0_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cache_bus0_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cache_bus0_id`] module"]
#[doc(alias = "L1CACHE_BUS0_ID")]
pub type L1cacheBus0Id = crate::Reg<l1cache_bus0_id::L1cacheBus0IdSpec>;
#[doc = "NA"]
pub mod l1cache_bus0_id;
#[doc = "L1CACHE_BUS1_ID (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l1cache_bus1_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cache_bus1_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cache_bus1_id`] module"]
#[doc(alias = "L1CACHE_BUS1_ID")]
pub type L1cacheBus1Id = crate::Reg<l1cache_bus1_id::L1cacheBus1IdSpec>;
#[doc = "NA"]
pub mod l1cache_bus1_id;
#[doc = "L2_MEM_RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_rdn_eco_cs`] module"]
#[doc(alias = "L2_MEM_RDN_ECO_CS")]
pub type L2MemRdnEcoCs = crate::Reg<l2_mem_rdn_eco_cs::L2MemRdnEcoCsSpec>;
#[doc = "NA"]
pub mod l2_mem_rdn_eco_cs;
#[doc = "L2_MEM_RDN_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_rdn_eco_low`] module"]
#[doc(alias = "L2_MEM_RDN_ECO_LOW")]
pub type L2MemRdnEcoLow = crate::Reg<l2_mem_rdn_eco_low::L2MemRdnEcoLowSpec>;
#[doc = "NA"]
pub mod l2_mem_rdn_eco_low;
#[doc = "L2_MEM_RDN_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_rdn_eco_high`] module"]
#[doc(alias = "L2_MEM_RDN_ECO_HIGH")]
pub type L2MemRdnEcoHigh = crate::Reg<l2_mem_rdn_eco_high::L2MemRdnEcoHighSpec>;
#[doc = "NA"]
pub mod l2_mem_rdn_eco_high;
#[doc = "TCM_RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_rdn_eco_cs`] module"]
#[doc(alias = "TCM_RDN_ECO_CS")]
pub type TcmRdnEcoCs = crate::Reg<tcm_rdn_eco_cs::TcmRdnEcoCsSpec>;
#[doc = "NA"]
pub mod tcm_rdn_eco_cs;
#[doc = "TCM_RDN_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_rdn_eco_low`] module"]
#[doc(alias = "TCM_RDN_ECO_LOW")]
pub type TcmRdnEcoLow = crate::Reg<tcm_rdn_eco_low::TcmRdnEcoLowSpec>;
#[doc = "NA"]
pub mod tcm_rdn_eco_low;
#[doc = "TCM_RDN_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_rdn_eco_high`] module"]
#[doc(alias = "TCM_RDN_ECO_HIGH")]
pub type TcmRdnEcoHigh = crate::Reg<tcm_rdn_eco_high::TcmRdnEcoHighSpec>;
#[doc = "NA"]
pub mod tcm_rdn_eco_high;
#[doc = "GPIO_DED_HOLD_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ded_hold_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ded_hold_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ded_hold_ctrl`] module"]
#[doc(alias = "GPIO_DED_HOLD_CTRL")]
pub type GpioDedHoldCtrl = crate::Reg<gpio_ded_hold_ctrl::GpioDedHoldCtrlSpec>;
#[doc = "NA"]
pub mod gpio_ded_hold_ctrl;
#[doc = "L2_MEM_SW_ECC_BWE_MASK (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_sw_ecc_bwe_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_sw_ecc_bwe_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_sw_ecc_bwe_mask`] module"]
#[doc(alias = "L2_MEM_SW_ECC_BWE_MASK")]
pub type L2MemSwEccBweMask = crate::Reg<l2_mem_sw_ecc_bwe_mask::L2MemSwEccBweMaskSpec>;
#[doc = "NA"]
pub mod l2_mem_sw_ecc_bwe_mask;
#[doc = "USB20OTG_MEM_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`usb20otg_mem_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb20otg_mem_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20otg_mem_ctrl`] module"]
#[doc(alias = "USB20OTG_MEM_CTRL")]
pub type Usb20otgMemCtrl = crate::Reg<usb20otg_mem_ctrl::Usb20otgMemCtrlSpec>;
#[doc = "NA"]
pub mod usb20otg_mem_ctrl;
#[doc = "TCM_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_raw`] module"]
#[doc(alias = "TCM_INT_RAW")]
pub type TcmIntRaw = crate::Reg<tcm_int_raw::TcmIntRawSpec>;
#[doc = "need_des"]
pub mod tcm_int_raw;
#[doc = "TCM_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_st`] module"]
#[doc(alias = "TCM_INT_ST")]
pub type TcmIntSt = crate::Reg<tcm_int_st::TcmIntStSpec>;
#[doc = "need_des"]
pub mod tcm_int_st;
#[doc = "TCM_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_ena`] module"]
#[doc(alias = "TCM_INT_ENA")]
pub type TcmIntEna = crate::Reg<tcm_int_ena::TcmIntEnaSpec>;
#[doc = "need_des"]
pub mod tcm_int_ena;
#[doc = "TCM_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_clr`] module"]
#[doc(alias = "TCM_INT_CLR")]
pub type TcmIntClr = crate::Reg<tcm_int_clr::TcmIntClrSpec>;
#[doc = "need_des"]
pub mod tcm_int_clr;
#[doc = "TCM_PARITY_INT_RECORD (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_parity_int_record::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_parity_int_record`] module"]
#[doc(alias = "TCM_PARITY_INT_RECORD")]
pub type TcmParityIntRecord = crate::Reg<tcm_parity_int_record::TcmParityIntRecordSpec>;
#[doc = "need_des"]
pub mod tcm_parity_int_record;
#[doc = "L1_CACHE_PWR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_pwr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_pwr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_cache_pwr_ctrl`] module"]
#[doc(alias = "L1_CACHE_PWR_CTRL")]
pub type L1CachePwrCtrl = crate::Reg<l1_cache_pwr_ctrl::L1CachePwrCtrlSpec>;
#[doc = "NA"]
pub mod l1_cache_pwr_ctrl;
#[doc = "L2_CACHE_PWR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_pwr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_pwr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_cache_pwr_ctrl`] module"]
#[doc(alias = "L2_CACHE_PWR_CTRL")]
pub type L2CachePwrCtrl = crate::Reg<l2_cache_pwr_ctrl::L2CachePwrCtrlSpec>;
#[doc = "NA"]
pub mod l2_cache_pwr_ctrl;
#[doc = "CPU_WAITI_CONF (rw) register accessor: CPU_WAITI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_waiti_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_waiti_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_waiti_conf`] module"]
#[doc(alias = "CPU_WAITI_CONF")]
pub type CpuWaitiConf = crate::Reg<cpu_waiti_conf::CpuWaitiConfSpec>;
#[doc = "CPU_WAITI configuration register"]
pub mod cpu_waiti_conf;
#[doc = "CORE_DEBUG_RUNSTALL_CONF (rw) register accessor: Core Debug runstall configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_debug_runstall_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_debug_runstall_conf`] module"]
#[doc(alias = "CORE_DEBUG_RUNSTALL_CONF")]
pub type CoreDebugRunstallConf = crate::Reg<core_debug_runstall_conf::CoreDebugRunstallConfSpec>;
#[doc = "Core Debug runstall configure register"]
pub mod core_debug_runstall_conf;
#[doc = "CORE_AHB_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_ahb_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_ahb_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_ahb_timeout`] module"]
#[doc(alias = "CORE_AHB_TIMEOUT")]
pub type CoreAhbTimeout = crate::Reg<core_ahb_timeout::CoreAhbTimeoutSpec>;
#[doc = "need_des"]
pub mod core_ahb_timeout;
#[doc = "CORE_IBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_ibus_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_ibus_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_ibus_timeout`] module"]
#[doc(alias = "CORE_IBUS_TIMEOUT")]
pub type CoreIbusTimeout = crate::Reg<core_ibus_timeout::CoreIbusTimeoutSpec>;
#[doc = "need_des"]
pub mod core_ibus_timeout;
#[doc = "CORE_DBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_dbus_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_dbus_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_dbus_timeout`] module"]
#[doc(alias = "CORE_DBUS_TIMEOUT")]
pub type CoreDbusTimeout = crate::Reg<core_dbus_timeout::CoreDbusTimeoutSpec>;
#[doc = "need_des"]
pub mod core_dbus_timeout;
#[doc = "ICM_CPU_H2X_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_cpu_h2x_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_cpu_h2x_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_cpu_h2x_cfg`] module"]
#[doc(alias = "ICM_CPU_H2X_CFG")]
pub type IcmCpuH2xCfg = crate::Reg<icm_cpu_h2x_cfg::IcmCpuH2xCfgSpec>;
#[doc = "need_des"]
pub mod icm_cpu_h2x_cfg;
#[doc = "PERI1_APB_POSTW_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri1_apb_postw_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri1_apb_postw_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri1_apb_postw_en`] module"]
#[doc(alias = "PERI1_APB_POSTW_EN")]
pub type Peri1ApbPostwEn = crate::Reg<peri1_apb_postw_en::Peri1ApbPostwEnSpec>;
#[doc = "NA"]
pub mod peri1_apb_postw_en;
#[doc = "BITSCRAMBLER_PERI_SEL (rw) register accessor: Bitscrambler Peri Sel\n\nYou can [`read`](crate::Reg::read) this register and get [`bitscrambler_peri_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitscrambler_peri_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bitscrambler_peri_sel`] module"]
#[doc(alias = "BITSCRAMBLER_PERI_SEL")]
pub type BitscramblerPeriSel = crate::Reg<bitscrambler_peri_sel::BitscramblerPeriSelSpec>;
#[doc = "Bitscrambler Peri Sel"]
pub mod bitscrambler_peri_sel;
#[doc = "APB_SYNC_POSTW_EN (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_sync_postw_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_sync_postw_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_sync_postw_en`] module"]
#[doc(alias = "APB_SYNC_POSTW_EN")]
pub type ApbSyncPostwEn = crate::Reg<apb_sync_postw_en::ApbSyncPostwEnSpec>;
#[doc = "N/A"]
pub mod apb_sync_postw_en;
#[doc = "GDMA_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctrl`] module"]
#[doc(alias = "GDMA_CTRL")]
pub type GdmaCtrl = crate::Reg<gdma_ctrl::GdmaCtrlSpec>;
#[doc = "N/A"]
pub mod gdma_ctrl;
#[doc = "GMAC_CTRL0 (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_ctrl0`] module"]
#[doc(alias = "GMAC_CTRL0")]
pub type GmacCtrl0 = crate::Reg<gmac_ctrl0::GmacCtrl0Spec>;
#[doc = "N/A"]
pub mod gmac_ctrl0;
#[doc = "GMAC_CTRL1 (r) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_ctrl1`] module"]
#[doc(alias = "GMAC_CTRL1")]
pub type GmacCtrl1 = crate::Reg<gmac_ctrl1::GmacCtrl1Spec>;
#[doc = "N/A"]
pub mod gmac_ctrl1;
#[doc = "GMAC_CTRL2 (r) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_ctrl2`] module"]
#[doc(alias = "GMAC_CTRL2")]
pub type GmacCtrl2 = crate::Reg<gmac_ctrl2::GmacCtrl2Spec>;
#[doc = "N/A"]
pub mod gmac_ctrl2;
#[doc = "VPU_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`vpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpu_ctrl`] module"]
#[doc(alias = "VPU_CTRL")]
pub type VpuCtrl = crate::Reg<vpu_ctrl::VpuCtrlSpec>;
#[doc = "N/A"]
pub mod vpu_ctrl;
#[doc = "USBOTG20_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`usbotg20_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbotg20_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbotg20_ctrl`] module"]
#[doc(alias = "USBOTG20_CTRL")]
pub type Usbotg20Ctrl = crate::Reg<usbotg20_ctrl::Usbotg20CtrlSpec>;
#[doc = "N/A"]
pub mod usbotg20_ctrl;
#[doc = "TCM_ERR_RESP_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_err_resp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_err_resp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_err_resp_ctrl`] module"]
#[doc(alias = "TCM_ERR_RESP_CTRL")]
pub type TcmErrRespCtrl = crate::Reg<tcm_err_resp_ctrl::TcmErrRespCtrlSpec>;
#[doc = "need_des"]
pub mod tcm_err_resp_ctrl;
#[doc = "L2_MEM_REFRESH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_refresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_refresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_refresh`] module"]
#[doc(alias = "L2_MEM_REFRESH")]
pub type L2MemRefresh = crate::Reg<l2_mem_refresh::L2MemRefreshSpec>;
#[doc = "NA"]
pub mod l2_mem_refresh;
#[doc = "TCM_INIT (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_init`] module"]
#[doc(alias = "TCM_INIT")]
pub type TcmInit = crate::Reg<tcm_init::TcmInitSpec>;
#[doc = "NA"]
pub mod tcm_init;
#[doc = "TCM_PARITY_CHECK_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_parity_check_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_parity_check_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_parity_check_ctrl`] module"]
#[doc(alias = "TCM_PARITY_CHECK_CTRL")]
pub type TcmParityCheckCtrl = crate::Reg<tcm_parity_check_ctrl::TcmParityCheckCtrlSpec>;
#[doc = "need_des"]
pub mod tcm_parity_check_ctrl;
#[doc = "DESIGN_FOR_VERIFICATION0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`design_for_verification0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`design_for_verification0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@design_for_verification0`] module"]
#[doc(alias = "DESIGN_FOR_VERIFICATION0")]
pub type DesignForVerification0 = crate::Reg<design_for_verification0::DesignForVerification0Spec>;
#[doc = "need_des"]
pub mod design_for_verification0;
#[doc = "DESIGN_FOR_VERIFICATION1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`design_for_verification1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`design_for_verification1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@design_for_verification1`] module"]
#[doc(alias = "DESIGN_FOR_VERIFICATION1")]
pub type DesignForVerification1 = crate::Reg<design_for_verification1::DesignForVerification1Spec>;
#[doc = "need_des"]
pub mod design_for_verification1;
#[doc = "PSRAM_FLASH_ADDR_INTERCHANGE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_flash_addr_interchange::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_flash_addr_interchange::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_flash_addr_interchange`] module"]
#[doc(alias = "PSRAM_FLASH_ADDR_INTERCHANGE")]
pub type PsramFlashAddrInterchange =
    crate::Reg<psram_flash_addr_interchange::PsramFlashAddrInterchangeSpec>;
#[doc = "need_des"]
pub mod psram_flash_addr_interchange;
#[doc = "AHB2AXI_BRESP_ERR_INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2axi_bresp_err_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_raw`] module"]
#[doc(alias = "AHB2AXI_BRESP_ERR_INT_RAW")]
pub type Ahb2axiBrespErrIntRaw = crate::Reg<ahb2axi_bresp_err_int_raw::Ahb2axiBrespErrIntRawSpec>;
#[doc = "NA"]
pub mod ahb2axi_bresp_err_int_raw;
#[doc = "AHB2AXI_BRESP_ERR_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2axi_bresp_err_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_st`] module"]
#[doc(alias = "AHB2AXI_BRESP_ERR_INT_ST")]
pub type Ahb2axiBrespErrIntSt = crate::Reg<ahb2axi_bresp_err_int_st::Ahb2axiBrespErrIntStSpec>;
#[doc = "need_des"]
pub mod ahb2axi_bresp_err_int_st;
#[doc = "AHB2AXI_BRESP_ERR_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2axi_bresp_err_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_ena`] module"]
#[doc(alias = "AHB2AXI_BRESP_ERR_INT_ENA")]
pub type Ahb2axiBrespErrIntEna = crate::Reg<ahb2axi_bresp_err_int_ena::Ahb2axiBrespErrIntEnaSpec>;
#[doc = "need_des"]
pub mod ahb2axi_bresp_err_int_ena;
#[doc = "AHB2AXI_BRESP_ERR_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_clr`] module"]
#[doc(alias = "AHB2AXI_BRESP_ERR_INT_CLR")]
pub type Ahb2axiBrespErrIntClr = crate::Reg<ahb2axi_bresp_err_int_clr::Ahb2axiBrespErrIntClrSpec>;
#[doc = "need_des"]
pub mod ahb2axi_bresp_err_int_clr;
#[doc = "L2_MEM_ERR_RESP_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_err_resp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_err_resp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_err_resp_ctrl`] module"]
#[doc(alias = "L2_MEM_ERR_RESP_CTRL")]
pub type L2MemErrRespCtrl = crate::Reg<l2_mem_err_resp_ctrl::L2MemErrRespCtrlSpec>;
#[doc = "need_des"]
pub mod l2_mem_err_resp_ctrl;
#[doc = "L2_MEM_AHB_BUFFER_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_ahb_buffer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_ahb_buffer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_ahb_buffer_ctrl`] module"]
#[doc(alias = "L2_MEM_AHB_BUFFER_CTRL")]
pub type L2MemAhbBufferCtrl = crate::Reg<l2_mem_ahb_buffer_ctrl::L2MemAhbBufferCtrlSpec>;
#[doc = "need_des"]
pub mod l2_mem_ahb_buffer_ctrl;
#[doc = "CORE_DMACTIVE_LPCORE (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_dmactive_lpcore::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_dmactive_lpcore`] module"]
#[doc(alias = "CORE_DMACTIVE_LPCORE")]
pub type CoreDmactiveLpcore = crate::Reg<core_dmactive_lpcore::CoreDmactiveLpcoreSpec>;
#[doc = "need_des"]
pub mod core_dmactive_lpcore;
#[doc = "CORE_ERR_RESP_DIS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_err_resp_dis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_err_resp_dis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_err_resp_dis`] module"]
#[doc(alias = "CORE_ERR_RESP_DIS")]
pub type CoreErrRespDis = crate::Reg<core_err_resp_dis::CoreErrRespDisSpec>;
#[doc = "need_des"]
pub mod core_err_resp_dis;
#[doc = "CORE_TIMEOUT_INT_RAW (rw) register accessor: Hp core bus timeout interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_timeout_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_timeout_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_raw`] module"]
#[doc(alias = "CORE_TIMEOUT_INT_RAW")]
pub type CoreTimeoutIntRaw = crate::Reg<core_timeout_int_raw::CoreTimeoutIntRawSpec>;
#[doc = "Hp core bus timeout interrupt raw register"]
pub mod core_timeout_int_raw;
#[doc = "CORE_TIMEOUT_INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_timeout_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_st`] module"]
#[doc(alias = "CORE_TIMEOUT_INT_ST")]
pub type CoreTimeoutIntSt = crate::Reg<core_timeout_int_st::CoreTimeoutIntStSpec>;
#[doc = "masked interrupt register"]
pub mod core_timeout_int_st;
#[doc = "CORE_TIMEOUT_INT_ENA (rw) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_timeout_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_timeout_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_ena`] module"]
#[doc(alias = "CORE_TIMEOUT_INT_ENA")]
pub type CoreTimeoutIntEna = crate::Reg<core_timeout_int_ena::CoreTimeoutIntEnaSpec>;
#[doc = "masked interrupt register"]
pub mod core_timeout_int_ena;
#[doc = "CORE_TIMEOUT_INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_timeout_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_clr`] module"]
#[doc(alias = "CORE_TIMEOUT_INT_CLR")]
pub type CoreTimeoutIntClr = crate::Reg<core_timeout_int_clr::CoreTimeoutIntClrSpec>;
#[doc = "interrupt clear register"]
pub mod core_timeout_int_clr;
#[doc = "GPIO_O_HYS_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_o_hys_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_o_hys_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hys_ctrl0`] module"]
#[doc(alias = "GPIO_O_HYS_CTRL0")]
pub type GpioOHysCtrl0 = crate::Reg<gpio_o_hys_ctrl0::GpioOHysCtrl0Spec>;
#[doc = "NA"]
pub mod gpio_o_hys_ctrl0;
#[doc = "GPIO_O_HYS_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_o_hys_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_o_hys_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hys_ctrl1`] module"]
#[doc(alias = "GPIO_O_HYS_CTRL1")]
pub type GpioOHysCtrl1 = crate::Reg<gpio_o_hys_ctrl1::GpioOHysCtrl1Spec>;
#[doc = "NA"]
pub mod gpio_o_hys_ctrl1;
#[doc = "RSA_PD_CTRL (rw) register accessor: rsa pd ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_pd_ctrl`] module"]
#[doc(alias = "RSA_PD_CTRL")]
pub type RsaPdCtrl = crate::Reg<rsa_pd_ctrl::RsaPdCtrlSpec>;
#[doc = "rsa pd ctrl register"]
pub mod rsa_pd_ctrl;
#[doc = "ECC_PD_CTRL (rw) register accessor: ecc pd ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pd_ctrl`] module"]
#[doc(alias = "ECC_PD_CTRL")]
pub type EccPdCtrl = crate::Reg<ecc_pd_ctrl::EccPdCtrlSpec>;
#[doc = "ecc pd ctrl register"]
pub mod ecc_pd_ctrl;
#[doc = "RNG_CFG (rw) register accessor: rng cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cfg`] module"]
#[doc(alias = "RNG_CFG")]
pub type RngCfg = crate::Reg<rng_cfg::RngCfgSpec>;
#[doc = "rng cfg register"]
pub mod rng_cfg;
#[doc = "UART_PD_CTRL (rw) register accessor: ecc pd ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_pd_ctrl`] module"]
#[doc(alias = "UART_PD_CTRL")]
pub type UartPdCtrl = crate::Reg<uart_pd_ctrl::UartPdCtrlSpec>;
#[doc = "ecc pd ctrl register"]
pub mod uart_pd_ctrl;
#[doc = "PERI_MEM_CLK_FORCE_ON (rw) register accessor: hp peri mem clk force on regpster\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_mem_clk_force_on::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_mem_clk_force_on::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_mem_clk_force_on`] module"]
#[doc(alias = "PERI_MEM_CLK_FORCE_ON")]
pub type PeriMemClkForceOn = crate::Reg<peri_mem_clk_force_on::PeriMemClkForceOnSpec>;
#[doc = "hp peri mem clk force on regpster"]
pub mod peri_mem_clk_force_on;
#[doc = "USB_OTGHS_PHY_ST (r) register accessor: Usb otg2.0 PHY status register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_phy_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otghs_phy_st`] module"]
#[doc(alias = "USB_OTGHS_PHY_ST")]
pub type UsbOtghsPhySt = crate::Reg<usb_otghs_phy_st::UsbOtghsPhyStSpec>;
#[doc = "Usb otg2.0 PHY status register"]
pub mod usb_otghs_phy_st;
#[doc = "CPU_WAKEUP_EVENT (rw) register accessor: cpu wakeup event ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wakeup_event::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeup_event::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_wakeup_event`] module"]
#[doc(alias = "CPU_WAKEUP_EVENT")]
pub type CpuWakeupEvent = crate::Reg<cpu_wakeup_event::CpuWakeupEventSpec>;
#[doc = "cpu wakeup event ctrl register"]
pub mod cpu_wakeup_event;
#[doc = "HP2LP_INTR_GROUP0_EN (rw) register accessor: HpP2LP Interrupt Enable Register Group0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group0_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_intr_group0_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group0_en`] module"]
#[doc(alias = "HP2LP_INTR_GROUP0_EN")]
pub type Hp2lpIntrGroup0En = crate::Reg<hp2lp_intr_group0_en::Hp2lpIntrGroup0EnSpec>;
#[doc = "HpP2LP Interrupt Enable Register Group0"]
pub mod hp2lp_intr_group0_en;
#[doc = "HP2LP_INTR_GROUP1_EN (rw) register accessor: HpP2LP Interrupt Enable Register Group1\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group1_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_intr_group1_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group1_en`] module"]
#[doc(alias = "HP2LP_INTR_GROUP1_EN")]
pub type Hp2lpIntrGroup1En = crate::Reg<hp2lp_intr_group1_en::Hp2lpIntrGroup1EnSpec>;
#[doc = "HpP2LP Interrupt Enable Register Group1"]
pub mod hp2lp_intr_group1_en;
#[doc = "HP2LP_INTR_GROUP2_EN (rw) register accessor: HpP2LP Interrupt Enable Register Group2\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group2_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_intr_group2_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group2_en`] module"]
#[doc(alias = "HP2LP_INTR_GROUP2_EN")]
pub type Hp2lpIntrGroup2En = crate::Reg<hp2lp_intr_group2_en::Hp2lpIntrGroup2EnSpec>;
#[doc = "HpP2LP Interrupt Enable Register Group2"]
pub mod hp2lp_intr_group2_en;
#[doc = "HP2LP_INTR_GROUP3_EN (rw) register accessor: HpP2LP Interrupt Enable Register Group3\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group3_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_intr_group3_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group3_en`] module"]
#[doc(alias = "HP2LP_INTR_GROUP3_EN")]
pub type Hp2lpIntrGroup3En = crate::Reg<hp2lp_intr_group3_en::Hp2lpIntrGroup3EnSpec>;
#[doc = "HpP2LP Interrupt Enable Register Group3"]
pub mod hp2lp_intr_group3_en;
#[doc = "HP2LP_INTR_GROUP0_ST (r) register accessor: HpP2LP Interrupt Status Register Group0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group0_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group0_st`] module"]
#[doc(alias = "HP2LP_INTR_GROUP0_ST")]
pub type Hp2lpIntrGroup0St = crate::Reg<hp2lp_intr_group0_st::Hp2lpIntrGroup0StSpec>;
#[doc = "HpP2LP Interrupt Status Register Group0"]
pub mod hp2lp_intr_group0_st;
#[doc = "HP2LP_INTR_GROUP1_ST (r) register accessor: HpP2LP Interrupt Enable Register Group1\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group1_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group1_st`] module"]
#[doc(alias = "HP2LP_INTR_GROUP1_ST")]
pub type Hp2lpIntrGroup1St = crate::Reg<hp2lp_intr_group1_st::Hp2lpIntrGroup1StSpec>;
#[doc = "HpP2LP Interrupt Enable Register Group1"]
pub mod hp2lp_intr_group1_st;
#[doc = "HP2LP_INTR_GROUP2_ST (r) register accessor: HpP2LP Interrupt Enable Register Group2\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group2_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group2_st`] module"]
#[doc(alias = "HP2LP_INTR_GROUP2_ST")]
pub type Hp2lpIntrGroup2St = crate::Reg<hp2lp_intr_group2_st::Hp2lpIntrGroup2StSpec>;
#[doc = "HpP2LP Interrupt Enable Register Group2"]
pub mod hp2lp_intr_group2_st;
#[doc = "HP2LP_INTR_GROUP3_ST (r) register accessor: HpP2LP Interrupt Enable Register Group3\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group3_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_intr_group3_st`] module"]
#[doc(alias = "HP2LP_INTR_GROUP3_ST")]
pub type Hp2lpIntrGroup3St = crate::Reg<hp2lp_intr_group3_st::Hp2lpIntrGroup3StSpec>;
#[doc = "HpP2LP Interrupt Enable Register Group3"]
pub mod hp2lp_intr_group3_st;
#[doc = "HP2LP_WAKEUP_GROUP0_EN (rw) register accessor: HpP2LP Wakeup Enable Register Group0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_wakeup_group0_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_wakeup_group0_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_wakeup_group0_en`] module"]
#[doc(alias = "HP2LP_WAKEUP_GROUP0_EN")]
pub type Hp2lpWakeupGroup0En = crate::Reg<hp2lp_wakeup_group0_en::Hp2lpWakeupGroup0EnSpec>;
#[doc = "HpP2LP Wakeup Enable Register Group0"]
pub mod hp2lp_wakeup_group0_en;
#[doc = "HP2LP_WAKEUP_GROUP1_EN (rw) register accessor: HpP2LP Wakeup Enable Register Group1\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_wakeup_group1_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_wakeup_group1_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_wakeup_group1_en`] module"]
#[doc(alias = "HP2LP_WAKEUP_GROUP1_EN")]
pub type Hp2lpWakeupGroup1En = crate::Reg<hp2lp_wakeup_group1_en::Hp2lpWakeupGroup1EnSpec>;
#[doc = "HpP2LP Wakeup Enable Register Group1"]
pub mod hp2lp_wakeup_group1_en;
#[doc = "HP2LP_WAKEUP_GROUP2_EN (rw) register accessor: HpP2LP Wakeup Enable Register Group2\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_wakeup_group2_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_wakeup_group2_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_wakeup_group2_en`] module"]
#[doc(alias = "HP2LP_WAKEUP_GROUP2_EN")]
pub type Hp2lpWakeupGroup2En = crate::Reg<hp2lp_wakeup_group2_en::Hp2lpWakeupGroup2EnSpec>;
#[doc = "HpP2LP Wakeup Enable Register Group2"]
pub mod hp2lp_wakeup_group2_en;
#[doc = "HP2LP_WAKEUP_GROUP3_EN (rw) register accessor: HpP2LP Wakeup Enable Register Group3\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_wakeup_group3_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_wakeup_group3_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_wakeup_group3_en`] module"]
#[doc(alias = "HP2LP_WAKEUP_GROUP3_EN")]
pub type Hp2lpWakeupGroup3En = crate::Reg<hp2lp_wakeup_group3_en::Hp2lpWakeupGroup3EnSpec>;
#[doc = "HpP2LP Wakeup Enable Register Group3"]
pub mod hp2lp_wakeup_group3_en;
