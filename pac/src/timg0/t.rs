#[repr(C)]
#[doc = "Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD"]
pub struct T {
    config: Config,
    lo: Lo,
    hi: Hi,
    update: Update,
    alarmlo: Alarmlo,
    alarmhi: Alarmhi,
    loadlo: Loadlo,
    loadhi: Loadhi,
    load: Load,
}
impl T {
    #[doc = "0x00 - Timer 0 configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Timer 0 current value, low 32 bits"]
    #[inline(always)]
    pub const fn lo(&self) -> &Lo {
        &self.lo
    }
    #[doc = "0x08 - Timer 0 current value, high 22 bits"]
    #[inline(always)]
    pub const fn hi(&self) -> &Hi {
        &self.hi
    }
    #[doc = "0x0c - Write to copy current timer value to TIMGn_T0_(LO/HI)_REG"]
    #[inline(always)]
    pub const fn update(&self) -> &Update {
        &self.update
    }
    #[doc = "0x10 - Timer 0 alarm value, low 32 bits"]
    #[inline(always)]
    pub const fn alarmlo(&self) -> &Alarmlo {
        &self.alarmlo
    }
    #[doc = "0x14 - Timer 0 alarm value, high bits"]
    #[inline(always)]
    pub const fn alarmhi(&self) -> &Alarmhi {
        &self.alarmhi
    }
    #[doc = "0x18 - Timer 0 reload value, low 32 bits"]
    #[inline(always)]
    pub const fn loadlo(&self) -> &Loadlo {
        &self.loadlo
    }
    #[doc = "0x1c - Timer 0 reload value, high 22 bits"]
    #[inline(always)]
    pub const fn loadhi(&self) -> &Loadhi {
        &self.loadhi
    }
    #[doc = "0x20 - Write to reload timer from TIMG_T0_(LOADLOLOADHI)_REG"]
    #[inline(always)]
    pub const fn load(&self) -> &Load {
        &self.load
    }
}
#[doc = "CONFIG (rw) register accessor: Timer 0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Timer 0 configuration register"]
pub mod config;
#[doc = "LO (r) register accessor: Timer 0 current value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
#[doc(alias = "LO")]
pub type Lo = crate::Reg<lo::LoSpec>;
#[doc = "Timer 0 current value, low 32 bits"]
pub mod lo;
#[doc = "HI (r) register accessor: Timer 0 current value, high 22 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
#[doc(alias = "HI")]
pub type Hi = crate::Reg<hi::HiSpec>;
#[doc = "Timer 0 current value, high 22 bits"]
pub mod hi;
#[doc = "UPDATE (rw) register accessor: Write to copy current timer value to TIMGn_T0_(LO/HI)_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@update`] module"]
#[doc(alias = "UPDATE")]
pub type Update = crate::Reg<update::UpdateSpec>;
#[doc = "Write to copy current timer value to TIMGn_T0_(LO/HI)_REG"]
pub mod update;
#[doc = "ALARMLO (rw) register accessor: Timer 0 alarm value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmlo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmlo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmlo`] module"]
#[doc(alias = "ALARMLO")]
pub type Alarmlo = crate::Reg<alarmlo::AlarmloSpec>;
#[doc = "Timer 0 alarm value, low 32 bits"]
pub mod alarmlo;
#[doc = "ALARMHI (rw) register accessor: Timer 0 alarm value, high bits\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmhi`] module"]
#[doc(alias = "ALARMHI")]
pub type Alarmhi = crate::Reg<alarmhi::AlarmhiSpec>;
#[doc = "Timer 0 alarm value, high bits"]
pub mod alarmhi;
#[doc = "LOADLO (rw) register accessor: Timer 0 reload value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`loadlo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadlo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loadlo`] module"]
#[doc(alias = "LOADLO")]
pub type Loadlo = crate::Reg<loadlo::LoadloSpec>;
#[doc = "Timer 0 reload value, low 32 bits"]
pub mod loadlo;
#[doc = "LOADHI (rw) register accessor: Timer 0 reload value, high 22 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`loadhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loadhi`] module"]
#[doc(alias = "LOADHI")]
pub type Loadhi = crate::Reg<loadhi::LoadhiSpec>;
#[doc = "Timer 0 reload value, high 22 bits"]
pub mod loadhi;
#[doc = "LOAD (w) register accessor: Write to reload timer from TIMG_T0_(LOADLOLOADHI)_REG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`] module"]
#[doc(alias = "LOAD")]
pub type Load = crate::Reg<load::LoadSpec>;
#[doc = "Write to reload timer from TIMG_T0_(LOADLOLOADHI)_REG"]
pub mod load;
