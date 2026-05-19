#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_ch: [InCh; 3],
    out_ch: [OutCh; 3],
    arb_timeout: ArbTimeout,
    weight_en: WeightEn,
    in_mem_conf: InMemConf,
    intr_mem_start_addr: IntrMemStartAddr,
    intr_mem_end_addr: IntrMemEndAddr,
    extr_mem_start_addr: ExtrMemStartAddr,
    extr_mem_end_addr: ExtrMemEndAddr,
    in_reset_avail_ch: [InResetAvailCh; 3],
    out_reset_avail_ch: [OutResetAvailCh; 3],
    _reserved11: [u8; 0x04],
    misc_conf: MiscConf,
    rdn_result: RdnResult,
    rdn_eco_high: RdnEcoHigh,
    rdn_eco_low: RdnEcoLow,
    wresp_cnt: WrespCnt,
    rresp_cnt: RrespCnt,
    infifo_status1_ch0: InfifoStatus1Ch0,
    infifo_status1_ch1: InfifoStatus1Ch1,
    infifo_status1_ch2: InfifoStatus1Ch2,
    outfifo_status1_ch0: OutfifoStatus1Ch0,
    outfifo_status1_ch1: OutfifoStatus1Ch1,
    outfifo_status1_ch2: OutfifoStatus1Ch2,
    date: Date,
    link_switch_state: LinkSwitchState,
}
impl RegisterBlock {
    #[doc = "0x00..0x138 - Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub const fn in_ch(&self, n: usize) -> &InCh {
        &self.in_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x138 - Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub fn in_ch_iter(&self) -> impl Iterator<Item = &InCh> {
        self.in_ch.iter()
    }
    #[doc = "0x138..0x270 - Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub const fn out_ch(&self, n: usize) -> &OutCh {
        &self.out_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x138..0x270 - Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub fn out_ch_iter(&self) -> impl Iterator<Item = &OutCh> {
        self.out_ch.iter()
    }
    #[doc = "0x270 - This retister is used to config arbiter time slice"]
    #[inline(always)]
    pub const fn arb_timeout(&self) -> &ArbTimeout {
        &self.arb_timeout
    }
    #[doc = "0x274 - This register is used to config arbiter weight function to on or off"]
    #[inline(always)]
    pub const fn weight_en(&self) -> &WeightEn {
        &self.weight_en
    }
    #[doc = "0x278 - Mem power configure register of Rx channel"]
    #[inline(always)]
    pub const fn in_mem_conf(&self) -> &InMemConf {
        &self.in_mem_conf
    }
    #[doc = "0x27c - The start address of accessible address space."]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &IntrMemStartAddr {
        &self.intr_mem_start_addr
    }
    #[doc = "0x280 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &IntrMemEndAddr {
        &self.intr_mem_end_addr
    }
    #[doc = "0x284 - The start address of accessible address space."]
    #[inline(always)]
    pub const fn extr_mem_start_addr(&self) -> &ExtrMemStartAddr {
        &self.extr_mem_start_addr
    }
    #[doc = "0x288 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn extr_mem_end_addr(&self) -> &ExtrMemEndAddr {
        &self.extr_mem_end_addr
    }
    #[doc = "0x28c..0x298 - The rx channel %s reset valid_flag register."]
    #[inline(always)]
    pub const fn in_reset_avail_ch(&self, n: usize) -> &InResetAvailCh {
        &self.in_reset_avail_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28c..0x298 - The rx channel %s reset valid_flag register."]
    #[inline(always)]
    pub fn in_reset_avail_ch_iter(&self) -> impl Iterator<Item = &InResetAvailCh> {
        self.in_reset_avail_ch.iter()
    }
    #[doc = "0x298..0x2a4 - The tx channel %s reset valid_flag register."]
    #[inline(always)]
    pub const fn out_reset_avail_ch(&self, n: usize) -> &OutResetAvailCh {
        &self.out_reset_avail_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x298..0x2a4 - The tx channel %s reset valid_flag register."]
    #[inline(always)]
    pub fn out_reset_avail_ch_iter(&self) -> impl Iterator<Item = &OutResetAvailCh> {
        self.out_reset_avail_ch.iter()
    }
    #[doc = "0x2a8 - MISC register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MiscConf {
        &self.misc_conf
    }
    #[doc = "0x2ac - reserved"]
    #[inline(always)]
    pub const fn rdn_result(&self) -> &RdnResult {
        &self.rdn_result
    }
    #[doc = "0x2b0 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RdnEcoHigh {
        &self.rdn_eco_high
    }
    #[doc = "0x2b4 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RdnEcoLow {
        &self.rdn_eco_low
    }
    #[doc = "0x2b8 - AXI wr responce cnt register."]
    #[inline(always)]
    pub const fn wresp_cnt(&self) -> &WrespCnt {
        &self.wresp_cnt
    }
    #[doc = "0x2bc - AXI wr responce cnt register."]
    #[inline(always)]
    pub const fn rresp_cnt(&self) -> &RrespCnt {
        &self.rresp_cnt
    }
    #[doc = "0x2c0 - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub const fn infifo_status1_ch0(&self) -> &InfifoStatus1Ch0 {
        &self.infifo_status1_ch0
    }
    #[doc = "0x2c4 - Receive FIFO status of Rx channel 1"]
    #[inline(always)]
    pub const fn infifo_status1_ch1(&self) -> &InfifoStatus1Ch1 {
        &self.infifo_status1_ch1
    }
    #[doc = "0x2c8 - Receive FIFO status of Rx channel 1"]
    #[inline(always)]
    pub const fn infifo_status1_ch2(&self) -> &InfifoStatus1Ch2 {
        &self.infifo_status1_ch2
    }
    #[doc = "0x2cc - Receive FIFO status of Tx channel 0"]
    #[inline(always)]
    pub const fn outfifo_status1_ch0(&self) -> &OutfifoStatus1Ch0 {
        &self.outfifo_status1_ch0
    }
    #[doc = "0x2d0 - Receive FIFO status of Tx channel 1"]
    #[inline(always)]
    pub const fn outfifo_status1_ch1(&self) -> &OutfifoStatus1Ch1 {
        &self.outfifo_status1_ch1
    }
    #[doc = "0x2d4 - Receive FIFO status of Tx channel 1"]
    #[inline(always)]
    pub const fn outfifo_status1_ch2(&self) -> &OutfifoStatus1Ch2 {
        &self.outfifo_status1_ch2
    }
    #[doc = "0x2d8 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x2dc - Version control register"]
    #[inline(always)]
    pub const fn link_switch_state(&self) -> &LinkSwitchState {
        &self.link_switch_state
    }
}
#[doc = "Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
pub use self::in_ch::InCh;
#[doc = r"Cluster"]
#[doc = "Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
pub mod in_ch;
#[doc = "Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
pub use self::out_ch::OutCh;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
pub mod out_ch;
#[doc = "ARB_TIMEOUT (rw) register accessor: This retister is used to config arbiter time slice\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout`] module"]
#[doc(alias = "ARB_TIMEOUT")]
pub type ArbTimeout = crate::Reg<arb_timeout::ArbTimeoutSpec>;
#[doc = "This retister is used to config arbiter time slice"]
pub mod arb_timeout;
#[doc = "WEIGHT_EN (rw) register accessor: This register is used to config arbiter weight function to on or off\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en`] module"]
#[doc(alias = "WEIGHT_EN")]
pub type WeightEn = crate::Reg<weight_en::WeightEnSpec>;
#[doc = "This register is used to config arbiter weight function to on or off"]
pub mod weight_en;
#[doc = "IN_MEM_CONF (rw) register accessor: Mem power configure register of Rx channel\n\nYou can [`read`](crate::Reg::read) this register and get [`in_mem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_mem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_mem_conf`] module"]
#[doc(alias = "IN_MEM_CONF")]
pub type InMemConf = crate::Reg<in_mem_conf::InMemConfSpec>;
#[doc = "Mem power configure register of Rx channel"]
pub mod in_mem_conf;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
#[doc(alias = "INTR_MEM_START_ADDR")]
pub type IntrMemStartAddr = crate::Reg<intr_mem_start_addr::IntrMemStartAddrSpec>;
#[doc = "The start address of accessible address space."]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
#[doc(alias = "INTR_MEM_END_ADDR")]
pub type IntrMemEndAddr = crate::Reg<intr_mem_end_addr::IntrMemEndAddrSpec>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod intr_mem_end_addr;
#[doc = "EXTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`extr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_start_addr`] module"]
#[doc(alias = "EXTR_MEM_START_ADDR")]
pub type ExtrMemStartAddr = crate::Reg<extr_mem_start_addr::ExtrMemStartAddrSpec>;
#[doc = "The start address of accessible address space."]
pub mod extr_mem_start_addr;
#[doc = "EXTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::Reg::read) this register and get [`extr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_end_addr`] module"]
#[doc(alias = "EXTR_MEM_END_ADDR")]
pub type ExtrMemEndAddr = crate::Reg<extr_mem_end_addr::ExtrMemEndAddrSpec>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod extr_mem_end_addr;
#[doc = "IN_RESET_AVAIL_CH (r) register accessor: The rx channel %s reset valid_flag register.\n\nYou can [`read`](crate::Reg::read) this register and get [`in_reset_avail_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_reset_avail_ch`] module"]
#[doc(alias = "IN_RESET_AVAIL_CH")]
pub type InResetAvailCh = crate::Reg<in_reset_avail_ch::InResetAvailChSpec>;
#[doc = "The rx channel %s reset valid_flag register."]
pub mod in_reset_avail_ch;
#[doc = "OUT_RESET_AVAIL_CH (r) register accessor: The tx channel %s reset valid_flag register.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_reset_avail_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_reset_avail_ch`] module"]
#[doc(alias = "OUT_RESET_AVAIL_CH")]
pub type OutResetAvailCh = crate::Reg<out_reset_avail_ch::OutResetAvailChSpec>;
#[doc = "The tx channel %s reset valid_flag register."]
pub mod out_reset_avail_ch;
#[doc = "MISC_CONF (rw) register accessor: MISC register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
#[doc(alias = "MISC_CONF")]
pub type MiscConf = crate::Reg<misc_conf::MiscConfSpec>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "RDN_RESULT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_result`] module"]
#[doc(alias = "RDN_RESULT")]
pub type RdnResult = crate::Reg<rdn_result::RdnResultSpec>;
#[doc = "reserved"]
pub mod rdn_result;
#[doc = "RDN_ECO_HIGH (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_high`] module"]
#[doc(alias = "RDN_ECO_HIGH")]
pub type RdnEcoHigh = crate::Reg<rdn_eco_high::RdnEcoHighSpec>;
#[doc = "reserved"]
pub mod rdn_eco_high;
#[doc = "RDN_ECO_LOW (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_low`] module"]
#[doc(alias = "RDN_ECO_LOW")]
pub type RdnEcoLow = crate::Reg<rdn_eco_low::RdnEcoLowSpec>;
#[doc = "reserved"]
pub mod rdn_eco_low;
#[doc = "WRESP_CNT (r) register accessor: AXI wr responce cnt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wresp_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wresp_cnt`] module"]
#[doc(alias = "WRESP_CNT")]
pub type WrespCnt = crate::Reg<wresp_cnt::WrespCntSpec>;
#[doc = "AXI wr responce cnt register."]
pub mod wresp_cnt;
#[doc = "RRESP_CNT (r) register accessor: AXI wr responce cnt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rresp_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rresp_cnt`] module"]
#[doc(alias = "RRESP_CNT")]
pub type RrespCnt = crate::Reg<rresp_cnt::RrespCntSpec>;
#[doc = "AXI wr responce cnt register."]
pub mod rresp_cnt;
#[doc = "INFIFO_STATUS1_CH0 (r) register accessor: Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status1_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status1_ch0`] module"]
#[doc(alias = "INFIFO_STATUS1_CH0")]
pub type InfifoStatus1Ch0 = crate::Reg<infifo_status1_ch0::InfifoStatus1Ch0Spec>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status1_ch0;
#[doc = "INFIFO_STATUS1_CH1 (r) register accessor: Receive FIFO status of Rx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status1_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status1_ch1`] module"]
#[doc(alias = "INFIFO_STATUS1_CH1")]
pub type InfifoStatus1Ch1 = crate::Reg<infifo_status1_ch1::InfifoStatus1Ch1Spec>;
#[doc = "Receive FIFO status of Rx channel 1"]
pub mod infifo_status1_ch1;
#[doc = "INFIFO_STATUS1_CH2 (r) register accessor: Receive FIFO status of Rx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status1_ch2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status1_ch2`] module"]
#[doc(alias = "INFIFO_STATUS1_CH2")]
pub type InfifoStatus1Ch2 = crate::Reg<infifo_status1_ch2::InfifoStatus1Ch2Spec>;
#[doc = "Receive FIFO status of Rx channel 1"]
pub mod infifo_status1_ch2;
#[doc = "OUTFIFO_STATUS1_CH0 (r) register accessor: Receive FIFO status of Tx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status1_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status1_ch0`] module"]
#[doc(alias = "OUTFIFO_STATUS1_CH0")]
pub type OutfifoStatus1Ch0 = crate::Reg<outfifo_status1_ch0::OutfifoStatus1Ch0Spec>;
#[doc = "Receive FIFO status of Tx channel 0"]
pub mod outfifo_status1_ch0;
#[doc = "OUTFIFO_STATUS1_CH1 (r) register accessor: Receive FIFO status of Tx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status1_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status1_ch1`] module"]
#[doc(alias = "OUTFIFO_STATUS1_CH1")]
pub type OutfifoStatus1Ch1 = crate::Reg<outfifo_status1_ch1::OutfifoStatus1Ch1Spec>;
#[doc = "Receive FIFO status of Tx channel 1"]
pub mod outfifo_status1_ch1;
#[doc = "OUTFIFO_STATUS1_CH2 (r) register accessor: Receive FIFO status of Tx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status1_ch2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status1_ch2`] module"]
#[doc(alias = "OUTFIFO_STATUS1_CH2")]
pub type OutfifoStatus1Ch2 = crate::Reg<outfifo_status1_ch2::OutfifoStatus1Ch2Spec>;
#[doc = "Receive FIFO status of Tx channel 1"]
pub mod outfifo_status1_ch2;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control register"]
pub mod date;
#[doc = "LINK_SWITCH_STATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`link_switch_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_switch_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_switch_state`] module"]
#[doc(alias = "LINK_SWITCH_STATE")]
pub type LinkSwitchState = crate::Reg<link_switch_state::LinkSwitchStateSpec>;
#[doc = "Version control register"]
pub mod link_switch_state;
