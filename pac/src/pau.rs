#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regdma_conf: RegdmaConf,
    regdma_clk_conf: RegdmaClkConf,
    regdma_etm_ctrl: RegdmaEtmCtrl,
    regdma_link_0_addr: RegdmaLink0Addr,
    regdma_link_1_addr: RegdmaLink1Addr,
    regdma_link_2_addr: RegdmaLink2Addr,
    regdma_link_3_addr: RegdmaLink3Addr,
    regdma_link_mac_addr: RegdmaLinkMacAddr,
    regdma_current_link_addr: RegdmaCurrentLinkAddr,
    regdma_backup_addr: RegdmaBackupAddr,
    regdma_mem_addr: RegdmaMemAddr,
    regdma_bkp_conf: RegdmaBkpConf,
    int_ena: IntEna,
    int_raw: IntRaw,
    int_clr: IntClr,
    int_st: IntSt,
    _reserved16: [u8; 0x03bc],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - Peri backup control register"]
    #[inline(always)]
    pub const fn regdma_conf(&self) -> &RegdmaConf {
        &self.regdma_conf
    }
    #[doc = "0x04 - Clock control register"]
    #[inline(always)]
    pub const fn regdma_clk_conf(&self) -> &RegdmaClkConf {
        &self.regdma_clk_conf
    }
    #[doc = "0x08 - ETM start ctrl reg"]
    #[inline(always)]
    pub const fn regdma_etm_ctrl(&self) -> &RegdmaEtmCtrl {
        &self.regdma_etm_ctrl
    }
    #[doc = "0x0c - link_0_addr"]
    #[inline(always)]
    pub const fn regdma_link_0_addr(&self) -> &RegdmaLink0Addr {
        &self.regdma_link_0_addr
    }
    #[doc = "0x10 - Link_1_addr"]
    #[inline(always)]
    pub const fn regdma_link_1_addr(&self) -> &RegdmaLink1Addr {
        &self.regdma_link_1_addr
    }
    #[doc = "0x14 - Link_2_addr"]
    #[inline(always)]
    pub const fn regdma_link_2_addr(&self) -> &RegdmaLink2Addr {
        &self.regdma_link_2_addr
    }
    #[doc = "0x18 - Link_3_addr"]
    #[inline(always)]
    pub const fn regdma_link_3_addr(&self) -> &RegdmaLink3Addr {
        &self.regdma_link_3_addr
    }
    #[doc = "0x1c - Link_mac_addr"]
    #[inline(always)]
    pub const fn regdma_link_mac_addr(&self) -> &RegdmaLinkMacAddr {
        &self.regdma_link_mac_addr
    }
    #[doc = "0x20 - current link addr"]
    #[inline(always)]
    pub const fn regdma_current_link_addr(&self) -> &RegdmaCurrentLinkAddr {
        &self.regdma_current_link_addr
    }
    #[doc = "0x24 - Backup addr"]
    #[inline(always)]
    pub const fn regdma_backup_addr(&self) -> &RegdmaBackupAddr {
        &self.regdma_backup_addr
    }
    #[doc = "0x28 - mem addr"]
    #[inline(always)]
    pub const fn regdma_mem_addr(&self) -> &RegdmaMemAddr {
        &self.regdma_mem_addr
    }
    #[doc = "0x2c - backup config"]
    #[inline(always)]
    pub const fn regdma_bkp_conf(&self) -> &RegdmaBkpConf {
        &self.regdma_bkp_conf
    }
    #[doc = "0x30 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x34 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x38 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x3c - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x3fc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "REGDMA_CONF (rw) register accessor: Peri backup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_conf`] module"]
#[doc(alias = "REGDMA_CONF")]
pub type RegdmaConf = crate::Reg<regdma_conf::RegdmaConfSpec>;
#[doc = "Peri backup control register"]
pub mod regdma_conf;
#[doc = "REGDMA_CLK_CONF (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_clk_conf`] module"]
#[doc(alias = "REGDMA_CLK_CONF")]
pub type RegdmaClkConf = crate::Reg<regdma_clk_conf::RegdmaClkConfSpec>;
#[doc = "Clock control register"]
pub mod regdma_clk_conf;
#[doc = "REGDMA_ETM_CTRL (w) register accessor: ETM start ctrl reg\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_etm_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_etm_ctrl`] module"]
#[doc(alias = "REGDMA_ETM_CTRL")]
pub type RegdmaEtmCtrl = crate::Reg<regdma_etm_ctrl::RegdmaEtmCtrlSpec>;
#[doc = "ETM start ctrl reg"]
pub mod regdma_etm_ctrl;
#[doc = "REGDMA_LINK_0_ADDR (rw) register accessor: link_0_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_link_0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_link_0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_link_0_addr`] module"]
#[doc(alias = "REGDMA_LINK_0_ADDR")]
pub type RegdmaLink0Addr = crate::Reg<regdma_link_0_addr::RegdmaLink0AddrSpec>;
#[doc = "link_0_addr"]
pub mod regdma_link_0_addr;
#[doc = "REGDMA_LINK_1_ADDR (rw) register accessor: Link_1_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_link_1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_link_1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_link_1_addr`] module"]
#[doc(alias = "REGDMA_LINK_1_ADDR")]
pub type RegdmaLink1Addr = crate::Reg<regdma_link_1_addr::RegdmaLink1AddrSpec>;
#[doc = "Link_1_addr"]
pub mod regdma_link_1_addr;
#[doc = "REGDMA_LINK_2_ADDR (rw) register accessor: Link_2_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_link_2_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_link_2_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_link_2_addr`] module"]
#[doc(alias = "REGDMA_LINK_2_ADDR")]
pub type RegdmaLink2Addr = crate::Reg<regdma_link_2_addr::RegdmaLink2AddrSpec>;
#[doc = "Link_2_addr"]
pub mod regdma_link_2_addr;
#[doc = "REGDMA_LINK_3_ADDR (rw) register accessor: Link_3_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_link_3_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_link_3_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_link_3_addr`] module"]
#[doc(alias = "REGDMA_LINK_3_ADDR")]
pub type RegdmaLink3Addr = crate::Reg<regdma_link_3_addr::RegdmaLink3AddrSpec>;
#[doc = "Link_3_addr"]
pub mod regdma_link_3_addr;
#[doc = "REGDMA_LINK_MAC_ADDR (rw) register accessor: Link_mac_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_link_mac_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_link_mac_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_link_mac_addr`] module"]
#[doc(alias = "REGDMA_LINK_MAC_ADDR")]
pub type RegdmaLinkMacAddr = crate::Reg<regdma_link_mac_addr::RegdmaLinkMacAddrSpec>;
#[doc = "Link_mac_addr"]
pub mod regdma_link_mac_addr;
#[doc = "REGDMA_CURRENT_LINK_ADDR (r) register accessor: current link addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_current_link_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_current_link_addr`] module"]
#[doc(alias = "REGDMA_CURRENT_LINK_ADDR")]
pub type RegdmaCurrentLinkAddr = crate::Reg<regdma_current_link_addr::RegdmaCurrentLinkAddrSpec>;
#[doc = "current link addr"]
pub mod regdma_current_link_addr;
#[doc = "REGDMA_BACKUP_ADDR (r) register accessor: Backup addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_backup_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_backup_addr`] module"]
#[doc(alias = "REGDMA_BACKUP_ADDR")]
pub type RegdmaBackupAddr = crate::Reg<regdma_backup_addr::RegdmaBackupAddrSpec>;
#[doc = "Backup addr"]
pub mod regdma_backup_addr;
#[doc = "REGDMA_MEM_ADDR (r) register accessor: mem addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_mem_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_mem_addr`] module"]
#[doc(alias = "REGDMA_MEM_ADDR")]
pub type RegdmaMemAddr = crate::Reg<regdma_mem_addr::RegdmaMemAddrSpec>;
#[doc = "mem addr"]
pub mod regdma_mem_addr;
#[doc = "REGDMA_BKP_CONF (rw) register accessor: backup config\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_bkp_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_bkp_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_bkp_conf`] module"]
#[doc(alias = "REGDMA_BKP_CONF")]
pub type RegdmaBkpConf = crate::Reg<regdma_bkp_conf::RegdmaBkpConfSpec>;
#[doc = "backup config"]
pub mod regdma_bkp_conf;
#[doc = "INT_ENA (rw) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "Read only register for error and done"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "Read only register for error and done"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: Read only register for error and done\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "Read only register for error and done"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "Read only register for error and done"]
pub mod int_st;
#[doc = "DATE (rw) register accessor: Date register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Date register."]
pub mod date;
