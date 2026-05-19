#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    version: Version,
    n_lanes: NLanes,
    csi2_resetn: Csi2Resetn,
    int_st_main: IntStMain,
    _reserved4: [u8; 0x30],
    phy_shutdownz: PhyShutdownz,
    dphy_rstz: DphyRstz,
    phy_rx: PhyRx,
    phy_stopstate: PhyStopstate,
    phy_test_ctrl0: PhyTestCtrl0,
    phy_test_ctrl1: PhyTestCtrl1,
    _reserved10: [u8; 0x70],
    vc_extension: VcExtension,
    phy_cal: PhyCal,
    _reserved12: [u8; 0x10],
    int_st_phy_fatal: IntStPhyFatal,
    int_msk_phy_fatal: IntMskPhyFatal,
    int_force_phy_fatal: IntForcePhyFatal,
    _reserved15: [u8; 0x04],
    int_st_pkt_fatal: IntStPktFatal,
    int_msk_pkt_fatal: IntMskPktFatal,
    int_force_pkt_fatal: IntForcePktFatal,
    _reserved18: [u8; 0x14],
    int_st_phy: IntStPhy,
    int_msk_phy: IntMskPhy,
    int_force_phy: IntForcePhy,
    _reserved21: [u8; 0x0164],
    int_st_bndry_frame_fatal: IntStBndryFrameFatal,
    int_msk_bndry_frame_fatal: IntMskBndryFrameFatal,
    int_force_bndry_frame_fatal: IntForceBndryFrameFatal,
    _reserved24: [u8; 0x04],
    int_st_seq_frame_fatal: IntStSeqFrameFatal,
    int_msk_seq_frame_fatal: IntMskSeqFrameFatal,
    int_force_seq_frame_fatal: IntForceSeqFrameFatal,
    _reserved27: [u8; 0x04],
    int_st_crc_frame_fatal: IntStCrcFrameFatal,
    int_msk_crc_frame_fatal: IntMskCrcFrameFatal,
    int_force_crc_frame_fatal: IntForceCrcFrameFatal,
    _reserved30: [u8; 0x04],
    int_st_pld_crc_fatal: IntStPldCrcFatal,
    int_msk_pld_crc_fatal: IntMskPldCrcFatal,
    int_force_pld_crc_fatal: IntForcePldCrcFatal,
    _reserved33: [u8; 0x04],
    int_st_data_id: IntStDataId,
    int_msk_data_id: IntMskDataId,
    int_force_data_id: IntForceDataId,
    _reserved36: [u8; 0x04],
    int_st_ecc_corrected: IntStEccCorrected,
    int_msk_ecc_corrected: IntMskEccCorrected,
    int_force_ecc_corrected: IntForceEccCorrected,
    _reserved39: [u8; 0x24],
    scrambling: Scrambling,
    scrambling_seed1: ScramblingSeed1,
    scrambling_seed2: ScramblingSeed2,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn n_lanes(&self) -> &NLanes {
        &self.n_lanes
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn csi2_resetn(&self) -> &Csi2Resetn {
        &self.csi2_resetn
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn int_st_main(&self) -> &IntStMain {
        &self.int_st_main
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn phy_shutdownz(&self) -> &PhyShutdownz {
        &self.phy_shutdownz
    }
    #[doc = "0x44 - NA"]
    #[inline(always)]
    pub const fn dphy_rstz(&self) -> &DphyRstz {
        &self.dphy_rstz
    }
    #[doc = "0x48 - NA"]
    #[inline(always)]
    pub const fn phy_rx(&self) -> &PhyRx {
        &self.phy_rx
    }
    #[doc = "0x4c - NA"]
    #[inline(always)]
    pub const fn phy_stopstate(&self) -> &PhyStopstate {
        &self.phy_stopstate
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn phy_test_ctrl0(&self) -> &PhyTestCtrl0 {
        &self.phy_test_ctrl0
    }
    #[doc = "0x54 - NA"]
    #[inline(always)]
    pub const fn phy_test_ctrl1(&self) -> &PhyTestCtrl1 {
        &self.phy_test_ctrl1
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn vc_extension(&self) -> &VcExtension {
        &self.vc_extension
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn phy_cal(&self) -> &PhyCal {
        &self.phy_cal
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn int_st_phy_fatal(&self) -> &IntStPhyFatal {
        &self.int_st_phy_fatal
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn int_msk_phy_fatal(&self) -> &IntMskPhyFatal {
        &self.int_msk_phy_fatal
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn int_force_phy_fatal(&self) -> &IntForcePhyFatal {
        &self.int_force_phy_fatal
    }
    #[doc = "0xf0 - NA"]
    #[inline(always)]
    pub const fn int_st_pkt_fatal(&self) -> &IntStPktFatal {
        &self.int_st_pkt_fatal
    }
    #[doc = "0xf4 - NA"]
    #[inline(always)]
    pub const fn int_msk_pkt_fatal(&self) -> &IntMskPktFatal {
        &self.int_msk_pkt_fatal
    }
    #[doc = "0xf8 - NA"]
    #[inline(always)]
    pub const fn int_force_pkt_fatal(&self) -> &IntForcePktFatal {
        &self.int_force_pkt_fatal
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn int_st_phy(&self) -> &IntStPhy {
        &self.int_st_phy
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn int_msk_phy(&self) -> &IntMskPhy {
        &self.int_msk_phy
    }
    #[doc = "0x118 - NA"]
    #[inline(always)]
    pub const fn int_force_phy(&self) -> &IntForcePhy {
        &self.int_force_phy
    }
    #[doc = "0x280 - NA"]
    #[inline(always)]
    pub const fn int_st_bndry_frame_fatal(&self) -> &IntStBndryFrameFatal {
        &self.int_st_bndry_frame_fatal
    }
    #[doc = "0x284 - NA"]
    #[inline(always)]
    pub const fn int_msk_bndry_frame_fatal(&self) -> &IntMskBndryFrameFatal {
        &self.int_msk_bndry_frame_fatal
    }
    #[doc = "0x288 - NA"]
    #[inline(always)]
    pub const fn int_force_bndry_frame_fatal(&self) -> &IntForceBndryFrameFatal {
        &self.int_force_bndry_frame_fatal
    }
    #[doc = "0x290 - NA"]
    #[inline(always)]
    pub const fn int_st_seq_frame_fatal(&self) -> &IntStSeqFrameFatal {
        &self.int_st_seq_frame_fatal
    }
    #[doc = "0x294 - NA"]
    #[inline(always)]
    pub const fn int_msk_seq_frame_fatal(&self) -> &IntMskSeqFrameFatal {
        &self.int_msk_seq_frame_fatal
    }
    #[doc = "0x298 - NA"]
    #[inline(always)]
    pub const fn int_force_seq_frame_fatal(&self) -> &IntForceSeqFrameFatal {
        &self.int_force_seq_frame_fatal
    }
    #[doc = "0x2a0 - NA"]
    #[inline(always)]
    pub const fn int_st_crc_frame_fatal(&self) -> &IntStCrcFrameFatal {
        &self.int_st_crc_frame_fatal
    }
    #[doc = "0x2a4 - NA"]
    #[inline(always)]
    pub const fn int_msk_crc_frame_fatal(&self) -> &IntMskCrcFrameFatal {
        &self.int_msk_crc_frame_fatal
    }
    #[doc = "0x2a8 - NA"]
    #[inline(always)]
    pub const fn int_force_crc_frame_fatal(&self) -> &IntForceCrcFrameFatal {
        &self.int_force_crc_frame_fatal
    }
    #[doc = "0x2b0 - NA"]
    #[inline(always)]
    pub const fn int_st_pld_crc_fatal(&self) -> &IntStPldCrcFatal {
        &self.int_st_pld_crc_fatal
    }
    #[doc = "0x2b4 - NA"]
    #[inline(always)]
    pub const fn int_msk_pld_crc_fatal(&self) -> &IntMskPldCrcFatal {
        &self.int_msk_pld_crc_fatal
    }
    #[doc = "0x2b8 - NA"]
    #[inline(always)]
    pub const fn int_force_pld_crc_fatal(&self) -> &IntForcePldCrcFatal {
        &self.int_force_pld_crc_fatal
    }
    #[doc = "0x2c0 - NA"]
    #[inline(always)]
    pub const fn int_st_data_id(&self) -> &IntStDataId {
        &self.int_st_data_id
    }
    #[doc = "0x2c4 - NA"]
    #[inline(always)]
    pub const fn int_msk_data_id(&self) -> &IntMskDataId {
        &self.int_msk_data_id
    }
    #[doc = "0x2c8 - NA"]
    #[inline(always)]
    pub const fn int_force_data_id(&self) -> &IntForceDataId {
        &self.int_force_data_id
    }
    #[doc = "0x2d0 - NA"]
    #[inline(always)]
    pub const fn int_st_ecc_corrected(&self) -> &IntStEccCorrected {
        &self.int_st_ecc_corrected
    }
    #[doc = "0x2d4 - NA"]
    #[inline(always)]
    pub const fn int_msk_ecc_corrected(&self) -> &IntMskEccCorrected {
        &self.int_msk_ecc_corrected
    }
    #[doc = "0x2d8 - NA"]
    #[inline(always)]
    pub const fn int_force_ecc_corrected(&self) -> &IntForceEccCorrected {
        &self.int_force_ecc_corrected
    }
    #[doc = "0x300 - NA"]
    #[inline(always)]
    pub const fn scrambling(&self) -> &Scrambling {
        &self.scrambling
    }
    #[doc = "0x304 - NA"]
    #[inline(always)]
    pub const fn scrambling_seed1(&self) -> &ScramblingSeed1 {
        &self.scrambling_seed1
    }
    #[doc = "0x308 - NA"]
    #[inline(always)]
    pub const fn scrambling_seed2(&self) -> &ScramblingSeed2 {
        &self.scrambling_seed2
    }
}
#[doc = "VERSION (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "NA"]
pub mod version;
#[doc = "N_LANES (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`n_lanes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_lanes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_lanes`] module"]
#[doc(alias = "N_LANES")]
pub type NLanes = crate::Reg<n_lanes::NLanesSpec>;
#[doc = "NA"]
pub mod n_lanes;
#[doc = "CSI2_RESETN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_resetn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_resetn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_resetn`] module"]
#[doc(alias = "CSI2_RESETN")]
pub type Csi2Resetn = crate::Reg<csi2_resetn::Csi2ResetnSpec>;
#[doc = "NA"]
pub mod csi2_resetn;
#[doc = "INT_ST_MAIN (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_main::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_main`] module"]
#[doc(alias = "INT_ST_MAIN")]
pub type IntStMain = crate::Reg<int_st_main::IntStMainSpec>;
#[doc = "NA"]
pub mod int_st_main;
#[doc = "PHY_SHUTDOWNZ (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_shutdownz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_shutdownz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_shutdownz`] module"]
#[doc(alias = "PHY_SHUTDOWNZ")]
pub type PhyShutdownz = crate::Reg<phy_shutdownz::PhyShutdownzSpec>;
#[doc = "NA"]
pub mod phy_shutdownz;
#[doc = "DPHY_RSTZ (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_rstz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_rstz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_rstz`] module"]
#[doc(alias = "DPHY_RSTZ")]
pub type DphyRstz = crate::Reg<dphy_rstz::DphyRstzSpec>;
#[doc = "NA"]
pub mod dphy_rstz;
#[doc = "PHY_RX (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_rx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_rx`] module"]
#[doc(alias = "PHY_RX")]
pub type PhyRx = crate::Reg<phy_rx::PhyRxSpec>;
#[doc = "NA"]
pub mod phy_rx;
#[doc = "PHY_STOPSTATE (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_stopstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_stopstate`] module"]
#[doc(alias = "PHY_STOPSTATE")]
pub type PhyStopstate = crate::Reg<phy_stopstate::PhyStopstateSpec>;
#[doc = "NA"]
pub mod phy_stopstate;
#[doc = "PHY_TEST_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_test_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_test_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_test_ctrl0`] module"]
#[doc(alias = "PHY_TEST_CTRL0")]
pub type PhyTestCtrl0 = crate::Reg<phy_test_ctrl0::PhyTestCtrl0Spec>;
#[doc = "NA"]
pub mod phy_test_ctrl0;
#[doc = "PHY_TEST_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_test_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_test_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_test_ctrl1`] module"]
#[doc(alias = "PHY_TEST_CTRL1")]
pub type PhyTestCtrl1 = crate::Reg<phy_test_ctrl1::PhyTestCtrl1Spec>;
#[doc = "NA"]
pub mod phy_test_ctrl1;
#[doc = "VC_EXTENSION (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vc_extension::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc_extension::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vc_extension`] module"]
#[doc(alias = "VC_EXTENSION")]
pub type VcExtension = crate::Reg<vc_extension::VcExtensionSpec>;
#[doc = "NA"]
pub mod vc_extension;
#[doc = "PHY_CAL (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cal`] module"]
#[doc(alias = "PHY_CAL")]
pub type PhyCal = crate::Reg<phy_cal::PhyCalSpec>;
#[doc = "NA"]
pub mod phy_cal;
#[doc = "INT_ST_PHY_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_phy_fatal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_phy_fatal`] module"]
#[doc(alias = "INT_ST_PHY_FATAL")]
pub type IntStPhyFatal = crate::Reg<int_st_phy_fatal::IntStPhyFatalSpec>;
#[doc = "NA"]
pub mod int_st_phy_fatal;
#[doc = "INT_MSK_PHY_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_phy_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_phy_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_phy_fatal`] module"]
#[doc(alias = "INT_MSK_PHY_FATAL")]
pub type IntMskPhyFatal = crate::Reg<int_msk_phy_fatal::IntMskPhyFatalSpec>;
#[doc = "NA"]
pub mod int_msk_phy_fatal;
#[doc = "INT_FORCE_PHY_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_phy_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_phy_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_phy_fatal`] module"]
#[doc(alias = "INT_FORCE_PHY_FATAL")]
pub type IntForcePhyFatal = crate::Reg<int_force_phy_fatal::IntForcePhyFatalSpec>;
#[doc = "NA"]
pub mod int_force_phy_fatal;
#[doc = "INT_ST_PKT_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_pkt_fatal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_pkt_fatal`] module"]
#[doc(alias = "INT_ST_PKT_FATAL")]
pub type IntStPktFatal = crate::Reg<int_st_pkt_fatal::IntStPktFatalSpec>;
#[doc = "NA"]
pub mod int_st_pkt_fatal;
#[doc = "INT_MSK_PKT_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_pkt_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_pkt_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_pkt_fatal`] module"]
#[doc(alias = "INT_MSK_PKT_FATAL")]
pub type IntMskPktFatal = crate::Reg<int_msk_pkt_fatal::IntMskPktFatalSpec>;
#[doc = "NA"]
pub mod int_msk_pkt_fatal;
#[doc = "INT_FORCE_PKT_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_pkt_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_pkt_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_pkt_fatal`] module"]
#[doc(alias = "INT_FORCE_PKT_FATAL")]
pub type IntForcePktFatal = crate::Reg<int_force_pkt_fatal::IntForcePktFatalSpec>;
#[doc = "NA"]
pub mod int_force_pkt_fatal;
#[doc = "INT_ST_PHY (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_phy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_phy`] module"]
#[doc(alias = "INT_ST_PHY")]
pub type IntStPhy = crate::Reg<int_st_phy::IntStPhySpec>;
#[doc = "NA"]
pub mod int_st_phy;
#[doc = "INT_MSK_PHY (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_phy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_phy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_phy`] module"]
#[doc(alias = "INT_MSK_PHY")]
pub type IntMskPhy = crate::Reg<int_msk_phy::IntMskPhySpec>;
#[doc = "NA"]
pub mod int_msk_phy;
#[doc = "INT_FORCE_PHY (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_phy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_phy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_phy`] module"]
#[doc(alias = "INT_FORCE_PHY")]
pub type IntForcePhy = crate::Reg<int_force_phy::IntForcePhySpec>;
#[doc = "NA"]
pub mod int_force_phy;
#[doc = "INT_ST_BNDRY_FRAME_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_bndry_frame_fatal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_bndry_frame_fatal`] module"]
#[doc(alias = "INT_ST_BNDRY_FRAME_FATAL")]
pub type IntStBndryFrameFatal = crate::Reg<int_st_bndry_frame_fatal::IntStBndryFrameFatalSpec>;
#[doc = "NA"]
pub mod int_st_bndry_frame_fatal;
#[doc = "INT_MSK_BNDRY_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_bndry_frame_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_bndry_frame_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_bndry_frame_fatal`] module"]
#[doc(alias = "INT_MSK_BNDRY_FRAME_FATAL")]
pub type IntMskBndryFrameFatal = crate::Reg<int_msk_bndry_frame_fatal::IntMskBndryFrameFatalSpec>;
#[doc = "NA"]
pub mod int_msk_bndry_frame_fatal;
#[doc = "INT_FORCE_BNDRY_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_bndry_frame_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_bndry_frame_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_bndry_frame_fatal`] module"]
#[doc(alias = "INT_FORCE_BNDRY_FRAME_FATAL")]
pub type IntForceBndryFrameFatal =
    crate::Reg<int_force_bndry_frame_fatal::IntForceBndryFrameFatalSpec>;
#[doc = "NA"]
pub mod int_force_bndry_frame_fatal;
#[doc = "INT_ST_SEQ_FRAME_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_seq_frame_fatal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_seq_frame_fatal`] module"]
#[doc(alias = "INT_ST_SEQ_FRAME_FATAL")]
pub type IntStSeqFrameFatal = crate::Reg<int_st_seq_frame_fatal::IntStSeqFrameFatalSpec>;
#[doc = "NA"]
pub mod int_st_seq_frame_fatal;
#[doc = "INT_MSK_SEQ_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_seq_frame_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_seq_frame_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_seq_frame_fatal`] module"]
#[doc(alias = "INT_MSK_SEQ_FRAME_FATAL")]
pub type IntMskSeqFrameFatal = crate::Reg<int_msk_seq_frame_fatal::IntMskSeqFrameFatalSpec>;
#[doc = "NA"]
pub mod int_msk_seq_frame_fatal;
#[doc = "INT_FORCE_SEQ_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_seq_frame_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_seq_frame_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_seq_frame_fatal`] module"]
#[doc(alias = "INT_FORCE_SEQ_FRAME_FATAL")]
pub type IntForceSeqFrameFatal = crate::Reg<int_force_seq_frame_fatal::IntForceSeqFrameFatalSpec>;
#[doc = "NA"]
pub mod int_force_seq_frame_fatal;
#[doc = "INT_ST_CRC_FRAME_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_crc_frame_fatal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_crc_frame_fatal`] module"]
#[doc(alias = "INT_ST_CRC_FRAME_FATAL")]
pub type IntStCrcFrameFatal = crate::Reg<int_st_crc_frame_fatal::IntStCrcFrameFatalSpec>;
#[doc = "NA"]
pub mod int_st_crc_frame_fatal;
#[doc = "INT_MSK_CRC_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_crc_frame_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_crc_frame_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_crc_frame_fatal`] module"]
#[doc(alias = "INT_MSK_CRC_FRAME_FATAL")]
pub type IntMskCrcFrameFatal = crate::Reg<int_msk_crc_frame_fatal::IntMskCrcFrameFatalSpec>;
#[doc = "NA"]
pub mod int_msk_crc_frame_fatal;
#[doc = "INT_FORCE_CRC_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_crc_frame_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_crc_frame_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_crc_frame_fatal`] module"]
#[doc(alias = "INT_FORCE_CRC_FRAME_FATAL")]
pub type IntForceCrcFrameFatal = crate::Reg<int_force_crc_frame_fatal::IntForceCrcFrameFatalSpec>;
#[doc = "NA"]
pub mod int_force_crc_frame_fatal;
#[doc = "INT_ST_PLD_CRC_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_pld_crc_fatal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_pld_crc_fatal`] module"]
#[doc(alias = "INT_ST_PLD_CRC_FATAL")]
pub type IntStPldCrcFatal = crate::Reg<int_st_pld_crc_fatal::IntStPldCrcFatalSpec>;
#[doc = "NA"]
pub mod int_st_pld_crc_fatal;
#[doc = "INT_MSK_PLD_CRC_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_pld_crc_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_pld_crc_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_pld_crc_fatal`] module"]
#[doc(alias = "INT_MSK_PLD_CRC_FATAL")]
pub type IntMskPldCrcFatal = crate::Reg<int_msk_pld_crc_fatal::IntMskPldCrcFatalSpec>;
#[doc = "NA"]
pub mod int_msk_pld_crc_fatal;
#[doc = "INT_FORCE_PLD_CRC_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_pld_crc_fatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_pld_crc_fatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_pld_crc_fatal`] module"]
#[doc(alias = "INT_FORCE_PLD_CRC_FATAL")]
pub type IntForcePldCrcFatal = crate::Reg<int_force_pld_crc_fatal::IntForcePldCrcFatalSpec>;
#[doc = "NA"]
pub mod int_force_pld_crc_fatal;
#[doc = "INT_ST_DATA_ID (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_data_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_data_id`] module"]
#[doc(alias = "INT_ST_DATA_ID")]
pub type IntStDataId = crate::Reg<int_st_data_id::IntStDataIdSpec>;
#[doc = "NA"]
pub mod int_st_data_id;
#[doc = "INT_MSK_DATA_ID (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_data_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_data_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_data_id`] module"]
#[doc(alias = "INT_MSK_DATA_ID")]
pub type IntMskDataId = crate::Reg<int_msk_data_id::IntMskDataIdSpec>;
#[doc = "NA"]
pub mod int_msk_data_id;
#[doc = "INT_FORCE_DATA_ID (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_data_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_data_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_data_id`] module"]
#[doc(alias = "INT_FORCE_DATA_ID")]
pub type IntForceDataId = crate::Reg<int_force_data_id::IntForceDataIdSpec>;
#[doc = "NA"]
pub mod int_force_data_id;
#[doc = "INT_ST_ECC_CORRECTED (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_ecc_corrected::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_ecc_corrected`] module"]
#[doc(alias = "INT_ST_ECC_CORRECTED")]
pub type IntStEccCorrected = crate::Reg<int_st_ecc_corrected::IntStEccCorrectedSpec>;
#[doc = "NA"]
pub mod int_st_ecc_corrected;
#[doc = "INT_MSK_ECC_CORRECTED (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_ecc_corrected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_ecc_corrected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_ecc_corrected`] module"]
#[doc(alias = "INT_MSK_ECC_CORRECTED")]
pub type IntMskEccCorrected = crate::Reg<int_msk_ecc_corrected::IntMskEccCorrectedSpec>;
#[doc = "NA"]
pub mod int_msk_ecc_corrected;
#[doc = "INT_FORCE_ECC_CORRECTED (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_ecc_corrected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_ecc_corrected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_ecc_corrected`] module"]
#[doc(alias = "INT_FORCE_ECC_CORRECTED")]
pub type IntForceEccCorrected = crate::Reg<int_force_ecc_corrected::IntForceEccCorrectedSpec>;
#[doc = "NA"]
pub mod int_force_ecc_corrected;
#[doc = "SCRAMBLING (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scrambling::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambling::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambling`] module"]
#[doc(alias = "SCRAMBLING")]
pub type Scrambling = crate::Reg<scrambling::ScramblingSpec>;
#[doc = "NA"]
pub mod scrambling;
#[doc = "SCRAMBLING_SEED1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scrambling_seed1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambling_seed1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambling_seed1`] module"]
#[doc(alias = "SCRAMBLING_SEED1")]
pub type ScramblingSeed1 = crate::Reg<scrambling_seed1::ScramblingSeed1Spec>;
#[doc = "NA"]
pub mod scrambling_seed1;
#[doc = "SCRAMBLING_SEED2 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scrambling_seed2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambling_seed2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambling_seed2`] module"]
#[doc(alias = "SCRAMBLING_SEED2")]
pub type ScramblingSeed2 = crate::Reg<scrambling_seed2::ScramblingSeed2Spec>;
#[doc = "NA"]
pub mod scrambling_seed2;
