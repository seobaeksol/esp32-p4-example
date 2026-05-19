#[repr(C)]
#[doc = "Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
#[doc(alias = "TIMER")]
pub struct Timer {
    cfg0: Cfg0,
    cfg1: Cfg1,
    sync: Sync,
    status: Status,
}
impl Timer {
    #[doc = "0x00 - PWM timer0 period and update method configuration register."]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x04 - PWM timer0 working mode and start/stop control register."]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x08 - PWM timer0 sync function configuration register."]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x0c - PWM timer0 status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "CFG0 (rw) register accessor: PWM timer0 period and update method configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "PWM timer0 period and update method configuration register."]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: PWM timer0 working mode and start/stop control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "PWM timer0 working mode and start/stop control register."]
pub mod cfg1;
#[doc = "SYNC (rw) register accessor: PWM timer0 sync function configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`] module"]
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "PWM timer0 sync function configuration register."]
pub mod sync;
#[doc = "STATUS (r) register accessor: PWM timer0 status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "PWM timer0 status register."]
pub mod status;
