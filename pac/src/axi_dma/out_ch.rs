#[repr(C)]
#[doc = "Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
#[doc(alias = "OUT_CH")]
pub struct OutCh {
    out_int: OutInt,
    out_conf0: OutConf0,
    out_conf1: OutConf1,
    outfifo_status: OutfifoStatus,
    out_push: OutPush,
    out_link1: OutLink1,
    out_link2: OutLink2,
    out_state: OutState,
    out_eof_des_addr: OutEofDesAddr,
    out_eof_bfr_des_addr: OutEofBfrDesAddr,
    out_dscr: OutDscr,
    out_dscr_bf0: OutDscrBf0,
    out_dscr_bf1: OutDscrBf1,
    out_pri: OutPri,
    out_peri_sel: OutPeriSel,
    crc: Crc,
}
impl OutCh {
    #[doc = "0x00..0x10 - Cluster OUT_INT, containing OUT_INT_RAW, OUT_INT_ST, OUT_INT_ENA, OUT_INT_CLR"]
    #[inline(always)]
    pub const fn out_int(&self) -> &OutInt {
        &self.out_int
    }
    #[doc = "0x10 - Configure 0 register of Tx channel0"]
    #[inline(always)]
    pub const fn out_conf0(&self) -> &OutConf0 {
        &self.out_conf0
    }
    #[doc = "0x14 - Configure 1 register of Tx channel0"]
    #[inline(always)]
    pub const fn out_conf1(&self) -> &OutConf1 {
        &self.out_conf1
    }
    #[doc = "0x18 - Transmit FIFO status of Tx channel0"]
    #[inline(always)]
    pub const fn outfifo_status(&self) -> &OutfifoStatus {
        &self.outfifo_status
    }
    #[doc = "0x1c - Push control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_push(&self) -> &OutPush {
        &self.out_push
    }
    #[doc = "0x20 - Link descriptor configure and control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_link1(&self) -> &OutLink1 {
        &self.out_link1
    }
    #[doc = "0x24 - Link descriptor configure and control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_link2(&self) -> &OutLink2 {
        &self.out_link2
    }
    #[doc = "0x28 - Transmit status of Tx channel0"]
    #[inline(always)]
    pub const fn out_state(&self) -> &OutState {
        &self.out_state
    }
    #[doc = "0x2c - Outlink descriptor address when EOF occurs of Tx channel0"]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OutEofDesAddr {
        &self.out_eof_des_addr
    }
    #[doc = "0x30 - The last outlink descriptor address when EOF occurs of Tx channel0"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OutEofBfrDesAddr {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x34 - Current outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr(&self) -> &OutDscr {
        &self.out_dscr
    }
    #[doc = "0x38 - The last outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr_bf0(&self) -> &OutDscrBf0 {
        &self.out_dscr_bf0
    }
    #[doc = "0x3c - The second-to-last outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr_bf1(&self) -> &OutDscrBf1 {
        &self.out_dscr_bf1
    }
    #[doc = "0x40 - Priority register of Tx channel0."]
    #[inline(always)]
    pub const fn out_pri(&self) -> &OutPri {
        &self.out_pri
    }
    #[doc = "0x44 - Peripheral selection of Tx channel0"]
    #[inline(always)]
    pub const fn out_peri_sel(&self) -> &OutPeriSel {
        &self.out_peri_sel
    }
    #[doc = "0x48..0x68 - Cluster CRC, containing OUT_CRC_INIT_DATA, TX_CRC_WIDTH, OUT_CRC_CLEAR, OUT_CRC_FINAL_RESULT, TX_CRC_EN_WR_DATA, TX_CRC_EN_ADDR, TX_CRC_DATA_EN_WR_DATA, TX_CRC_DATA_EN_ADDR"]
    #[inline(always)]
    pub const fn crc(&self) -> &Crc {
        &self.crc
    }
}
#[doc = "Cluster OUT_INT, containing OUT_INT_RAW, OUT_INT_ST, OUT_INT_ENA, OUT_INT_CLR"]
pub use self::out_int::OutInt;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_INT, containing OUT_INT_RAW, OUT_INT_ST, OUT_INT_ENA, OUT_INT_CLR"]
pub mod out_int;
#[doc = "OUT_CONF0 (rw) register accessor: Configure 0 register of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0`] module"]
#[doc(alias = "OUT_CONF0")]
pub type OutConf0 = crate::Reg<out_conf0::OutConf0Spec>;
#[doc = "Configure 0 register of Tx channel0"]
pub mod out_conf0;
#[doc = "OUT_CONF1 (rw) register accessor: Configure 1 register of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1`] module"]
#[doc(alias = "OUT_CONF1")]
pub type OutConf1 = crate::Reg<out_conf1::OutConf1Spec>;
#[doc = "Configure 1 register of Tx channel0"]
pub mod out_conf1;
#[doc = "OUTFIFO_STATUS (r) register accessor: Transmit FIFO status of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status`] module"]
#[doc(alias = "OUTFIFO_STATUS")]
pub type OutfifoStatus = crate::Reg<outfifo_status::OutfifoStatusSpec>;
#[doc = "Transmit FIFO status of Tx channel0"]
pub mod outfifo_status;
#[doc = "OUT_PUSH (rw) register accessor: Push control register of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push`] module"]
#[doc(alias = "OUT_PUSH")]
pub type OutPush = crate::Reg<out_push::OutPushSpec>;
#[doc = "Push control register of Tx channel0"]
pub mod out_push;
#[doc = "OUT_LINK1 (rw) register accessor: Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link1`] module"]
#[doc(alias = "OUT_LINK1")]
pub type OutLink1 = crate::Reg<out_link1::OutLink1Spec>;
#[doc = "Link descriptor configure and control register of Tx channel0"]
pub mod out_link1;
#[doc = "OUT_LINK2 (rw) register accessor: Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link2`] module"]
#[doc(alias = "OUT_LINK2")]
pub type OutLink2 = crate::Reg<out_link2::OutLink2Spec>;
#[doc = "Link descriptor configure and control register of Tx channel0"]
pub mod out_link2;
#[doc = "OUT_STATE (r) register accessor: Transmit status of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state`] module"]
#[doc(alias = "OUT_STATE")]
pub type OutState = crate::Reg<out_state::OutStateSpec>;
#[doc = "Transmit status of Tx channel0"]
pub mod out_state;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: Outlink descriptor address when EOF occurs of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
#[doc(alias = "OUT_EOF_DES_ADDR")]
pub type OutEofDesAddr = crate::Reg<out_eof_des_addr::OutEofDesAddrSpec>;
#[doc = "Outlink descriptor address when EOF occurs of Tx channel0"]
pub mod out_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: The last outlink descriptor address when EOF occurs of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
#[doc(alias = "OUT_EOF_BFR_DES_ADDR")]
pub type OutEofBfrDesAddr = crate::Reg<out_eof_bfr_des_addr::OutEofBfrDesAddrSpec>;
#[doc = "The last outlink descriptor address when EOF occurs of Tx channel0"]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_DSCR (r) register accessor: Current outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr`] module"]
#[doc(alias = "OUT_DSCR")]
pub type OutDscr = crate::Reg<out_dscr::OutDscrSpec>;
#[doc = "Current outlink descriptor address of Tx channel0"]
pub mod out_dscr;
#[doc = "OUT_DSCR_BF0 (r) register accessor: The last outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0`] module"]
#[doc(alias = "OUT_DSCR_BF0")]
pub type OutDscrBf0 = crate::Reg<out_dscr_bf0::OutDscrBf0Spec>;
#[doc = "The last outlink descriptor address of Tx channel0"]
pub mod out_dscr_bf0;
#[doc = "OUT_DSCR_BF1 (r) register accessor: The second-to-last outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1`] module"]
#[doc(alias = "OUT_DSCR_BF1")]
pub type OutDscrBf1 = crate::Reg<out_dscr_bf1::OutDscrBf1Spec>;
#[doc = "The second-to-last outlink descriptor address of Tx channel0"]
pub mod out_dscr_bf1;
#[doc = "OUT_PRI (rw) register accessor: Priority register of Tx channel0.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_pri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_pri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri`] module"]
#[doc(alias = "OUT_PRI")]
pub type OutPri = crate::Reg<out_pri::OutPriSpec>;
#[doc = "Priority register of Tx channel0."]
pub mod out_pri;
#[doc = "OUT_PERI_SEL (rw) register accessor: Peripheral selection of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel`] module"]
#[doc(alias = "OUT_PERI_SEL")]
pub type OutPeriSel = crate::Reg<out_peri_sel::OutPeriSelSpec>;
#[doc = "Peripheral selection of Tx channel0"]
pub mod out_peri_sel;
#[doc = "Cluster CRC, containing OUT_CRC_INIT_DATA, TX_CRC_WIDTH, OUT_CRC_CLEAR, OUT_CRC_FINAL_RESULT, TX_CRC_EN_WR_DATA, TX_CRC_EN_ADDR, TX_CRC_DATA_EN_WR_DATA, TX_CRC_DATA_EN_ADDR"]
pub use self::crc::Crc;
#[doc = r"Cluster"]
#[doc = "Cluster CRC, containing OUT_CRC_INIT_DATA, TX_CRC_WIDTH, OUT_CRC_CLEAR, OUT_CRC_FINAL_RESULT, TX_CRC_EN_WR_DATA, TX_CRC_EN_ADDR, TX_CRC_DATA_EN_WR_DATA, TX_CRC_DATA_EN_ADDR"]
pub mod crc;
