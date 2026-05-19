#[repr(C)]
#[doc = "Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
#[doc(alias = "TIMER")]
pub struct Timer {
    conf: Conf,
    value: Value,
}
impl Timer {
    #[doc = "0x00 - Timer 0 configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x04 - Timer 0 current counter value register"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "CONF (rw) register accessor: Timer 0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "Timer 0 configuration register"]
pub mod conf;
#[doc = "VALUE (r) register accessor: Timer 0 current counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Timer 0 current counter value register"]
pub mod value;
