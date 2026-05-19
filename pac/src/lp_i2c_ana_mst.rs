#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c0_ctrl: I2c0Ctrl,
    i2c1_ctrl: I2c1Ctrl,
    i2c0_conf: I2c0Conf,
    i2c1_conf: I2c1Conf,
    i2c_burst_conf: I2cBurstConf,
    i2c_burst_status: I2cBurstStatus,
    ana_conf0: AnaConf0,
    ana_conf1: AnaConf1,
    ana_conf2: AnaConf2,
    i2c0_ctrl1: I2c0Ctrl1,
    i2c1_ctrl1: I2c1Ctrl1,
    hw_i2c_ctrl: HwI2cCtrl,
    nouse: Nouse,
    clk160m: Clk160m,
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - need des"]
    #[inline(always)]
    pub const fn i2c0_ctrl(&self) -> &I2c0Ctrl {
        &self.i2c0_ctrl
    }
    #[doc = "0x04 - need des"]
    #[inline(always)]
    pub const fn i2c1_ctrl(&self) -> &I2c1Ctrl {
        &self.i2c1_ctrl
    }
    #[doc = "0x08 - need des"]
    #[inline(always)]
    pub const fn i2c0_conf(&self) -> &I2c0Conf {
        &self.i2c0_conf
    }
    #[doc = "0x0c - need des"]
    #[inline(always)]
    pub const fn i2c1_conf(&self) -> &I2c1Conf {
        &self.i2c1_conf
    }
    #[doc = "0x10 - need des"]
    #[inline(always)]
    pub const fn i2c_burst_conf(&self) -> &I2cBurstConf {
        &self.i2c_burst_conf
    }
    #[doc = "0x14 - need des"]
    #[inline(always)]
    pub const fn i2c_burst_status(&self) -> &I2cBurstStatus {
        &self.i2c_burst_status
    }
    #[doc = "0x18 - need des"]
    #[inline(always)]
    pub const fn ana_conf0(&self) -> &AnaConf0 {
        &self.ana_conf0
    }
    #[doc = "0x1c - need des"]
    #[inline(always)]
    pub const fn ana_conf1(&self) -> &AnaConf1 {
        &self.ana_conf1
    }
    #[doc = "0x20 - need des"]
    #[inline(always)]
    pub const fn ana_conf2(&self) -> &AnaConf2 {
        &self.ana_conf2
    }
    #[doc = "0x24 - need des"]
    #[inline(always)]
    pub const fn i2c0_ctrl1(&self) -> &I2c0Ctrl1 {
        &self.i2c0_ctrl1
    }
    #[doc = "0x28 - need des"]
    #[inline(always)]
    pub const fn i2c1_ctrl1(&self) -> &I2c1Ctrl1 {
        &self.i2c1_ctrl1
    }
    #[doc = "0x2c - need des"]
    #[inline(always)]
    pub const fn hw_i2c_ctrl(&self) -> &HwI2cCtrl {
        &self.hw_i2c_ctrl
    }
    #[doc = "0x30 - need des"]
    #[inline(always)]
    pub const fn nouse(&self) -> &Nouse {
        &self.nouse
    }
    #[doc = "0x34 - need des"]
    #[inline(always)]
    pub const fn clk160m(&self) -> &Clk160m {
        &self.clk160m
    }
    #[doc = "0x38 - need des"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "I2C0_CTRL (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl`] module"]
#[doc(alias = "I2C0_CTRL")]
pub type I2c0Ctrl = crate::Reg<i2c0_ctrl::I2c0CtrlSpec>;
#[doc = "need des"]
pub mod i2c0_ctrl;
#[doc = "I2C1_CTRL (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl`] module"]
#[doc(alias = "I2C1_CTRL")]
pub type I2c1Ctrl = crate::Reg<i2c1_ctrl::I2c1CtrlSpec>;
#[doc = "need des"]
pub mod i2c1_ctrl;
#[doc = "I2C0_CONF (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_conf`] module"]
#[doc(alias = "I2C0_CONF")]
pub type I2c0Conf = crate::Reg<i2c0_conf::I2c0ConfSpec>;
#[doc = "need des"]
pub mod i2c0_conf;
#[doc = "I2C1_CONF (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_conf`] module"]
#[doc(alias = "I2C1_CONF")]
pub type I2c1Conf = crate::Reg<i2c1_conf::I2c1ConfSpec>;
#[doc = "need des"]
pub mod i2c1_conf;
#[doc = "I2C_BURST_CONF (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_burst_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_burst_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_burst_conf`] module"]
#[doc(alias = "I2C_BURST_CONF")]
pub type I2cBurstConf = crate::Reg<i2c_burst_conf::I2cBurstConfSpec>;
#[doc = "need des"]
pub mod i2c_burst_conf;
#[doc = "I2C_BURST_STATUS (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_burst_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_burst_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_burst_status`] module"]
#[doc(alias = "I2C_BURST_STATUS")]
pub type I2cBurstStatus = crate::Reg<i2c_burst_status::I2cBurstStatusSpec>;
#[doc = "need des"]
pub mod i2c_burst_status;
#[doc = "ANA_CONF0 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf0`] module"]
#[doc(alias = "ANA_CONF0")]
pub type AnaConf0 = crate::Reg<ana_conf0::AnaConf0Spec>;
#[doc = "need des"]
pub mod ana_conf0;
#[doc = "ANA_CONF1 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf1`] module"]
#[doc(alias = "ANA_CONF1")]
pub type AnaConf1 = crate::Reg<ana_conf1::AnaConf1Spec>;
#[doc = "need des"]
pub mod ana_conf1;
#[doc = "ANA_CONF2 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf2`] module"]
#[doc(alias = "ANA_CONF2")]
pub type AnaConf2 = crate::Reg<ana_conf2::AnaConf2Spec>;
#[doc = "need des"]
pub mod ana_conf2;
#[doc = "I2C0_CTRL1 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl1`] module"]
#[doc(alias = "I2C0_CTRL1")]
pub type I2c0Ctrl1 = crate::Reg<i2c0_ctrl1::I2c0Ctrl1Spec>;
#[doc = "need des"]
pub mod i2c0_ctrl1;
#[doc = "I2C1_CTRL1 (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl1`] module"]
#[doc(alias = "I2C1_CTRL1")]
pub type I2c1Ctrl1 = crate::Reg<i2c1_ctrl1::I2c1Ctrl1Spec>;
#[doc = "need des"]
pub mod i2c1_ctrl1;
#[doc = "HW_I2C_CTRL (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_i2c_ctrl`] module"]
#[doc(alias = "HW_I2C_CTRL")]
pub type HwI2cCtrl = crate::Reg<hw_i2c_ctrl::HwI2cCtrlSpec>;
#[doc = "need des"]
pub mod hw_i2c_ctrl;
#[doc = "NOUSE (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`nouse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nouse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nouse`] module"]
#[doc(alias = "NOUSE")]
pub type Nouse = crate::Reg<nouse::NouseSpec>;
#[doc = "need des"]
pub mod nouse;
#[doc = "CLK160M (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk160m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk160m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk160m`] module"]
#[doc(alias = "CLK160M")]
pub type Clk160m = crate::Reg<clk160m::Clk160mSpec>;
#[doc = "need des"]
pub mod clk160m;
#[doc = "DATE (rw) register accessor: need des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "need des"]
pub mod date;
