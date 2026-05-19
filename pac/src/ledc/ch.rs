#[repr(C)]
#[doc = "Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
#[doc(alias = "CH")]
pub struct Ch {
    conf0: Conf0,
    hpoint: Hpoint,
    duty: Duty,
    conf1: Conf1,
    duty_r: DutyR,
}
impl Ch {
    #[doc = "0x00 - Configuration register 0 for channel 0"]
    #[inline(always)]
    pub const fn conf0(&self) -> &Conf0 {
        &self.conf0
    }
    #[doc = "0x04 - High point register for channel 0"]
    #[inline(always)]
    pub const fn hpoint(&self) -> &Hpoint {
        &self.hpoint
    }
    #[doc = "0x08 - Initial duty cycle register for channel 0"]
    #[inline(always)]
    pub const fn duty(&self) -> &Duty {
        &self.duty
    }
    #[doc = "0x0c - Configuration register 1 for channel 0"]
    #[inline(always)]
    pub const fn conf1(&self) -> &Conf1 {
        &self.conf1
    }
    #[doc = "0x10 - Current duty cycle register for channel 0"]
    #[inline(always)]
    pub const fn duty_r(&self) -> &DutyR {
        &self.duty_r
    }
}
#[doc = "CONF0 (rw) register accessor: Configuration register 0 for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
#[doc(alias = "CONF0")]
pub type Conf0 = crate::Reg<conf0::Conf0Spec>;
#[doc = "Configuration register 0 for channel 0"]
pub mod conf0;
#[doc = "HPOINT (rw) register accessor: High point register for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hpoint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpoint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpoint`] module"]
#[doc(alias = "HPOINT")]
pub type Hpoint = crate::Reg<hpoint::HpointSpec>;
#[doc = "High point register for channel 0"]
pub mod hpoint;
#[doc = "DUTY (rw) register accessor: Initial duty cycle register for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duty`] module"]
#[doc(alias = "DUTY")]
pub type Duty = crate::Reg<duty::DutySpec>;
#[doc = "Initial duty cycle register for channel 0"]
pub mod duty;
#[doc = "CONF1 (rw) register accessor: Configuration register 1 for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
#[doc(alias = "CONF1")]
pub type Conf1 = crate::Reg<conf1::Conf1Spec>;
#[doc = "Configuration register 1 for channel 0"]
pub mod conf1;
#[doc = "DUTY_R (r) register accessor: Current duty cycle register for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`duty_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duty_r`] module"]
#[doc(alias = "DUTY_R")]
pub type DutyR = crate::Reg<duty_r::DutyRSpec>;
#[doc = "Current duty cycle register for channel 0"]
pub mod duty_r;
