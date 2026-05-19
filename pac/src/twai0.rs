#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: Mode,
    cmd: Cmd,
    status: Status,
    interrupt: Interrupt,
    interrupt_enable: InterruptEnable,
    _reserved5: [u8; 0x04],
    bus_timing_0: BusTiming0,
    bus_timing_1: BusTiming1,
    _reserved7: [u8; 0x0c],
    arb_lost_cap: ArbLostCap,
    err_code_cap: ErrCodeCap,
    err_warning_limit: ErrWarningLimit,
    rx_err_cnt: RxErrCnt,
    tx_err_cnt: TxErrCnt,
    data: [Data; 13],
    rx_message_counter: RxMessageCounter,
    _reserved14: [u8; 0x04],
    clock_divider: ClockDivider,
    sw_standby_cfg: SwStandbyCfg,
    hw_cfg: HwCfg,
    hw_standby_cnt: HwStandbyCnt,
    idle_intr_cnt: IdleIntrCnt,
    eco_cfg: EcoCfg,
    timestamp_data: TimestampData,
    timestamp_prescaler: TimestampPrescaler,
    timestamp_cfg: TimestampCfg,
}
impl RegisterBlock {
    #[doc = "0x00 - TWAI mode register."]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - TWAI command register."]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x08 - TWAI status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Interrupt signals' register."]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0x10 - Interrupt enable register."]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &InterruptEnable {
        &self.interrupt_enable
    }
    #[doc = "0x18 - Bit timing configuration register 0."]
    #[inline(always)]
    pub const fn bus_timing_0(&self) -> &BusTiming0 {
        &self.bus_timing_0
    }
    #[doc = "0x1c - Bit timing configuration register 1."]
    #[inline(always)]
    pub const fn bus_timing_1(&self) -> &BusTiming1 {
        &self.bus_timing_1
    }
    #[doc = "0x2c - TWAI arbiter lost capture register."]
    #[inline(always)]
    pub const fn arb_lost_cap(&self) -> &ArbLostCap {
        &self.arb_lost_cap
    }
    #[doc = "0x30 - TWAI error info capture register."]
    #[inline(always)]
    pub const fn err_code_cap(&self) -> &ErrCodeCap {
        &self.err_code_cap
    }
    #[doc = "0x34 - TWAI error threshold configuration register."]
    #[inline(always)]
    pub const fn err_warning_limit(&self) -> &ErrWarningLimit {
        &self.err_warning_limit
    }
    #[doc = "0x38 - Rx error counter register."]
    #[inline(always)]
    pub const fn rx_err_cnt(&self) -> &RxErrCnt {
        &self.rx_err_cnt
    }
    #[doc = "0x3c - Tx error counter register."]
    #[inline(always)]
    pub const fn tx_err_cnt(&self) -> &TxErrCnt {
        &self.tx_err_cnt
    }
    #[doc = "0x40..0x74 - Data register %s."]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x74 - Data register %s."]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
    #[doc = "0x74 - Received message counter register."]
    #[inline(always)]
    pub const fn rx_message_counter(&self) -> &RxMessageCounter {
        &self.rx_message_counter
    }
    #[doc = "0x7c - Clock divider register."]
    #[inline(always)]
    pub const fn clock_divider(&self) -> &ClockDivider {
        &self.clock_divider
    }
    #[doc = "0x80 - Software configure standby pin directly."]
    #[inline(always)]
    pub const fn sw_standby_cfg(&self) -> &SwStandbyCfg {
        &self.sw_standby_cfg
    }
    #[doc = "0x84 - Hardware configure standby pin."]
    #[inline(always)]
    pub const fn hw_cfg(&self) -> &HwCfg {
        &self.hw_cfg
    }
    #[doc = "0x88 - Configure standby counter."]
    #[inline(always)]
    pub const fn hw_standby_cnt(&self) -> &HwStandbyCnt {
        &self.hw_standby_cnt
    }
    #[doc = "0x8c - Configure idle interrupt counter."]
    #[inline(always)]
    pub const fn idle_intr_cnt(&self) -> &IdleIntrCnt {
        &self.idle_intr_cnt
    }
    #[doc = "0x90 - ECO configuration register."]
    #[inline(always)]
    pub const fn eco_cfg(&self) -> &EcoCfg {
        &self.eco_cfg
    }
    #[doc = "0x94 - Timestamp data register"]
    #[inline(always)]
    pub const fn timestamp_data(&self) -> &TimestampData {
        &self.timestamp_data
    }
    #[doc = "0x98 - Timestamp configuration register"]
    #[inline(always)]
    pub const fn timestamp_prescaler(&self) -> &TimestampPrescaler {
        &self.timestamp_prescaler
    }
    #[doc = "0x9c - Timestamp configuration register"]
    #[inline(always)]
    pub const fn timestamp_cfg(&self) -> &TimestampCfg {
        &self.timestamp_cfg
    }
}
#[doc = "MODE (rw) register accessor: TWAI mode register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "TWAI mode register."]
pub mod mode;
#[doc = "CMD (w) register accessor: TWAI command register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "TWAI command register."]
pub mod cmd;
#[doc = "STATUS (r) register accessor: TWAI status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "TWAI status register."]
pub mod status;
#[doc = "INTERRUPT (r) register accessor: Interrupt signals' register.\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`] module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Interrupt signals' register."]
pub mod interrupt;
#[doc = "INTERRUPT_ENABLE (rw) register accessor: Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`] module"]
#[doc(alias = "INTERRUPT_ENABLE")]
pub type InterruptEnable = crate::Reg<interrupt_enable::InterruptEnableSpec>;
#[doc = "Interrupt enable register."]
pub mod interrupt_enable;
#[doc = "BUS_TIMING_0 (rw) register accessor: Bit timing configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timing_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_timing_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timing_0`] module"]
#[doc(alias = "BUS_TIMING_0")]
pub type BusTiming0 = crate::Reg<bus_timing_0::BusTiming0Spec>;
#[doc = "Bit timing configuration register 0."]
pub mod bus_timing_0;
#[doc = "BUS_TIMING_1 (rw) register accessor: Bit timing configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timing_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_timing_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timing_1`] module"]
#[doc(alias = "BUS_TIMING_1")]
pub type BusTiming1 = crate::Reg<bus_timing_1::BusTiming1Spec>;
#[doc = "Bit timing configuration register 1."]
pub mod bus_timing_1;
#[doc = "ARB_LOST_CAP (r) register accessor: TWAI arbiter lost capture register.\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_lost_cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_lost_cap`] module"]
#[doc(alias = "ARB_LOST_CAP")]
pub type ArbLostCap = crate::Reg<arb_lost_cap::ArbLostCapSpec>;
#[doc = "TWAI arbiter lost capture register."]
pub mod arb_lost_cap;
#[doc = "ERR_CODE_CAP (r) register accessor: TWAI error info capture register.\n\nYou can [`read`](crate::Reg::read) this register and get [`err_code_cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_code_cap`] module"]
#[doc(alias = "ERR_CODE_CAP")]
pub type ErrCodeCap = crate::Reg<err_code_cap::ErrCodeCapSpec>;
#[doc = "TWAI error info capture register."]
pub mod err_code_cap;
#[doc = "ERR_WARNING_LIMIT (rw) register accessor: TWAI error threshold configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`err_warning_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_warning_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_warning_limit`] module"]
#[doc(alias = "ERR_WARNING_LIMIT")]
pub type ErrWarningLimit = crate::Reg<err_warning_limit::ErrWarningLimitSpec>;
#[doc = "TWAI error threshold configuration register."]
pub mod err_warning_limit;
#[doc = "RX_ERR_CNT (rw) register accessor: Rx error counter register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_err_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_err_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_cnt`] module"]
#[doc(alias = "RX_ERR_CNT")]
pub type RxErrCnt = crate::Reg<rx_err_cnt::RxErrCntSpec>;
#[doc = "Rx error counter register."]
pub mod rx_err_cnt;
#[doc = "TX_ERR_CNT (rw) register accessor: Tx error counter register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_err_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_err_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_err_cnt`] module"]
#[doc(alias = "TX_ERR_CNT")]
pub type TxErrCnt = crate::Reg<tx_err_cnt::TxErrCntSpec>;
#[doc = "Tx error counter register."]
pub mod tx_err_cnt;
#[doc = "DATA (rw) register accessor: Data register %s.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data register %s."]
pub mod data;
#[doc = "RX_MESSAGE_COUNTER (r) register accessor: Received message counter register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_message_counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_message_counter`] module"]
#[doc(alias = "RX_MESSAGE_COUNTER")]
pub type RxMessageCounter = crate::Reg<rx_message_counter::RxMessageCounterSpec>;
#[doc = "Received message counter register."]
pub mod rx_message_counter;
#[doc = "CLOCK_DIVIDER (rw) register accessor: Clock divider register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_divider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_divider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_divider`] module"]
#[doc(alias = "CLOCK_DIVIDER")]
pub type ClockDivider = crate::Reg<clock_divider::ClockDividerSpec>;
#[doc = "Clock divider register."]
pub mod clock_divider;
#[doc = "SW_STANDBY_CFG (rw) register accessor: Software configure standby pin directly.\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_standby_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_standby_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_standby_cfg`] module"]
#[doc(alias = "SW_STANDBY_CFG")]
pub type SwStandbyCfg = crate::Reg<sw_standby_cfg::SwStandbyCfgSpec>;
#[doc = "Software configure standby pin directly."]
pub mod sw_standby_cfg;
#[doc = "HW_CFG (rw) register accessor: Hardware configure standby pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_cfg`] module"]
#[doc(alias = "HW_CFG")]
pub type HwCfg = crate::Reg<hw_cfg::HwCfgSpec>;
#[doc = "Hardware configure standby pin."]
pub mod hw_cfg;
#[doc = "HW_STANDBY_CNT (rw) register accessor: Configure standby counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_standby_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_standby_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_standby_cnt`] module"]
#[doc(alias = "HW_STANDBY_CNT")]
pub type HwStandbyCnt = crate::Reg<hw_standby_cnt::HwStandbyCntSpec>;
#[doc = "Configure standby counter."]
pub mod hw_standby_cnt;
#[doc = "IDLE_INTR_CNT (rw) register accessor: Configure idle interrupt counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_intr_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_intr_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle_intr_cnt`] module"]
#[doc(alias = "IDLE_INTR_CNT")]
pub type IdleIntrCnt = crate::Reg<idle_intr_cnt::IdleIntrCntSpec>;
#[doc = "Configure idle interrupt counter."]
pub mod idle_intr_cnt;
#[doc = "ECO_CFG (rw) register accessor: ECO configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_cfg`] module"]
#[doc(alias = "ECO_CFG")]
pub type EcoCfg = crate::Reg<eco_cfg::EcoCfgSpec>;
#[doc = "ECO configuration register."]
pub mod eco_cfg;
#[doc = "TIMESTAMP_DATA (r) register accessor: Timestamp data register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_data`] module"]
#[doc(alias = "TIMESTAMP_DATA")]
pub type TimestampData = crate::Reg<timestamp_data::TimestampDataSpec>;
#[doc = "Timestamp data register"]
pub mod timestamp_data;
#[doc = "TIMESTAMP_PRESCALER (rw) register accessor: Timestamp configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_prescaler`] module"]
#[doc(alias = "TIMESTAMP_PRESCALER")]
pub type TimestampPrescaler = crate::Reg<timestamp_prescaler::TimestampPrescalerSpec>;
#[doc = "Timestamp configuration register"]
pub mod timestamp_prescaler;
#[doc = "TIMESTAMP_CFG (rw) register accessor: Timestamp configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_cfg`] module"]
#[doc(alias = "TIMESTAMP_CFG")]
pub type TimestampCfg = crate::Reg<timestamp_cfg::TimestampCfgSpec>;
#[doc = "Timestamp configuration register"]
pub mod timestamp_cfg;
