#[repr(C)]
#[doc = "Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
#[doc(alias = "CH")]
pub struct Ch {
    gen_stmp_cfg: GenStmpCfg,
    gen_tstmp_a: GenTstmpA,
    gen_tstmp_b: GenTstmpB,
    gen_cfg0: GenCfg0,
    gen_force: GenForce,
    gen_: [Gen; 2],
    dt_cfg: DtCfg,
    dt_fed_cfg: DtFedCfg,
    dt_red_cfg: DtRedCfg,
    carrier_cfg: CarrierCfg,
    fh_cfg0: FhCfg0,
    fh_cfg1: FhCfg1,
    fh_status: FhStatus,
}
impl Ch {
    #[doc = "0x00 - Generator0 time stamp registers A and B transfer status and update method register"]
    #[inline(always)]
    pub const fn gen_stmp_cfg(&self) -> &GenStmpCfg {
        &self.gen_stmp_cfg
    }
    #[doc = "0x04 - Generator0 time stamp A's shadow register"]
    #[inline(always)]
    pub const fn gen_tstmp_a(&self) -> &GenTstmpA {
        &self.gen_tstmp_a
    }
    #[doc = "0x08 - Generator0 time stamp B's shadow register"]
    #[inline(always)]
    pub const fn gen_tstmp_b(&self) -> &GenTstmpB {
        &self.gen_tstmp_b
    }
    #[doc = "0x0c - Generator0 fault event T0 and T1 configuration register"]
    #[inline(always)]
    pub const fn gen_cfg0(&self) -> &GenCfg0 {
        &self.gen_cfg0
    }
    #[doc = "0x10 - Generator0 output signal force mode register."]
    #[inline(always)]
    pub const fn gen_force(&self) -> &GenForce {
        &self.gen_force
    }
    #[doc = "0x14..0x1c - Actions triggered by events on PWMx%s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `GENA` register.</div>"]
    #[inline(always)]
    pub const fn gen_(&self, n: usize) -> &Gen {
        &self.gen_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x1c - Actions triggered by events on PWMx%s"]
    #[inline(always)]
    pub fn gen__iter(&self) -> impl Iterator<Item = &Gen> {
        self.gen_.iter()
    }
    #[doc = "0x14 - Actions triggered by events on PWMxA"]
    #[inline(always)]
    pub const fn gena(&self) -> &Gen {
        self.gen_(0)
    }
    #[doc = "0x18 - Actions triggered by events on PWMxB"]
    #[inline(always)]
    pub const fn genb(&self) -> &Gen {
        self.gen_(1)
    }
    #[doc = "0x1c - Dead time configuration register"]
    #[inline(always)]
    pub const fn dt_cfg(&self) -> &DtCfg {
        &self.dt_cfg
    }
    #[doc = "0x20 - Falling edge delay (FED) shadow register"]
    #[inline(always)]
    pub const fn dt_fed_cfg(&self) -> &DtFedCfg {
        &self.dt_fed_cfg
    }
    #[doc = "0x24 - Rising edge delay (RED) shadow register"]
    #[inline(always)]
    pub const fn dt_red_cfg(&self) -> &DtRedCfg {
        &self.dt_red_cfg
    }
    #[doc = "0x28 - Carrier0 configuration register"]
    #[inline(always)]
    pub const fn carrier_cfg(&self) -> &CarrierCfg {
        &self.carrier_cfg
    }
    #[doc = "0x2c - PWM0 A and PWM0 B trip events actions configuration register"]
    #[inline(always)]
    pub const fn fh_cfg0(&self) -> &FhCfg0 {
        &self.fh_cfg0
    }
    #[doc = "0x30 - Software triggers for fault handler actions configuration register"]
    #[inline(always)]
    pub const fn fh_cfg1(&self) -> &FhCfg1 {
        &self.fh_cfg1
    }
    #[doc = "0x34 - Fault events status register"]
    #[inline(always)]
    pub const fn fh_status(&self) -> &FhStatus {
        &self.fh_status
    }
}
#[doc = "GEN_STMP_CFG (rw) register accessor: Generator0 time stamp registers A and B transfer status and update method register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_stmp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_stmp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_stmp_cfg`] module"]
#[doc(alias = "GEN_STMP_CFG")]
pub type GenStmpCfg = crate::Reg<gen_stmp_cfg::GenStmpCfgSpec>;
#[doc = "Generator0 time stamp registers A and B transfer status and update method register"]
pub mod gen_stmp_cfg;
#[doc = "GEN_TSTMP_A (rw) register accessor: Generator0 time stamp A's shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_tstmp_a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_tstmp_a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_tstmp_a`] module"]
#[doc(alias = "GEN_TSTMP_A")]
pub type GenTstmpA = crate::Reg<gen_tstmp_a::GenTstmpASpec>;
#[doc = "Generator0 time stamp A's shadow register"]
pub mod gen_tstmp_a;
#[doc = "GEN_TSTMP_B (rw) register accessor: Generator0 time stamp B's shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_tstmp_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_tstmp_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_tstmp_b`] module"]
#[doc(alias = "GEN_TSTMP_B")]
pub type GenTstmpB = crate::Reg<gen_tstmp_b::GenTstmpBSpec>;
#[doc = "Generator0 time stamp B's shadow register"]
pub mod gen_tstmp_b;
#[doc = "GEN_CFG0 (rw) register accessor: Generator0 fault event T0 and T1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_cfg0`] module"]
#[doc(alias = "GEN_CFG0")]
pub type GenCfg0 = crate::Reg<gen_cfg0::GenCfg0Spec>;
#[doc = "Generator0 fault event T0 and T1 configuration register"]
pub mod gen_cfg0;
#[doc = "GEN_FORCE (rw) register accessor: Generator0 output signal force mode register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_force`] module"]
#[doc(alias = "GEN_FORCE")]
pub type GenForce = crate::Reg<gen_force::GenForceSpec>;
#[doc = "Generator0 output signal force mode register."]
pub mod gen_force;
#[doc = "GEN (rw) register accessor: Actions triggered by events on PWMx%s\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_`] module"]
#[doc(alias = "GEN")]
pub type Gen = crate::Reg<gen_::GenSpec>;
#[doc = "Actions triggered by events on PWMx%s"]
pub mod gen_;
#[doc = "DT_CFG (rw) register accessor: Dead time configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_cfg`] module"]
#[doc(alias = "DT_CFG")]
pub type DtCfg = crate::Reg<dt_cfg::DtCfgSpec>;
#[doc = "Dead time configuration register"]
pub mod dt_cfg;
#[doc = "DT_FED_CFG (rw) register accessor: Falling edge delay (FED) shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_fed_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt_fed_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_fed_cfg`] module"]
#[doc(alias = "DT_FED_CFG")]
pub type DtFedCfg = crate::Reg<dt_fed_cfg::DtFedCfgSpec>;
#[doc = "Falling edge delay (FED) shadow register"]
pub mod dt_fed_cfg;
#[doc = "DT_RED_CFG (rw) register accessor: Rising edge delay (RED) shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_red_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt_red_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_red_cfg`] module"]
#[doc(alias = "DT_RED_CFG")]
pub type DtRedCfg = crate::Reg<dt_red_cfg::DtRedCfgSpec>;
#[doc = "Rising edge delay (RED) shadow register"]
pub mod dt_red_cfg;
#[doc = "CARRIER_CFG (rw) register accessor: Carrier0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`carrier_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`carrier_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@carrier_cfg`] module"]
#[doc(alias = "CARRIER_CFG")]
pub type CarrierCfg = crate::Reg<carrier_cfg::CarrierCfgSpec>;
#[doc = "Carrier0 configuration register"]
pub mod carrier_cfg;
#[doc = "FH_CFG0 (rw) register accessor: PWM0 A and PWM0 B trip events actions configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fh_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fh_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_cfg0`] module"]
#[doc(alias = "FH_CFG0")]
pub type FhCfg0 = crate::Reg<fh_cfg0::FhCfg0Spec>;
#[doc = "PWM0 A and PWM0 B trip events actions configuration register"]
pub mod fh_cfg0;
#[doc = "FH_CFG1 (rw) register accessor: Software triggers for fault handler actions configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fh_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fh_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_cfg1`] module"]
#[doc(alias = "FH_CFG1")]
pub type FhCfg1 = crate::Reg<fh_cfg1::FhCfg1Spec>;
#[doc = "Software triggers for fault handler actions configuration register"]
pub mod fh_cfg1;
#[doc = "FH_STATUS (r) register accessor: Fault events status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fh_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_status`] module"]
#[doc(alias = "FH_STATUS")]
pub type FhStatus = crate::Reg<fh_status::FhStatusSpec>;
#[doc = "Fault events status register"]
pub mod fh_status;
