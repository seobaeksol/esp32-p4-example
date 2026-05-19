#[repr(C)]
#[doc = "Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
#[doc(alias = "CH")]
pub struct Ch {
    sar0: Sar0,
    sar1: Sar1,
    dar0: Dar0,
    dar1: Dar1,
    block_ts0: BlockTs0,
    _reserved5: [u8; 0x04],
    ctl0: Ctl0,
    ctl1: Ctl1,
    cfg0: Cfg0,
    cfg1: Cfg1,
    llp0: Llp0,
    llp1: Llp1,
    status0: Status0,
    status1: Status1,
    swhssrc0: Swhssrc0,
    _reserved14: [u8; 0x04],
    swhsdst0: Swhsdst0,
    _reserved15: [u8; 0x04],
    blk_tfr_resumereq0: BlkTfrResumereq0,
    _reserved16: [u8; 0x04],
    axi_id0: AxiId0,
    _reserved17: [u8; 0x04],
    axi_qos0: AxiQos0,
    _reserved18: [u8; 0x04],
    sstat0: Sstat0,
    _reserved19: [u8; 0x04],
    dstat0: Dstat0,
    _reserved20: [u8; 0x04],
    sstatar0: Sstatar0,
    sstatar1: Sstatar1,
    dstatar0: Dstatar0,
    dstatar1: Dstatar1,
    intstatus_enable0: IntstatusEnable0,
    intstatus_enable1: IntstatusEnable1,
    intstatus0: Intstatus0,
    intstatus1: Intstatus1,
    intsignal_enable0: IntsignalEnable0,
    intsignal_enable1: IntsignalEnable1,
    intclear0: Intclear0,
    intclear1: Intclear1,
}
impl Ch {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn sar0(&self) -> &Sar0 {
        &self.sar0
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn sar1(&self) -> &Sar1 {
        &self.sar1
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn dar0(&self) -> &Dar0 {
        &self.dar0
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn dar1(&self) -> &Dar1 {
        &self.dar1
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn block_ts0(&self) -> &BlockTs0 {
        &self.block_ts0
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x1c - NA"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x20 - NA"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x24 - NA"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x28 - NA"]
    #[inline(always)]
    pub const fn llp0(&self) -> &Llp0 {
        &self.llp0
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn llp1(&self) -> &Llp1 {
        &self.llp1
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn status0(&self) -> &Status0 {
        &self.status0
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn status1(&self) -> &Status1 {
        &self.status1
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn swhssrc0(&self) -> &Swhssrc0 {
        &self.swhssrc0
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn swhsdst0(&self) -> &Swhsdst0 {
        &self.swhsdst0
    }
    #[doc = "0x48 - NA"]
    #[inline(always)]
    pub const fn blk_tfr_resumereq0(&self) -> &BlkTfrResumereq0 {
        &self.blk_tfr_resumereq0
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn axi_id0(&self) -> &AxiId0 {
        &self.axi_id0
    }
    #[doc = "0x58 - NA"]
    #[inline(always)]
    pub const fn axi_qos0(&self) -> &AxiQos0 {
        &self.axi_qos0
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn sstat0(&self) -> &Sstat0 {
        &self.sstat0
    }
    #[doc = "0x68 - NA"]
    #[inline(always)]
    pub const fn dstat0(&self) -> &Dstat0 {
        &self.dstat0
    }
    #[doc = "0x70 - NA"]
    #[inline(always)]
    pub const fn sstatar0(&self) -> &Sstatar0 {
        &self.sstatar0
    }
    #[doc = "0x74 - NA"]
    #[inline(always)]
    pub const fn sstatar1(&self) -> &Sstatar1 {
        &self.sstatar1
    }
    #[doc = "0x78 - NA"]
    #[inline(always)]
    pub const fn dstatar0(&self) -> &Dstatar0 {
        &self.dstatar0
    }
    #[doc = "0x7c - NA"]
    #[inline(always)]
    pub const fn dstatar1(&self) -> &Dstatar1 {
        &self.dstatar1
    }
    #[doc = "0x80 - NA"]
    #[inline(always)]
    pub const fn intstatus_enable0(&self) -> &IntstatusEnable0 {
        &self.intstatus_enable0
    }
    #[doc = "0x84 - NA"]
    #[inline(always)]
    pub const fn intstatus_enable1(&self) -> &IntstatusEnable1 {
        &self.intstatus_enable1
    }
    #[doc = "0x88 - NA"]
    #[inline(always)]
    pub const fn intstatus0(&self) -> &Intstatus0 {
        &self.intstatus0
    }
    #[doc = "0x8c - NA"]
    #[inline(always)]
    pub const fn intstatus1(&self) -> &Intstatus1 {
        &self.intstatus1
    }
    #[doc = "0x90 - NA"]
    #[inline(always)]
    pub const fn intsignal_enable0(&self) -> &IntsignalEnable0 {
        &self.intsignal_enable0
    }
    #[doc = "0x94 - NA"]
    #[inline(always)]
    pub const fn intsignal_enable1(&self) -> &IntsignalEnable1 {
        &self.intsignal_enable1
    }
    #[doc = "0x98 - NA"]
    #[inline(always)]
    pub const fn intclear0(&self) -> &Intclear0 {
        &self.intclear0
    }
    #[doc = "0x9c - NA"]
    #[inline(always)]
    pub const fn intclear1(&self) -> &Intclear1 {
        &self.intclear1
    }
}
#[doc = "SAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar0`] module"]
#[doc(alias = "SAR0")]
pub type Sar0 = crate::Reg<sar0::Sar0Spec>;
#[doc = "NA"]
pub mod sar0;
#[doc = "SAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1`] module"]
#[doc(alias = "SAR1")]
pub type Sar1 = crate::Reg<sar1::Sar1Spec>;
#[doc = "NA"]
pub mod sar1;
#[doc = "DAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar0`] module"]
#[doc(alias = "DAR0")]
pub type Dar0 = crate::Reg<dar0::Dar0Spec>;
#[doc = "NA"]
pub mod dar0;
#[doc = "DAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar1`] module"]
#[doc(alias = "DAR1")]
pub type Dar1 = crate::Reg<dar1::Dar1Spec>;
#[doc = "NA"]
pub mod dar1;
#[doc = "BLOCK_TS0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`block_ts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_ts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_ts0`] module"]
#[doc(alias = "BLOCK_TS0")]
pub type BlockTs0 = crate::Reg<block_ts0::BlockTs0Spec>;
#[doc = "NA"]
pub mod block_ts0;
#[doc = "CTL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`] module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "NA"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "NA"]
pub mod ctl1;
#[doc = "CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "NA"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "NA"]
pub mod cfg1;
#[doc = "LLP0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`llp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp0`] module"]
#[doc(alias = "LLP0")]
pub type Llp0 = crate::Reg<llp0::Llp0Spec>;
#[doc = "NA"]
pub mod llp0;
#[doc = "LLP1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`llp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp1`] module"]
#[doc(alias = "LLP1")]
pub type Llp1 = crate::Reg<llp1::Llp1Spec>;
#[doc = "NA"]
pub mod llp1;
#[doc = "STATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status0`] module"]
#[doc(alias = "STATUS0")]
pub type Status0 = crate::Reg<status0::Status0Spec>;
#[doc = "NA"]
pub mod status0;
#[doc = "STATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`] module"]
#[doc(alias = "STATUS1")]
pub type Status1 = crate::Reg<status1::Status1Spec>;
#[doc = "NA"]
pub mod status1;
#[doc = "SWHSSRC0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`swhssrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swhssrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swhssrc0`] module"]
#[doc(alias = "SWHSSRC0")]
pub type Swhssrc0 = crate::Reg<swhssrc0::Swhssrc0Spec>;
#[doc = "NA"]
pub mod swhssrc0;
#[doc = "SWHSDST0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`swhsdst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swhsdst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swhsdst0`] module"]
#[doc(alias = "SWHSDST0")]
pub type Swhsdst0 = crate::Reg<swhsdst0::Swhsdst0Spec>;
#[doc = "NA"]
pub mod swhsdst0;
#[doc = "BLK_TFR_RESUMEREQ0 (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_tfr_resumereq0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_tfr_resumereq0`] module"]
#[doc(alias = "BLK_TFR_RESUMEREQ0")]
pub type BlkTfrResumereq0 = crate::Reg<blk_tfr_resumereq0::BlkTfrResumereq0Spec>;
#[doc = "NA"]
pub mod blk_tfr_resumereq0;
#[doc = "AXI_ID0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_id0`] module"]
#[doc(alias = "AXI_ID0")]
pub type AxiId0 = crate::Reg<axi_id0::AxiId0Spec>;
#[doc = "NA"]
pub mod axi_id0;
#[doc = "AXI_QOS0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_qos0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_qos0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_qos0`] module"]
#[doc(alias = "AXI_QOS0")]
pub type AxiQos0 = crate::Reg<axi_qos0::AxiQos0Spec>;
#[doc = "NA"]
pub mod axi_qos0;
#[doc = "SSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sstat0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstat0`] module"]
#[doc(alias = "SSTAT0")]
pub type Sstat0 = crate::Reg<sstat0::Sstat0Spec>;
#[doc = "NA"]
pub mod sstat0;
#[doc = "DSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstat0`] module"]
#[doc(alias = "DSTAT0")]
pub type Dstat0 = crate::Reg<dstat0::Dstat0Spec>;
#[doc = "NA"]
pub mod dstat0;
#[doc = "SSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatar0`] module"]
#[doc(alias = "SSTATAR0")]
pub type Sstatar0 = crate::Reg<sstatar0::Sstatar0Spec>;
#[doc = "NA"]
pub mod sstatar0;
#[doc = "SSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatar1`] module"]
#[doc(alias = "SSTATAR1")]
pub type Sstatar1 = crate::Reg<sstatar1::Sstatar1Spec>;
#[doc = "NA"]
pub mod sstatar1;
#[doc = "DSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstatar0`] module"]
#[doc(alias = "DSTATAR0")]
pub type Dstatar0 = crate::Reg<dstatar0::Dstatar0Spec>;
#[doc = "NA"]
pub mod dstatar0;
#[doc = "DSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstatar1`] module"]
#[doc(alias = "DSTATAR1")]
pub type Dstatar1 = crate::Reg<dstatar1::Dstatar1Spec>;
#[doc = "NA"]
pub mod dstatar1;
#[doc = "INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus_enable0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatus_enable0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus_enable0`] module"]
#[doc(alias = "INTSTATUS_ENABLE0")]
pub type IntstatusEnable0 = crate::Reg<intstatus_enable0::IntstatusEnable0Spec>;
#[doc = "NA"]
pub mod intstatus_enable0;
#[doc = "INTSTATUS_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus_enable1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus_enable1`] module"]
#[doc(alias = "INTSTATUS_ENABLE1")]
pub type IntstatusEnable1 = crate::Reg<intstatus_enable1::IntstatusEnable1Spec>;
#[doc = "NA"]
pub mod intstatus_enable1;
#[doc = "INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus0`] module"]
#[doc(alias = "INTSTATUS0")]
pub type Intstatus0 = crate::Reg<intstatus0::Intstatus0Spec>;
#[doc = "NA"]
pub mod intstatus0;
#[doc = "INTSTATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus1`] module"]
#[doc(alias = "INTSTATUS1")]
pub type Intstatus1 = crate::Reg<intstatus1::Intstatus1Spec>;
#[doc = "NA"]
pub mod intstatus1;
#[doc = "INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intsignal_enable0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsignal_enable0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsignal_enable0`] module"]
#[doc(alias = "INTSIGNAL_ENABLE0")]
pub type IntsignalEnable0 = crate::Reg<intsignal_enable0::IntsignalEnable0Spec>;
#[doc = "NA"]
pub mod intsignal_enable0;
#[doc = "INTSIGNAL_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intsignal_enable1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsignal_enable1`] module"]
#[doc(alias = "INTSIGNAL_ENABLE1")]
pub type IntsignalEnable1 = crate::Reg<intsignal_enable1::IntsignalEnable1Spec>;
#[doc = "NA"]
pub mod intsignal_enable1;
#[doc = "INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear0`] module"]
#[doc(alias = "INTCLEAR0")]
pub type Intclear0 = crate::Reg<intclear0::Intclear0Spec>;
#[doc = "NA"]
pub mod intclear0;
#[doc = "INTCLEAR1 (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear1`] module"]
#[doc(alias = "INTCLEAR1")]
pub type Intclear1 = crate::Reg<intclear1::Intclear1Spec>;
#[doc = "NA"]
pub mod intclear1;
