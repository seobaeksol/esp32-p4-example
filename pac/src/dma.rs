#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id0: Id0,
    _reserved1: [u8; 0x04],
    compver0: Compver0,
    _reserved2: [u8; 0x04],
    cfg0: Cfg0,
    _reserved3: [u8; 0x04],
    chen0: Chen0,
    chen1: Chen1,
    _reserved5: [u8; 0x10],
    intstatus0: Intstatus0,
    _reserved6: [u8; 0x04],
    commonreg_intclear0: CommonregIntclear0,
    _reserved7: [u8; 0x04],
    commonreg_intstatus_enable0: CommonregIntstatusEnable0,
    _reserved8: [u8; 0x04],
    commonreg_intsignal_enable0: CommonregIntsignalEnable0,
    _reserved9: [u8; 0x04],
    commonreg_intstatus0: CommonregIntstatus0,
    _reserved10: [u8; 0x04],
    reset0: Reset0,
    _reserved11: [u8; 0x04],
    lowpower_cfg0: LowpowerCfg0,
    lowpower_cfg1: LowpowerCfg1,
    _reserved13: [u8; 0x98],
    ch: (),
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn id0(&self) -> &Id0 {
        &self.id0
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn compver0(&self) -> &Compver0 {
        &self.compver0
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn chen0(&self) -> &Chen0 {
        &self.chen0
    }
    #[doc = "0x1c - NA"]
    #[inline(always)]
    pub const fn chen1(&self) -> &Chen1 {
        &self.chen1
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn intstatus0(&self) -> &Intstatus0 {
        &self.intstatus0
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn commonreg_intclear0(&self) -> &CommonregIntclear0 {
        &self.commonreg_intclear0
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn commonreg_intstatus_enable0(&self) -> &CommonregIntstatusEnable0 {
        &self.commonreg_intstatus_enable0
    }
    #[doc = "0x48 - NA"]
    #[inline(always)]
    pub const fn commonreg_intsignal_enable0(&self) -> &CommonregIntsignalEnable0 {
        &self.commonreg_intsignal_enable0
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn commonreg_intstatus0(&self) -> &CommonregIntstatus0 {
        &self.commonreg_intstatus0
    }
    #[doc = "0x58 - NA"]
    #[inline(always)]
    pub const fn reset0(&self) -> &Reset0 {
        &self.reset0
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn lowpower_cfg0(&self) -> &LowpowerCfg0 {
        &self.lowpower_cfg0
    }
    #[doc = "0x64 - NA"]
    #[inline(always)]
    pub const fn lowpower_cfg1(&self) -> &LowpowerCfg1 {
        &self.lowpower_cfg1
    }
    #[doc = "0x100..0x380 - Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of cluster in the array. `n == 0` corresponds to `CH1` cluster.</div>"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x380 - Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x100..0x1a0 - Cluster CH1, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch1(&self) -> &Ch {
        self.ch(0)
    }
    #[doc = "0x200..0x2a0 - Cluster CH2, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch2(&self) -> &Ch {
        self.ch(1)
    }
    #[doc = "0x300..0x3a0 - Cluster CH3, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch3(&self) -> &Ch {
        self.ch(2)
    }
    #[doc = "0x400..0x4a0 - Cluster CH4, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch4(&self) -> &Ch {
        self.ch(3)
    }
}
#[doc = "ID0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0`] module"]
#[doc(alias = "ID0")]
pub type Id0 = crate::Reg<id0::Id0Spec>;
#[doc = "NA"]
pub mod id0;
#[doc = "COMPVER0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`compver0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compver0`] module"]
#[doc(alias = "COMPVER0")]
pub type Compver0 = crate::Reg<compver0::Compver0Spec>;
#[doc = "NA"]
pub mod compver0;
#[doc = "CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "NA"]
pub mod cfg0;
#[doc = "CHEN0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`chen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen0`] module"]
#[doc(alias = "CHEN0")]
pub type Chen0 = crate::Reg<chen0::Chen0Spec>;
#[doc = "NA"]
pub mod chen0;
#[doc = "CHEN1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`chen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen1`] module"]
#[doc(alias = "CHEN1")]
pub type Chen1 = crate::Reg<chen1::Chen1Spec>;
#[doc = "NA"]
pub mod chen1;
#[doc = "INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus0`] module"]
#[doc(alias = "INTSTATUS0")]
pub type Intstatus0 = crate::Reg<intstatus0::Intstatus0Spec>;
#[doc = "NA"]
pub mod intstatus0;
#[doc = "COMMONREG_INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intclear0`] module"]
#[doc(alias = "COMMONREG_INTCLEAR0")]
pub type CommonregIntclear0 = crate::Reg<commonreg_intclear0::CommonregIntclear0Spec>;
#[doc = "NA"]
pub mod commonreg_intclear0;
#[doc = "COMMONREG_INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intstatus_enable0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intstatus_enable0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intstatus_enable0`] module"]
#[doc(alias = "COMMONREG_INTSTATUS_ENABLE0")]
pub type CommonregIntstatusEnable0 =
    crate::Reg<commonreg_intstatus_enable0::CommonregIntstatusEnable0Spec>;
#[doc = "NA"]
pub mod commonreg_intstatus_enable0;
#[doc = "COMMONREG_INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intsignal_enable0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intsignal_enable0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intsignal_enable0`] module"]
#[doc(alias = "COMMONREG_INTSIGNAL_ENABLE0")]
pub type CommonregIntsignalEnable0 =
    crate::Reg<commonreg_intsignal_enable0::CommonregIntsignalEnable0Spec>;
#[doc = "NA"]
pub mod commonreg_intsignal_enable0;
#[doc = "COMMONREG_INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intstatus0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intstatus0`] module"]
#[doc(alias = "COMMONREG_INTSTATUS0")]
pub type CommonregIntstatus0 = crate::Reg<commonreg_intstatus0::CommonregIntstatus0Spec>;
#[doc = "NA"]
pub mod commonreg_intstatus0;
#[doc = "RESET0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`reset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset0`] module"]
#[doc(alias = "RESET0")]
pub type Reset0 = crate::Reg<reset0::Reset0Spec>;
#[doc = "NA"]
pub mod reset0;
#[doc = "LOWPOWER_CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower_cfg0`] module"]
#[doc(alias = "LOWPOWER_CFG0")]
pub type LowpowerCfg0 = crate::Reg<lowpower_cfg0::LowpowerCfg0Spec>;
#[doc = "NA"]
pub mod lowpower_cfg0;
#[doc = "LOWPOWER_CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower_cfg1`] module"]
#[doc(alias = "LOWPOWER_CFG1")]
pub type LowpowerCfg1 = crate::Reg<lowpower_cfg1::LowpowerCfg1Spec>;
#[doc = "NA"]
pub mod lowpower_cfg1;
#[doc = "Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
pub mod ch;
