#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    otg_conf: OtgConf,
    test_conf: TestConf,
    _reserved2: [u8; 0x03f4],
    date: Date,
}
impl RegisterBlock {
    #[doc = "0x00 - USB wrapper configuration registers."]
    #[inline(always)]
    pub const fn otg_conf(&self) -> &OtgConf {
        &self.otg_conf
    }
    #[doc = "0x04 - USB wrapper test configuration registers."]
    #[inline(always)]
    pub const fn test_conf(&self) -> &TestConf {
        &self.test_conf
    }
    #[doc = "0x3fc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
}
#[doc = "OTG_CONF (rw) register accessor: USB wrapper configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_conf`] module"]
#[doc(alias = "OTG_CONF")]
pub type OtgConf = crate::Reg<otg_conf::OtgConfSpec>;
#[doc = "USB wrapper configuration registers."]
pub mod otg_conf;
#[doc = "TEST_CONF (rw) register accessor: USB wrapper test configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_conf`] module"]
#[doc(alias = "TEST_CONF")]
pub type TestConf = crate::Reg<test_conf::TestConfSpec>;
#[doc = "USB wrapper test configuration registers."]
pub mod test_conf;
#[doc = "DATE (r) register accessor: Date register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Date register."]
pub mod date;
