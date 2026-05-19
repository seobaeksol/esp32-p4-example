#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [Ch; 8],
    timer: [Timer; 4],
    int_raw: IntRaw,
    int_st: IntSt,
    int_ena: IntEna,
    int_clr: IntClr,
    _reserved6: [u8; 0x30],
    ch_gamma_conf: [ChGammaConf; 8],
    evt_task_en0: EvtTaskEn0,
    evt_task_en1: EvtTaskEn1,
    evt_task_en2: EvtTaskEn2,
    _reserved10: [u8; 0x14],
    timer_cmp: [TimerCmp; 4],
    timer_cnt_cap: [TimerCntCap; 4],
    _reserved12: [u8; 0x10],
    conf: Conf,
    date: Date,
    _reserved14: [u8; 0x0288],
    ch_gamma_range: [ChGammaRange; 128],
}
impl RegisterBlock {
    #[doc = "0x00..0xa0 - Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xa0 - Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0xa0..0xc0 - Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
    #[inline(always)]
    pub const fn timer(&self, n: usize) -> &Timer {
        &self.timer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xc0 - Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
    #[inline(always)]
    pub fn timer_iter(&self) -> impl Iterator<Item = &Timer> {
        self.timer.iter()
    }
    #[doc = "0xc0 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0xc4 - Interrupt masked status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0xc8 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0xcc - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x100..0x120 - Ledc ch%s gamma config register."]
    #[inline(always)]
    pub const fn ch_gamma_conf(&self, n: usize) -> &ChGammaConf {
        &self.ch_gamma_conf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Ledc ch%s gamma config register."]
    #[inline(always)]
    pub fn ch_gamma_conf_iter(&self) -> impl Iterator<Item = &ChGammaConf> {
        self.ch_gamma_conf.iter()
    }
    #[doc = "0x100 - Ledc ch0 gamma config register."]
    #[inline(always)]
    pub const fn ch0_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(0)
    }
    #[doc = "0x104 - Ledc ch1 gamma config register."]
    #[inline(always)]
    pub const fn ch1_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(1)
    }
    #[doc = "0x108 - Ledc ch2 gamma config register."]
    #[inline(always)]
    pub const fn ch2_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(2)
    }
    #[doc = "0x10c - Ledc ch3 gamma config register."]
    #[inline(always)]
    pub const fn ch3_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(3)
    }
    #[doc = "0x110 - Ledc ch4 gamma config register."]
    #[inline(always)]
    pub const fn ch4_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(4)
    }
    #[doc = "0x114 - Ledc ch5 gamma config register."]
    #[inline(always)]
    pub const fn ch5_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(5)
    }
    #[doc = "0x118 - Ledc ch6 gamma config register."]
    #[inline(always)]
    pub const fn ch6_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(6)
    }
    #[doc = "0x11c - Ledc ch7 gamma config register."]
    #[inline(always)]
    pub const fn ch7_gamma_conf(&self) -> &ChGammaConf {
        self.ch_gamma_conf(7)
    }
    #[doc = "0x120 - Ledc event task enable bit register0."]
    #[inline(always)]
    pub const fn evt_task_en0(&self) -> &EvtTaskEn0 {
        &self.evt_task_en0
    }
    #[doc = "0x124 - Ledc event task enable bit register1."]
    #[inline(always)]
    pub const fn evt_task_en1(&self) -> &EvtTaskEn1 {
        &self.evt_task_en1
    }
    #[doc = "0x128 - Ledc event task enable bit register2."]
    #[inline(always)]
    pub const fn evt_task_en2(&self) -> &EvtTaskEn2 {
        &self.evt_task_en2
    }
    #[doc = "0x140..0x150 - Ledc timer%s compare value register."]
    #[inline(always)]
    pub const fn timer_cmp(&self, n: usize) -> &TimerCmp {
        &self.timer_cmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - Ledc timer%s compare value register."]
    #[inline(always)]
    pub fn timer_cmp_iter(&self) -> impl Iterator<Item = &TimerCmp> {
        self.timer_cmp.iter()
    }
    #[doc = "0x140 - Ledc timer0 compare value register."]
    #[inline(always)]
    pub const fn timer0_cmp(&self) -> &TimerCmp {
        self.timer_cmp(0)
    }
    #[doc = "0x144 - Ledc timer1 compare value register."]
    #[inline(always)]
    pub const fn timer1_cmp(&self) -> &TimerCmp {
        self.timer_cmp(1)
    }
    #[doc = "0x148 - Ledc timer2 compare value register."]
    #[inline(always)]
    pub const fn timer2_cmp(&self) -> &TimerCmp {
        self.timer_cmp(2)
    }
    #[doc = "0x14c - Ledc timer3 compare value register."]
    #[inline(always)]
    pub const fn timer3_cmp(&self) -> &TimerCmp {
        self.timer_cmp(3)
    }
    #[doc = "0x150..0x160 - Ledc timer%s captured count value register."]
    #[inline(always)]
    pub const fn timer_cnt_cap(&self, n: usize) -> &TimerCntCap {
        &self.timer_cnt_cap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - Ledc timer%s captured count value register."]
    #[inline(always)]
    pub fn timer_cnt_cap_iter(&self) -> impl Iterator<Item = &TimerCntCap> {
        self.timer_cnt_cap.iter()
    }
    #[doc = "0x150 - Ledc timer0 captured count value register."]
    #[inline(always)]
    pub const fn timer0_cnt_cap(&self) -> &TimerCntCap {
        self.timer_cnt_cap(0)
    }
    #[doc = "0x154 - Ledc timer1 captured count value register."]
    #[inline(always)]
    pub const fn timer1_cnt_cap(&self) -> &TimerCntCap {
        self.timer_cnt_cap(1)
    }
    #[doc = "0x158 - Ledc timer2 captured count value register."]
    #[inline(always)]
    pub const fn timer2_cnt_cap(&self) -> &TimerCntCap {
        self.timer_cnt_cap(2)
    }
    #[doc = "0x15c - Ledc timer3 captured count value register."]
    #[inline(always)]
    pub const fn timer3_cnt_cap(&self) -> &TimerCntCap {
        self.timer_cnt_cap(3)
    }
    #[doc = "0x170 - LEDC global configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x174 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x400..0x600 - Gamma range word %s (channel = %s / 16, range = %s mod 16)"]
    #[inline(always)]
    pub const fn ch_gamma_range(&self, n: usize) -> &ChGammaRange {
        &self.ch_gamma_range[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x600 - Gamma range word %s (channel = %s / 16, range = %s mod 16)"]
    #[inline(always)]
    pub fn ch_gamma_range_iter(&self) -> impl Iterator<Item = &ChGammaRange> {
        self.ch_gamma_range.iter()
    }
}
#[doc = "Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
pub mod ch;
#[doc = "Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
pub use self::timer::Timer;
#[doc = r"Cluster"]
#[doc = "Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
pub mod timer;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "Interrupt masked status register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "CH_GAMMA_CONF (rw) register accessor: Ledc ch%s gamma config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_gamma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_gamma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_gamma_conf`] module"]
#[doc(alias = "CH_GAMMA_CONF")]
pub type ChGammaConf = crate::Reg<ch_gamma_conf::ChGammaConfSpec>;
#[doc = "Ledc ch%s gamma config register."]
pub mod ch_gamma_conf;
#[doc = "EVT_TASK_EN0 (rw) register accessor: Ledc event task enable bit register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en0`] module"]
#[doc(alias = "EVT_TASK_EN0")]
pub type EvtTaskEn0 = crate::Reg<evt_task_en0::EvtTaskEn0Spec>;
#[doc = "Ledc event task enable bit register0."]
pub mod evt_task_en0;
#[doc = "EVT_TASK_EN1 (rw) register accessor: Ledc event task enable bit register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en1`] module"]
#[doc(alias = "EVT_TASK_EN1")]
pub type EvtTaskEn1 = crate::Reg<evt_task_en1::EvtTaskEn1Spec>;
#[doc = "Ledc event task enable bit register1."]
pub mod evt_task_en1;
#[doc = "EVT_TASK_EN2 (rw) register accessor: Ledc event task enable bit register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en2`] module"]
#[doc(alias = "EVT_TASK_EN2")]
pub type EvtTaskEn2 = crate::Reg<evt_task_en2::EvtTaskEn2Spec>;
#[doc = "Ledc event task enable bit register2."]
pub mod evt_task_en2;
#[doc = "TIMER_CMP (rw) register accessor: Ledc timer%s compare value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cmp`] module"]
#[doc(alias = "TIMER_CMP")]
pub type TimerCmp = crate::Reg<timer_cmp::TimerCmpSpec>;
#[doc = "Ledc timer%s compare value register."]
pub mod timer_cmp;
#[doc = "TIMER_CNT_CAP (r) register accessor: Ledc timer%s captured count value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cnt_cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cnt_cap`] module"]
#[doc(alias = "TIMER_CNT_CAP")]
pub type TimerCntCap = crate::Reg<timer_cnt_cap::TimerCntCapSpec>;
#[doc = "Ledc timer%s captured count value register."]
pub mod timer_cnt_cap;
#[doc = "CONF (rw) register accessor: LEDC global configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "LEDC global configuration register"]
pub mod conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control register"]
pub mod date;
#[doc = "CH_GAMMA_RANGE (rw) register accessor: Gamma range word %s (channel = %s / 16, range = %s mod 16)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_gamma_range::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_gamma_range::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_gamma_range`] module"]
#[doc(alias = "CH_GAMMA_RANGE")]
pub type ChGammaRange = crate::Reg<ch_gamma_range::ChGammaRangeSpec>;
#[doc = "Gamma range word %s (channel = %s / 16, range = %s mod 16)"]
pub mod ch_gamma_range;
