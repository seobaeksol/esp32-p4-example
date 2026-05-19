#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid_fileds: VeridFileds,
    hw_cfg: HwCfg,
    cmd: Cmd,
    data: Data,
}
impl RegisterBlock {
    #[doc = "0x00 - QoS block version ID (typo matches IDF symbol)"]
    #[inline(always)]
    pub const fn verid_fileds(&self) -> &VeridFileds {
        &self.verid_fileds
    }
    #[doc = "0x04 - QoS hardware configuration"]
    #[inline(always)]
    pub const fn hw_cfg(&self) -> &HwCfg {
        &self.hw_cfg
    }
    #[doc = "0x08 - QoS indirect command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x0c - QoS indirect data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "VERID_FILEDS (r) register accessor: QoS block version ID (typo matches IDF symbol)\n\nYou can [`read`](crate::Reg::read) this register and get [`verid_fileds::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid_fileds`] module"]
#[doc(alias = "VERID_FILEDS")]
pub type VeridFileds = crate::Reg<verid_fileds::VeridFiledsSpec>;
#[doc = "QoS block version ID (typo matches IDF symbol)"]
pub mod verid_fileds;
#[doc = "HW_CFG (r) register accessor: QoS hardware configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_cfg`] module"]
#[doc(alias = "HW_CFG")]
pub type HwCfg = crate::Reg<hw_cfg::HwCfgSpec>;
#[doc = "QoS hardware configuration"]
pub mod hw_cfg;
#[doc = "CMD (rw) register accessor: QoS indirect command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "QoS indirect command"]
pub mod cmd;
#[doc = "DATA (rw) register accessor: QoS indirect data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "QoS indirect data"]
pub mod data;
