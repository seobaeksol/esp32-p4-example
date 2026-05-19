#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: Cmd,
    _reserved1: [u8; 0x04],
    ctrl: Ctrl,
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    clock: Clock,
    user: User,
    user1: User1,
    user2: User2,
    _reserved8: [u8; 0x08],
    rd_status: RdStatus,
    _reserved9: [u8; 0x04],
    misc: Misc,
    _reserved10: [u8; 0x04],
    cache_fctrl: CacheFctrl,
    cache_sctrl: CacheSctrl,
    sram_cmd: SramCmd,
    sram_drd_cmd: SramDrdCmd,
    sram_dwr_cmd: SramDwrCmd,
    sram_clk: SramClk,
    fsm: Fsm,
    _reserved17: [u8; 0x68],
    int_ena: IntEna,
    int_clr: IntClr,
    int_raw: IntRaw,
    int_st: IntSt,
    _reserved21: [u8; 0x04],
    ddr: Ddr,
    spi_smem_ddr: SpiSmemDdr,
    dll_dly_db: DllDlyDb,
    dll_db_st0: DllDbSt0,
    dll_db_st1: DllDbSt1,
    _reserved26: [u8; 0x18],
    spi_fmem_pms_attr: [SpiFmemPmsAttr; 4],
    spi_fmem_pms_addr: [SpiFmemPmsAddr; 4],
    spi_fmem_pms_size: [SpiFmemPmsSize; 4],
    spi_smem_pms0_attr: SpiSmemPms0Attr,
    spi_smem_pms1_attr: SpiSmemPms1Attr,
    spi_smem_pms2_attr: SpiSmemPms2Attr,
    spi_smem_pms3_attr: SpiSmemPms3Attr,
    spi_smem_pms_addr: [SpiSmemPmsAddr; 4],
    spi_smem_pms_size: [SpiSmemPmsSize; 4],
    _reserved35: [u8; 0x04],
    pms_reject: PmsReject,
    ecc_ctrl: EccCtrl,
    ecc_err_addr: EccErrAddr,
    axi_err_addr: AxiErrAddr,
    spi_smem_ecc_ctrl: SpiSmemEccCtrl,
    spi_smem_axi_addr_ctrl: SpiSmemAxiAddrCtrl,
    axi_err_resp_en: AxiErrRespEn,
    timing_cali: TimingCali,
    din_mode: DinMode,
    din_num: DinNum,
    dout_mode: DoutMode,
    spi_smem_timing_cali: SpiSmemTimingCali,
    spi_smem_din_mode: SpiSmemDinMode,
    spi_smem_din_num: SpiSmemDinNum,
    spi_smem_dout_mode: SpiSmemDoutMode,
    spi_smem_ac: SpiSmemAc,
    spi_smem_din_hex_mode: SpiSmemDinHexMode,
    spi_smem_din_hex_num: SpiSmemDinHexNum,
    spi_smem_dout_hex_mode: SpiSmemDoutHexMode,
    _reserved54: [u8; 0x50],
    clock_gate: ClockGate,
    _reserved55: [u8; 0xfc],
    xts_plain_base: XtsPlainBase,
    _reserved56: [u8; 0x3c],
    xts_linesize: XtsLinesize,
    xts_destination: XtsDestination,
    xts_physical_address: XtsPhysicalAddress,
    xts_trigger: XtsTrigger,
    xts_release: XtsRelease,
    xts_destroy: XtsDestroy,
    xts_state: XtsState,
    xts_date: XtsDate,
    _reserved64: [u8; 0x1c],
    mmu_item_content: MmuItemContent,
    mmu_item_index: MmuItemIndex,
    mmu_power_ctrl: MmuPowerCtrl,
    dpa_ctrl: DpaCtrl,
    xts_pseudo_round_conf: XtsPseudoRoundConf,
    _reserved69: [u8; 0x60],
    registerrnd_eco_high: RegisterrndEcoHigh,
    registerrnd_eco_low: RegisterrndEcoLow,
    _reserved71: [u8; 0x04],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x08 - SPI0 control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - SPI0 control1 register."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x10 - SPI0 control2 register."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x14 - SPI clock division control register."]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x18 - SPI0 user register."]
    #[inline(always)]
    pub const fn user(&self) -> &User {
        &self.user
    }
    #[doc = "0x1c - SPI0 user1 register."]
    #[inline(always)]
    pub const fn user1(&self) -> &User1 {
        &self.user1
    }
    #[doc = "0x20 - SPI0 user2 register."]
    #[inline(always)]
    pub const fn user2(&self) -> &User2 {
        &self.user2
    }
    #[doc = "0x2c - SPI0 read control register."]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RdStatus {
        &self.rd_status
    }
    #[doc = "0x34 - SPI0 misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &Misc {
        &self.misc
    }
    #[doc = "0x3c - SPI0 bit mode control register."]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CacheFctrl {
        &self.cache_fctrl
    }
    #[doc = "0x40 - SPI0 external RAM control register"]
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CacheSctrl {
        &self.cache_sctrl
    }
    #[doc = "0x44 - SPI0 external RAM mode control register"]
    #[inline(always)]
    pub const fn sram_cmd(&self) -> &SramCmd {
        &self.sram_cmd
    }
    #[doc = "0x48 - SPI0 external RAM DDR read command control register"]
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SramDrdCmd {
        &self.sram_drd_cmd
    }
    #[doc = "0x4c - SPI0 external RAM DDR write command control register"]
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SramDwrCmd {
        &self.sram_dwr_cmd
    }
    #[doc = "0x50 - SPI0 external RAM clock control register"]
    #[inline(always)]
    pub const fn sram_clk(&self) -> &SramClk {
        &self.sram_clk
    }
    #[doc = "0x54 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn fsm(&self) -> &Fsm {
        &self.fsm
    }
    #[doc = "0xc0 - SPI0 interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0xc4 - SPI0 interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0xc8 - SPI0 interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0xcc - SPI0 interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0xd4 - SPI0 flash DDR mode control register"]
    #[inline(always)]
    pub const fn ddr(&self) -> &Ddr {
        &self.ddr
    }
    #[doc = "0xd8 - SPI0 external RAM DDR mode control register"]
    #[inline(always)]
    pub const fn spi_smem_ddr(&self) -> &SpiSmemDdr {
        &self.spi_smem_ddr
    }
    #[doc = "0xdc - MSPI DLL function and debug configuration register"]
    #[inline(always)]
    pub const fn dll_dly_db(&self) -> &DllDlyDb {
        &self.dll_dly_db
    }
    #[doc = "0xe0 - MSPI DLL debug status0 register"]
    #[inline(always)]
    pub const fn dll_db_st0(&self) -> &DllDbSt0 {
        &self.dll_db_st0
    }
    #[doc = "0xe4 - MSPI DLL debug status1 register"]
    #[inline(always)]
    pub const fn dll_db_st1(&self) -> &DllDbSt1 {
        &self.dll_db_st1
    }
    #[doc = "0x100..0x110 - SPI1 flash PMS section %s attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_attr(&self, n: usize) -> &SpiFmemPmsAttr {
        &self.spi_fmem_pms_attr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - SPI1 flash PMS section %s attribute register"]
    #[inline(always)]
    pub fn spi_fmem_pms_attr_iter(&self) -> impl Iterator<Item = &SpiFmemPmsAttr> {
        self.spi_fmem_pms_attr.iter()
    }
    #[doc = "0x100 - SPI1 flash PMS section 0 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_attr(&self) -> &SpiFmemPmsAttr {
        self.spi_fmem_pms_attr(0)
    }
    #[doc = "0x104 - SPI1 flash PMS section 1 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_attr(&self) -> &SpiFmemPmsAttr {
        self.spi_fmem_pms_attr(1)
    }
    #[doc = "0x108 - SPI1 flash PMS section 2 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_attr(&self) -> &SpiFmemPmsAttr {
        self.spi_fmem_pms_attr(2)
    }
    #[doc = "0x10c - SPI1 flash PMS section 3 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_attr(&self) -> &SpiFmemPmsAttr {
        self.spi_fmem_pms_attr(3)
    }
    #[doc = "0x110..0x120 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_addr(&self, n: usize) -> &SpiFmemPmsAddr {
        &self.spi_fmem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_fmem_pms_addr_iter(&self) -> impl Iterator<Item = &SpiFmemPmsAddr> {
        self.spi_fmem_pms_addr.iter()
    }
    #[doc = "0x110 - SPI1 flash PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_addr(&self) -> &SpiFmemPmsAddr {
        self.spi_fmem_pms_addr(0)
    }
    #[doc = "0x114 - SPI1 flash PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_addr(&self) -> &SpiFmemPmsAddr {
        self.spi_fmem_pms_addr(1)
    }
    #[doc = "0x118 - SPI1 flash PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_addr(&self) -> &SpiFmemPmsAddr {
        self.spi_fmem_pms_addr(2)
    }
    #[doc = "0x11c - SPI1 flash PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_addr(&self) -> &SpiFmemPmsAddr {
        self.spi_fmem_pms_addr(3)
    }
    #[doc = "0x120..0x130 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self, n: usize) -> &SpiFmemPmsSize {
        &self.spi_fmem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x130 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_fmem_pms_size_iter(&self) -> impl Iterator<Item = &SpiFmemPmsSize> {
        self.spi_fmem_pms_size.iter()
    }
    #[doc = "0x120 - SPI1 flash PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_size(&self) -> &SpiFmemPmsSize {
        self.spi_fmem_pms_size(0)
    }
    #[doc = "0x124 - SPI1 flash PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_size(&self) -> &SpiFmemPmsSize {
        self.spi_fmem_pms_size(1)
    }
    #[doc = "0x128 - SPI1 flash PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_size(&self) -> &SpiFmemPmsSize {
        self.spi_fmem_pms_size(2)
    }
    #[doc = "0x12c - SPI1 flash PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_size(&self) -> &SpiFmemPmsSize {
        self.spi_fmem_pms_size(3)
    }
    #[doc = "0x130 - SPI1 flash PMS section $n start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_attr(&self) -> &SpiSmemPms0Attr {
        &self.spi_smem_pms0_attr
    }
    #[doc = "0x134 - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_attr(&self) -> &SpiSmemPms1Attr {
        &self.spi_smem_pms1_attr
    }
    #[doc = "0x138 - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_attr(&self) -> &SpiSmemPms2Attr {
        &self.spi_smem_pms2_attr
    }
    #[doc = "0x13c - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_attr(&self) -> &SpiSmemPms3Attr {
        &self.spi_smem_pms3_attr
    }
    #[doc = "0x140..0x150 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_addr(&self, n: usize) -> &SpiSmemPmsAddr {
        &self.spi_smem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_addr_iter(&self) -> impl Iterator<Item = &SpiSmemPmsAddr> {
        self.spi_smem_pms_addr.iter()
    }
    #[doc = "0x140 - SPI1 external RAM PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_addr(&self) -> &SpiSmemPmsAddr {
        self.spi_smem_pms_addr(0)
    }
    #[doc = "0x144 - SPI1 external RAM PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_addr(&self) -> &SpiSmemPmsAddr {
        self.spi_smem_pms_addr(1)
    }
    #[doc = "0x148 - SPI1 external RAM PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_addr(&self) -> &SpiSmemPmsAddr {
        self.spi_smem_pms_addr(2)
    }
    #[doc = "0x14c - SPI1 external RAM PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_addr(&self) -> &SpiSmemPmsAddr {
        self.spi_smem_pms_addr(3)
    }
    #[doc = "0x150..0x160 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self, n: usize) -> &SpiSmemPmsSize {
        &self.spi_smem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_size_iter(&self) -> impl Iterator<Item = &SpiSmemPmsSize> {
        self.spi_smem_pms_size.iter()
    }
    #[doc = "0x150 - SPI1 external RAM PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_size(&self) -> &SpiSmemPmsSize {
        self.spi_smem_pms_size(0)
    }
    #[doc = "0x154 - SPI1 external RAM PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_size(&self) -> &SpiSmemPmsSize {
        self.spi_smem_pms_size(1)
    }
    #[doc = "0x158 - SPI1 external RAM PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_size(&self) -> &SpiSmemPmsSize {
        self.spi_smem_pms_size(2)
    }
    #[doc = "0x15c - SPI1 external RAM PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_size(&self) -> &SpiSmemPmsSize {
        self.spi_smem_pms_size(3)
    }
    #[doc = "0x164 - SPI1 access reject register"]
    #[inline(always)]
    pub const fn pms_reject(&self) -> &PmsReject {
        &self.pms_reject
    }
    #[doc = "0x168 - MSPI ECC control register"]
    #[inline(always)]
    pub const fn ecc_ctrl(&self) -> &EccCtrl {
        &self.ecc_ctrl
    }
    #[doc = "0x16c - MSPI ECC error address register"]
    #[inline(always)]
    pub const fn ecc_err_addr(&self) -> &EccErrAddr {
        &self.ecc_err_addr
    }
    #[doc = "0x170 - SPI0 AXI request error address."]
    #[inline(always)]
    pub const fn axi_err_addr(&self) -> &AxiErrAddr {
        &self.axi_err_addr
    }
    #[doc = "0x174 - MSPI ECC control register"]
    #[inline(always)]
    pub const fn spi_smem_ecc_ctrl(&self) -> &SpiSmemEccCtrl {
        &self.spi_smem_ecc_ctrl
    }
    #[doc = "0x178 - SPI0 AXI address control register"]
    #[inline(always)]
    pub const fn spi_smem_axi_addr_ctrl(&self) -> &SpiSmemAxiAddrCtrl {
        &self.spi_smem_axi_addr_ctrl
    }
    #[doc = "0x17c - SPI0 AXI error response enable register"]
    #[inline(always)]
    pub const fn axi_err_resp_en(&self) -> &AxiErrRespEn {
        &self.axi_err_resp_en
    }
    #[doc = "0x180 - SPI0 flash timing calibration register"]
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TimingCali {
        &self.timing_cali
    }
    #[doc = "0x184 - MSPI flash input timing delay mode control register"]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DinMode {
        &self.din_mode
    }
    #[doc = "0x188 - MSPI flash input timing delay number control register"]
    #[inline(always)]
    pub const fn din_num(&self) -> &DinNum {
        &self.din_num
    }
    #[doc = "0x18c - MSPI flash output timing adjustment control register"]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DoutMode {
        &self.dout_mode
    }
    #[doc = "0x190 - MSPI external RAM timing calibration register"]
    #[inline(always)]
    pub const fn spi_smem_timing_cali(&self) -> &SpiSmemTimingCali {
        &self.spi_smem_timing_cali
    }
    #[doc = "0x194 - MSPI external RAM input timing delay mode control register"]
    #[inline(always)]
    pub const fn spi_smem_din_mode(&self) -> &SpiSmemDinMode {
        &self.spi_smem_din_mode
    }
    #[doc = "0x198 - MSPI external RAM input timing delay number control register"]
    #[inline(always)]
    pub const fn spi_smem_din_num(&self) -> &SpiSmemDinNum {
        &self.spi_smem_din_num
    }
    #[doc = "0x19c - MSPI external RAM output timing adjustment control register"]
    #[inline(always)]
    pub const fn spi_smem_dout_mode(&self) -> &SpiSmemDoutMode {
        &self.spi_smem_dout_mode
    }
    #[doc = "0x1a0 - MSPI external RAM ECC and SPI CS timing control register"]
    #[inline(always)]
    pub const fn spi_smem_ac(&self) -> &SpiSmemAc {
        &self.spi_smem_ac
    }
    #[doc = "0x1a4 - MSPI 16x external RAM input timing delay mode control register"]
    #[inline(always)]
    pub const fn spi_smem_din_hex_mode(&self) -> &SpiSmemDinHexMode {
        &self.spi_smem_din_hex_mode
    }
    #[doc = "0x1a8 - MSPI 16x external RAM input timing delay number control register"]
    #[inline(always)]
    pub const fn spi_smem_din_hex_num(&self) -> &SpiSmemDinHexNum {
        &self.spi_smem_din_hex_num
    }
    #[doc = "0x1ac - MSPI 16x external RAM output timing adjustment control register"]
    #[inline(always)]
    pub const fn spi_smem_dout_hex_mode(&self) -> &SpiSmemDoutHexMode {
        &self.spi_smem_dout_hex_mode
    }
    #[doc = "0x200 - SPI0 clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &ClockGate {
        &self.clock_gate
    }
    #[doc = "0x300 - The base address of the memory that stores plaintext in Manual Encryption"]
    #[inline(always)]
    pub const fn xts_plain_base(&self) -> &XtsPlainBase {
        &self.xts_plain_base
    }
    #[doc = "0x340 - Manual Encryption Line-Size register"]
    #[inline(always)]
    pub const fn xts_linesize(&self) -> &XtsLinesize {
        &self.xts_linesize
    }
    #[doc = "0x344 - Manual Encryption destination register"]
    #[inline(always)]
    pub const fn xts_destination(&self) -> &XtsDestination {
        &self.xts_destination
    }
    #[doc = "0x348 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_physical_address(&self) -> &XtsPhysicalAddress {
        &self.xts_physical_address
    }
    #[doc = "0x34c - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_trigger(&self) -> &XtsTrigger {
        &self.xts_trigger
    }
    #[doc = "0x350 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_release(&self) -> &XtsRelease {
        &self.xts_release
    }
    #[doc = "0x354 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_destroy(&self) -> &XtsDestroy {
        &self.xts_destroy
    }
    #[doc = "0x358 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_state(&self) -> &XtsState {
        &self.xts_state
    }
    #[doc = "0x35c - Manual Encryption version register"]
    #[inline(always)]
    pub const fn xts_date(&self) -> &XtsDate {
        &self.xts_date
    }
    #[doc = "0x37c - MSPI-MMU item content register"]
    #[inline(always)]
    pub const fn mmu_item_content(&self) -> &MmuItemContent {
        &self.mmu_item_content
    }
    #[doc = "0x380 - MSPI-MMU item index register"]
    #[inline(always)]
    pub const fn mmu_item_index(&self) -> &MmuItemIndex {
        &self.mmu_item_index
    }
    #[doc = "0x384 - MSPI MMU power control register"]
    #[inline(always)]
    pub const fn mmu_power_ctrl(&self) -> &MmuPowerCtrl {
        &self.mmu_power_ctrl
    }
    #[doc = "0x388 - SPI memory cryption DPA register"]
    #[inline(always)]
    pub const fn dpa_ctrl(&self) -> &DpaCtrl {
        &self.dpa_ctrl
    }
    #[doc = "0x38c - SPI memory cryption PSEUDO register"]
    #[inline(always)]
    pub const fn xts_pseudo_round_conf(&self) -> &XtsPseudoRoundConf {
        &self.xts_pseudo_round_conf
    }
    #[doc = "0x3f0 - MSPI ECO high register"]
    #[inline(always)]
    pub const fn registerrnd_eco_high(&self) -> &RegisterrndEcoHigh {
        &self.registerrnd_eco_high
    }
    #[doc = "0x3f4 - MSPI ECO low register"]
    #[inline(always)]
    pub const fn registerrnd_eco_low(&self) -> &RegisterrndEcoLow {
        &self.registerrnd_eco_low
    }
    #[doc = "0x3fc - SPI0 version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "CMD (r) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "SPI0 FSM status register"]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: SPI0 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SPI0 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI0 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "SPI0 control1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: SPI0 control2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "SPI0 control2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI clock division control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "SPI clock division control register."]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI0 user register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
#[doc(alias = "USER")]
pub type User = crate::Reg<user::UserSpec>;
#[doc = "SPI0 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI0 user1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
#[doc(alias = "USER1")]
pub type User1 = crate::Reg<user1::User1Spec>;
#[doc = "SPI0 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI0 user2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
#[doc(alias = "USER2")]
pub type User2 = crate::Reg<user2::User2Spec>;
#[doc = "SPI0 user2 register."]
pub mod user2;
#[doc = "RD_STATUS (rw) register accessor: SPI0 read control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
#[doc(alias = "RD_STATUS")]
pub type RdStatus = crate::Reg<rd_status::RdStatusSpec>;
#[doc = "SPI0 read control register."]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
#[doc(alias = "MISC")]
pub type Misc = crate::Reg<misc::MiscSpec>;
#[doc = "SPI0 misc register"]
pub mod misc;
#[doc = "CACHE_FCTRL (rw) register accessor: SPI0 bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
#[doc(alias = "CACHE_FCTRL")]
pub type CacheFctrl = crate::Reg<cache_fctrl::CacheFctrlSpec>;
#[doc = "SPI0 bit mode control register."]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL (rw) register accessor: SPI0 external RAM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sctrl`] module"]
#[doc(alias = "CACHE_SCTRL")]
pub type CacheSctrl = crate::Reg<cache_sctrl::CacheSctrlSpec>;
#[doc = "SPI0 external RAM control register"]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: SPI0 external RAM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cmd`] module"]
#[doc(alias = "SRAM_CMD")]
pub type SramCmd = crate::Reg<sram_cmd::SramCmdSpec>;
#[doc = "SPI0 external RAM mode control register"]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: SPI0 external RAM DDR read command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_drd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_drd_cmd`] module"]
#[doc(alias = "SRAM_DRD_CMD")]
pub type SramDrdCmd = crate::Reg<sram_drd_cmd::SramDrdCmdSpec>;
#[doc = "SPI0 external RAM DDR read command control register"]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (rw) register accessor: SPI0 external RAM DDR write command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_dwr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_dwr_cmd`] module"]
#[doc(alias = "SRAM_DWR_CMD")]
pub type SramDwrCmd = crate::Reg<sram_dwr_cmd::SramDwrCmdSpec>;
#[doc = "SPI0 external RAM DDR write command control register"]
pub mod sram_dwr_cmd;
#[doc = "SRAM_CLK (rw) register accessor: SPI0 external RAM clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_clk`] module"]
#[doc(alias = "SRAM_CLK")]
pub type SramClk = crate::Reg<sram_clk::SramClkSpec>;
#[doc = "SPI0 external RAM clock control register"]
pub mod sram_clk;
#[doc = "FSM (rw) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
#[doc(alias = "FSM")]
pub type Fsm = crate::Reg<fsm::FsmSpec>;
#[doc = "SPI0 FSM status register"]
pub mod fsm;
#[doc = "INT_ENA (rw) register accessor: SPI0 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "SPI0 interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: SPI0 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "SPI0 interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: SPI0 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "SPI0 interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: SPI0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "SPI0 interrupt status register"]
pub mod int_st;
#[doc = "DDR (rw) register accessor: SPI0 flash DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
#[doc(alias = "DDR")]
pub type Ddr = crate::Reg<ddr::DdrSpec>;
#[doc = "SPI0 flash DDR mode control register"]
pub mod ddr;
#[doc = "SPI_SMEM_DDR (rw) register accessor: SPI0 external RAM DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ddr`] module"]
#[doc(alias = "SPI_SMEM_DDR")]
pub type SpiSmemDdr = crate::Reg<spi_smem_ddr::SpiSmemDdrSpec>;
#[doc = "SPI0 external RAM DDR mode control register"]
pub mod spi_smem_ddr;
#[doc = "DLL_DLY_DB (rw) register accessor: MSPI DLL function and debug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_dly_db::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_dly_db::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_dly_db`] module"]
#[doc(alias = "DLL_DLY_DB")]
pub type DllDlyDb = crate::Reg<dll_dly_db::DllDlyDbSpec>;
#[doc = "MSPI DLL function and debug configuration register"]
pub mod dll_dly_db;
#[doc = "DLL_DB_ST0 (r) register accessor: MSPI DLL debug status0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_db_st0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_db_st0`] module"]
#[doc(alias = "DLL_DB_ST0")]
pub type DllDbSt0 = crate::Reg<dll_db_st0::DllDbSt0Spec>;
#[doc = "MSPI DLL debug status0 register"]
pub mod dll_db_st0;
#[doc = "DLL_DB_ST1 (r) register accessor: MSPI DLL debug status1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_db_st1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_db_st1`] module"]
#[doc(alias = "DLL_DB_ST1")]
pub type DllDbSt1 = crate::Reg<dll_db_st1::DllDbSt1Spec>;
#[doc = "MSPI DLL debug status1 register"]
pub mod dll_db_st1;
#[doc = "SPI_FMEM_PMS_ATTR (rw) register accessor: SPI1 flash PMS section %s attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_attr`] module"]
#[doc(alias = "SPI_FMEM_PMS_ATTR")]
pub type SpiFmemPmsAttr = crate::Reg<spi_fmem_pms_attr::SpiFmemPmsAttrSpec>;
#[doc = "SPI1 flash PMS section %s attribute register"]
pub mod spi_fmem_pms_attr;
#[doc = "SPI_FMEM_PMS_ADDR (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_addr`] module"]
#[doc(alias = "SPI_FMEM_PMS_ADDR")]
pub type SpiFmemPmsAddr = crate::Reg<spi_fmem_pms_addr::SpiFmemPmsAddrSpec>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod spi_fmem_pms_addr;
#[doc = "SPI_FMEM_PMS_SIZE (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_size`] module"]
#[doc(alias = "SPI_FMEM_PMS_SIZE")]
pub type SpiFmemPmsSize = crate::Reg<spi_fmem_pms_size::SpiFmemPmsSizeSpec>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod spi_fmem_pms_size;
#[doc = "SPI_SMEM_PMS0_ATTR (rw) register accessor: SPI1 flash PMS section $n start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms0_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms0_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms0_attr`] module"]
#[doc(alias = "SPI_SMEM_PMS0_ATTR")]
pub type SpiSmemPms0Attr = crate::Reg<spi_smem_pms0_attr::SpiSmemPms0AttrSpec>;
#[doc = "SPI1 flash PMS section $n start address register"]
pub mod spi_smem_pms0_attr;
#[doc = "SPI_SMEM_PMS1_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms1_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms1_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms1_attr`] module"]
#[doc(alias = "SPI_SMEM_PMS1_ATTR")]
pub type SpiSmemPms1Attr = crate::Reg<spi_smem_pms1_attr::SpiSmemPms1AttrSpec>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod spi_smem_pms1_attr;
#[doc = "SPI_SMEM_PMS2_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms2_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms2_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms2_attr`] module"]
#[doc(alias = "SPI_SMEM_PMS2_ATTR")]
pub type SpiSmemPms2Attr = crate::Reg<spi_smem_pms2_attr::SpiSmemPms2AttrSpec>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod spi_smem_pms2_attr;
#[doc = "SPI_SMEM_PMS3_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms3_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms3_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms3_attr`] module"]
#[doc(alias = "SPI_SMEM_PMS3_ATTR")]
pub type SpiSmemPms3Attr = crate::Reg<spi_smem_pms3_attr::SpiSmemPms3AttrSpec>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod spi_smem_pms3_attr;
#[doc = "SPI_SMEM_PMS_ADDR (rw) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms_addr`] module"]
#[doc(alias = "SPI_SMEM_PMS_ADDR")]
pub type SpiSmemPmsAddr = crate::Reg<spi_smem_pms_addr::SpiSmemPmsAddrSpec>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod spi_smem_pms_addr;
#[doc = "SPI_SMEM_PMS_SIZE (rw) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms_size`] module"]
#[doc(alias = "SPI_SMEM_PMS_SIZE")]
pub type SpiSmemPmsSize = crate::Reg<spi_smem_pms_size::SpiSmemPmsSizeSpec>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod spi_smem_pms_size;
#[doc = "PMS_REJECT (rw) register accessor: SPI1 access reject register\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_reject::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_reject`] module"]
#[doc(alias = "PMS_REJECT")]
pub type PmsReject = crate::Reg<pms_reject::PmsRejectSpec>;
#[doc = "SPI1 access reject register"]
pub mod pms_reject;
#[doc = "ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctrl`] module"]
#[doc(alias = "ECC_CTRL")]
pub type EccCtrl = crate::Reg<ecc_ctrl::EccCtrlSpec>;
#[doc = "MSPI ECC control register"]
pub mod ecc_ctrl;
#[doc = "ECC_ERR_ADDR (r) register accessor: MSPI ECC error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_err_addr`] module"]
#[doc(alias = "ECC_ERR_ADDR")]
pub type EccErrAddr = crate::Reg<ecc_err_addr::EccErrAddrSpec>;
#[doc = "MSPI ECC error address register"]
pub mod ecc_err_addr;
#[doc = "AXI_ERR_ADDR (r) register accessor: SPI0 AXI request error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_err_addr`] module"]
#[doc(alias = "AXI_ERR_ADDR")]
pub type AxiErrAddr = crate::Reg<axi_err_addr::AxiErrAddrSpec>;
#[doc = "SPI0 AXI request error address."]
pub mod axi_err_addr;
#[doc = "SPI_SMEM_ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ecc_ctrl`] module"]
#[doc(alias = "SPI_SMEM_ECC_CTRL")]
pub type SpiSmemEccCtrl = crate::Reg<spi_smem_ecc_ctrl::SpiSmemEccCtrlSpec>;
#[doc = "MSPI ECC control register"]
pub mod spi_smem_ecc_ctrl;
#[doc = "SPI_SMEM_AXI_ADDR_CTRL (r) register accessor: SPI0 AXI address control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_axi_addr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_axi_addr_ctrl`] module"]
#[doc(alias = "SPI_SMEM_AXI_ADDR_CTRL")]
pub type SpiSmemAxiAddrCtrl = crate::Reg<spi_smem_axi_addr_ctrl::SpiSmemAxiAddrCtrlSpec>;
#[doc = "SPI0 AXI address control register"]
pub mod spi_smem_axi_addr_ctrl;
#[doc = "AXI_ERR_RESP_EN (rw) register accessor: SPI0 AXI error response enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_resp_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_err_resp_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_err_resp_en`] module"]
#[doc(alias = "AXI_ERR_RESP_EN")]
pub type AxiErrRespEn = crate::Reg<axi_err_resp_en::AxiErrRespEnSpec>;
#[doc = "SPI0 AXI error response enable register"]
pub mod axi_err_resp_en;
#[doc = "TIMING_CALI (rw) register accessor: SPI0 flash timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
#[doc(alias = "TIMING_CALI")]
pub type TimingCali = crate::Reg<timing_cali::TimingCaliSpec>;
#[doc = "SPI0 flash timing calibration register"]
pub mod timing_cali;
#[doc = "DIN_MODE (rw) register accessor: MSPI flash input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
#[doc(alias = "DIN_MODE")]
pub type DinMode = crate::Reg<din_mode::DinModeSpec>;
#[doc = "MSPI flash input timing delay mode control register"]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: MSPI flash input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
#[doc(alias = "DIN_NUM")]
pub type DinNum = crate::Reg<din_num::DinNumSpec>;
#[doc = "MSPI flash input timing delay number control register"]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
#[doc(alias = "DOUT_MODE")]
pub type DoutMode = crate::Reg<dout_mode::DoutModeSpec>;
#[doc = "MSPI flash output timing adjustment control register"]
pub mod dout_mode;
#[doc = "SPI_SMEM_TIMING_CALI (rw) register accessor: MSPI external RAM timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_timing_cali`] module"]
#[doc(alias = "SPI_SMEM_TIMING_CALI")]
pub type SpiSmemTimingCali = crate::Reg<spi_smem_timing_cali::SpiSmemTimingCaliSpec>;
#[doc = "MSPI external RAM timing calibration register"]
pub mod spi_smem_timing_cali;
#[doc = "SPI_SMEM_DIN_MODE (rw) register accessor: MSPI external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_mode`] module"]
#[doc(alias = "SPI_SMEM_DIN_MODE")]
pub type SpiSmemDinMode = crate::Reg<spi_smem_din_mode::SpiSmemDinModeSpec>;
#[doc = "MSPI external RAM input timing delay mode control register"]
pub mod spi_smem_din_mode;
#[doc = "SPI_SMEM_DIN_NUM (rw) register accessor: MSPI external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_num`] module"]
#[doc(alias = "SPI_SMEM_DIN_NUM")]
pub type SpiSmemDinNum = crate::Reg<spi_smem_din_num::SpiSmemDinNumSpec>;
#[doc = "MSPI external RAM input timing delay number control register"]
pub mod spi_smem_din_num;
#[doc = "SPI_SMEM_DOUT_MODE (rw) register accessor: MSPI external RAM output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_dout_mode`] module"]
#[doc(alias = "SPI_SMEM_DOUT_MODE")]
pub type SpiSmemDoutMode = crate::Reg<spi_smem_dout_mode::SpiSmemDoutModeSpec>;
#[doc = "MSPI external RAM output timing adjustment control register"]
pub mod spi_smem_dout_mode;
#[doc = "SPI_SMEM_AC (rw) register accessor: MSPI external RAM ECC and SPI CS timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ac`] module"]
#[doc(alias = "SPI_SMEM_AC")]
pub type SpiSmemAc = crate::Reg<spi_smem_ac::SpiSmemAcSpec>;
#[doc = "MSPI external RAM ECC and SPI CS timing control register"]
pub mod spi_smem_ac;
#[doc = "SPI_SMEM_DIN_HEX_MODE (rw) register accessor: MSPI 16x external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_hex_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_hex_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_hex_mode`] module"]
#[doc(alias = "SPI_SMEM_DIN_HEX_MODE")]
pub type SpiSmemDinHexMode = crate::Reg<spi_smem_din_hex_mode::SpiSmemDinHexModeSpec>;
#[doc = "MSPI 16x external RAM input timing delay mode control register"]
pub mod spi_smem_din_hex_mode;
#[doc = "SPI_SMEM_DIN_HEX_NUM (rw) register accessor: MSPI 16x external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_hex_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_hex_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_hex_num`] module"]
#[doc(alias = "SPI_SMEM_DIN_HEX_NUM")]
pub type SpiSmemDinHexNum = crate::Reg<spi_smem_din_hex_num::SpiSmemDinHexNumSpec>;
#[doc = "MSPI 16x external RAM input timing delay number control register"]
pub mod spi_smem_din_hex_num;
#[doc = "SPI_SMEM_DOUT_HEX_MODE (rw) register accessor: MSPI 16x external RAM output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_dout_hex_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_dout_hex_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_dout_hex_mode`] module"]
#[doc(alias = "SPI_SMEM_DOUT_HEX_MODE")]
pub type SpiSmemDoutHexMode = crate::Reg<spi_smem_dout_hex_mode::SpiSmemDoutHexModeSpec>;
#[doc = "MSPI 16x external RAM output timing adjustment control register"]
pub mod spi_smem_dout_hex_mode;
#[doc = "CLOCK_GATE (rw) register accessor: SPI0 clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
#[doc(alias = "CLOCK_GATE")]
pub type ClockGate = crate::Reg<clock_gate::ClockGateSpec>;
#[doc = "SPI0 clock gate register"]
pub mod clock_gate;
#[doc = "XTS_PLAIN_BASE (rw) register accessor: The base address of the memory that stores plaintext in Manual Encryption\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_plain_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_plain_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_plain_base`] module"]
#[doc(alias = "XTS_PLAIN_BASE")]
pub type XtsPlainBase = crate::Reg<xts_plain_base::XtsPlainBaseSpec>;
#[doc = "The base address of the memory that stores plaintext in Manual Encryption"]
pub mod xts_plain_base;
#[doc = "XTS_LINESIZE (rw) register accessor: Manual Encryption Line-Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_linesize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_linesize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_linesize`] module"]
#[doc(alias = "XTS_LINESIZE")]
pub type XtsLinesize = crate::Reg<xts_linesize::XtsLinesizeSpec>;
#[doc = "Manual Encryption Line-Size register"]
pub mod xts_linesize;
#[doc = "XTS_DESTINATION (rw) register accessor: Manual Encryption destination register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_destination::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destination::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_destination`] module"]
#[doc(alias = "XTS_DESTINATION")]
pub type XtsDestination = crate::Reg<xts_destination::XtsDestinationSpec>;
#[doc = "Manual Encryption destination register"]
pub mod xts_destination;
#[doc = "XTS_PHYSICAL_ADDRESS (rw) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_physical_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_physical_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_physical_address`] module"]
#[doc(alias = "XTS_PHYSICAL_ADDRESS")]
pub type XtsPhysicalAddress = crate::Reg<xts_physical_address::XtsPhysicalAddressSpec>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_physical_address;
#[doc = "XTS_TRIGGER (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_trigger`] module"]
#[doc(alias = "XTS_TRIGGER")]
pub type XtsTrigger = crate::Reg<xts_trigger::XtsTriggerSpec>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_trigger;
#[doc = "XTS_RELEASE (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_release`] module"]
#[doc(alias = "XTS_RELEASE")]
pub type XtsRelease = crate::Reg<xts_release::XtsReleaseSpec>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_release;
#[doc = "XTS_DESTROY (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_destroy`] module"]
#[doc(alias = "XTS_DESTROY")]
pub type XtsDestroy = crate::Reg<xts_destroy::XtsDestroySpec>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_destroy;
#[doc = "XTS_STATE (r) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_state`] module"]
#[doc(alias = "XTS_STATE")]
pub type XtsState = crate::Reg<xts_state::XtsStateSpec>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_state;
#[doc = "XTS_DATE (rw) register accessor: Manual Encryption version register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_date`] module"]
#[doc(alias = "XTS_DATE")]
pub type XtsDate = crate::Reg<xts_date::XtsDateSpec>;
#[doc = "Manual Encryption version register"]
pub mod xts_date;
#[doc = "MMU_ITEM_CONTENT (rw) register accessor: MSPI-MMU item content register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_content::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_content::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_item_content`] module"]
#[doc(alias = "MMU_ITEM_CONTENT")]
pub type MmuItemContent = crate::Reg<mmu_item_content::MmuItemContentSpec>;
#[doc = "MSPI-MMU item content register"]
pub mod mmu_item_content;
#[doc = "MMU_ITEM_INDEX (rw) register accessor: MSPI-MMU item index register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_item_index`] module"]
#[doc(alias = "MMU_ITEM_INDEX")]
pub type MmuItemIndex = crate::Reg<mmu_item_index::MmuItemIndexSpec>;
#[doc = "MSPI-MMU item index register"]
pub mod mmu_item_index;
#[doc = "MMU_POWER_CTRL (rw) register accessor: MSPI MMU power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_power_ctrl`] module"]
#[doc(alias = "MMU_POWER_CTRL")]
pub type MmuPowerCtrl = crate::Reg<mmu_power_ctrl::MmuPowerCtrlSpec>;
#[doc = "MSPI MMU power control register"]
pub mod mmu_power_ctrl;
#[doc = "DPA_CTRL (rw) register accessor: SPI memory cryption DPA register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_ctrl`] module"]
#[doc(alias = "DPA_CTRL")]
pub type DpaCtrl = crate::Reg<dpa_ctrl::DpaCtrlSpec>;
#[doc = "SPI memory cryption DPA register"]
pub mod dpa_ctrl;
#[doc = "XTS_PSEUDO_ROUND_CONF (rw) register accessor: SPI memory cryption PSEUDO register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_pseudo_round_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_pseudo_round_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_pseudo_round_conf`] module"]
#[doc(alias = "XTS_PSEUDO_ROUND_CONF")]
pub type XtsPseudoRoundConf = crate::Reg<xts_pseudo_round_conf::XtsPseudoRoundConfSpec>;
#[doc = "SPI memory cryption PSEUDO register"]
pub mod xts_pseudo_round_conf;
#[doc = "REGISTERRND_ECO_HIGH (rw) register accessor: MSPI ECO high register\n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@registerrnd_eco_high`] module"]
#[doc(alias = "REGISTERRND_ECO_HIGH")]
pub type RegisterrndEcoHigh = crate::Reg<registerrnd_eco_high::RegisterrndEcoHighSpec>;
#[doc = "MSPI ECO high register"]
pub mod registerrnd_eco_high;
#[doc = "REGISTERRND_ECO_LOW (rw) register accessor: MSPI ECO low register\n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@registerrnd_eco_low`] module"]
#[doc(alias = "REGISTERRND_ECO_LOW")]
pub type RegisterrndEcoLow = crate::Reg<registerrnd_eco_low::RegisterrndEcoLowSpec>;
#[doc = "MSPI ECO low register"]
pub mod registerrnd_eco_low;
#[doc = "DATE (rw) register accessor: SPI0 version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "SPI0 version control register"]
pub mod date;
