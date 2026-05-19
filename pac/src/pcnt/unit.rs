#[repr(C)]
#[doc = "Cluster UNIT%s, containing U?_CONF0, U?_CONF1, U?_CONF2"]
#[doc(alias = "UNIT")]
pub struct Unit {
    conf0: Conf0,
    conf1: Conf1,
    conf2: Conf2,
}
impl Unit {
    #[doc = "0x00 - Configuration register 0 for unit"]
    #[inline(always)]
    pub const fn conf0(&self) -> &Conf0 {
        &self.conf0
    }
    #[doc = "0x04 - Configuration register 1 for unit 0"]
    #[inline(always)]
    pub const fn conf1(&self) -> &Conf1 {
        &self.conf1
    }
    #[doc = "0x08 - Configuration register 2 for unit 0"]
    #[inline(always)]
    pub const fn conf2(&self) -> &Conf2 {
        &self.conf2
    }
}
#[doc = "CONF0 (rw) register accessor: Configuration register 0 for unit\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
#[doc(alias = "CONF0")]
pub type Conf0 = crate::Reg<conf0::Conf0Spec>;
#[doc = "Configuration register 0 for unit"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: Configuration register 1 for unit 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
#[doc(alias = "CONF1")]
pub type Conf1 = crate::Reg<conf1::Conf1Spec>;
#[doc = "Configuration register 1 for unit 0"]
pub mod conf1;
#[doc = "CONF2 (rw) register accessor: Configuration register 2 for unit 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf2`] module"]
#[doc(alias = "CONF2")]
pub type Conf2 = crate::Reg<conf2::Conf2Spec>;
#[doc = "Configuration register 2 for unit 0"]
pub mod conf2;
