#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scl_low_period: SclLowPeriod,
    ctr: Ctr,
    sr: Sr,
    to: To,
    slave_addr: SlaveAddr,
    fifo_st: FifoSt,
    fifo_conf: FifoConf,
    data: Data,
    int_raw: IntRaw,
    int_clr: IntClr,
    int_ena: IntEna,
    int_st: IntSt,
    sda_hold: SdaHold,
    sda_sample: SdaSample,
    scl_high_period: SclHighPeriod,
    _reserved15: [u8; 0x04],
    scl_start_hold: SclStartHold,
    scl_rstart_setup: SclRstartSetup,
    scl_stop_hold: SclStopHold,
    scl_stop_setup: SclStopSetup,
    filter_cfg: FilterCfg,
    clk_conf: ClkConf,
    comd: [Comd; 8],
    scl_st_time_out: SclStTimeOut,
    scl_main_st_time_out: SclMainStTimeOut,
    scl_sp_conf: SclSpConf,
    scl_stretch_conf: SclStretchConf,
    _reserved26: [u8; 0x70],
    date: Date,
    _reserved27: [u8; 0x04],
    txfifo_start_addr: TxfifoStartAddr,
    _reserved28: [u8; 0x7c],
    rxfifo_start_addr: RxfifoStartAddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Configures the low level width of the SCL Clock"]
    #[inline(always)]
    pub const fn scl_low_period(&self) -> &SclLowPeriod {
        &self.scl_low_period
    }
    #[doc = "0x04 - Transmission setting"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x08 - Describe I2C work status"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - Setting time out control for receiving data"]
    #[inline(always)]
    pub const fn to(&self) -> &To {
        &self.to
    }
    #[doc = "0x10 - Local slave address setting"]
    #[inline(always)]
    pub const fn slave_addr(&self) -> &SlaveAddr {
        &self.slave_addr
    }
    #[doc = "0x14 - FIFO status register"]
    #[inline(always)]
    pub const fn fifo_st(&self) -> &FifoSt {
        &self.fifo_st
    }
    #[doc = "0x18 - FIFO configuration register"]
    #[inline(always)]
    pub const fn fifo_conf(&self) -> &FifoConf {
        &self.fifo_conf
    }
    #[doc = "0x1c - Rx FIFO read data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x20 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
    #[doc = "0x24 - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x28 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x2c - Status of captured I2C communication events"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x30 - Configures the hold time after a negative SCL edge"]
    #[inline(always)]
    pub const fn sda_hold(&self) -> &SdaHold {
        &self.sda_hold
    }
    #[doc = "0x34 - Configures the sample time after a positive SCL edge"]
    #[inline(always)]
    pub const fn sda_sample(&self) -> &SdaSample {
        &self.sda_sample
    }
    #[doc = "0x38 - Configures the high level width of SCL"]
    #[inline(always)]
    pub const fn scl_high_period(&self) -> &SclHighPeriod {
        &self.scl_high_period
    }
    #[doc = "0x40 - Configures the delay between the SDA and SCL negative edge for a start condition"]
    #[inline(always)]
    pub const fn scl_start_hold(&self) -> &SclStartHold {
        &self.scl_start_hold
    }
    #[doc = "0x44 - Configures the delay between the positive edge of SCL and the negative edge of SDA"]
    #[inline(always)]
    pub const fn scl_rstart_setup(&self) -> &SclRstartSetup {
        &self.scl_rstart_setup
    }
    #[doc = "0x48 - Configures the delay after the SCL clock edge for a stop condition"]
    #[inline(always)]
    pub const fn scl_stop_hold(&self) -> &SclStopHold {
        &self.scl_stop_hold
    }
    #[doc = "0x4c - Configures the delay between the SDA and SCL positive edge for a stop condition"]
    #[inline(always)]
    pub const fn scl_stop_setup(&self) -> &SclStopSetup {
        &self.scl_stop_setup
    }
    #[doc = "0x50 - SCL and SDA filter configuration register"]
    #[inline(always)]
    pub const fn filter_cfg(&self) -> &FilterCfg {
        &self.filter_cfg
    }
    #[doc = "0x54 - I2C CLK configuration register"]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &ClkConf {
        &self.clk_conf
    }
    #[doc = "0x58..0x78 - I2C command register %s"]
    #[inline(always)]
    pub const fn comd(&self, n: usize) -> &Comd {
        &self.comd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x78 - I2C command register %s"]
    #[inline(always)]
    pub fn comd_iter(&self) -> impl Iterator<Item = &Comd> {
        self.comd.iter()
    }
    #[doc = "0x78 - SCL status time out register"]
    #[inline(always)]
    pub const fn scl_st_time_out(&self) -> &SclStTimeOut {
        &self.scl_st_time_out
    }
    #[doc = "0x7c - SCL main status time out register"]
    #[inline(always)]
    pub const fn scl_main_st_time_out(&self) -> &SclMainStTimeOut {
        &self.scl_main_st_time_out
    }
    #[doc = "0x80 - Power configuration register"]
    #[inline(always)]
    pub const fn scl_sp_conf(&self) -> &SclSpConf {
        &self.scl_sp_conf
    }
    #[doc = "0x84 - Set SCL stretch of I2C slave"]
    #[inline(always)]
    pub const fn scl_stretch_conf(&self) -> &SclStretchConf {
        &self.scl_stretch_conf
    }
    #[doc = "0xf8 - Version register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x100 - I2C TXFIFO base address register"]
    #[inline(always)]
    pub const fn txfifo_start_addr(&self) -> &TxfifoStartAddr {
        &self.txfifo_start_addr
    }
    #[doc = "0x180 - I2C RXFIFO base address register"]
    #[inline(always)]
    pub const fn rxfifo_start_addr(&self) -> &RxfifoStartAddr {
        &self.rxfifo_start_addr
    }
}
#[doc = "SCL_LOW_PERIOD (rw) register accessor: Configures the low level width of the SCL Clock\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_low_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_low_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_low_period`] module"]
#[doc(alias = "SCL_LOW_PERIOD")]
pub type SclLowPeriod = crate::Reg<scl_low_period::SclLowPeriodSpec>;
#[doc = "Configures the low level width of the SCL Clock"]
pub mod scl_low_period;
#[doc = "CTR (rw) register accessor: Transmission setting\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`] module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Transmission setting"]
pub mod ctr;
#[doc = "SR (r) register accessor: Describe I2C work status\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Describe I2C work status"]
pub mod sr;
#[doc = "TO (rw) register accessor: Setting time out control for receiving data\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to`] module"]
#[doc(alias = "TO")]
pub type To = crate::Reg<to::ToSpec>;
#[doc = "Setting time out control for receiving data"]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: Local slave address setting\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_addr`] module"]
#[doc(alias = "SLAVE_ADDR")]
pub type SlaveAddr = crate::Reg<slave_addr::SlaveAddrSpec>;
#[doc = "Local slave address setting"]
pub mod slave_addr;
#[doc = "FIFO_ST (r) register accessor: FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_st`] module"]
#[doc(alias = "FIFO_ST")]
pub type FifoSt = crate::Reg<fifo_st::FifoStSpec>;
#[doc = "FIFO status register"]
pub mod fifo_st;
#[doc = "FIFO_CONF (rw) register accessor: FIFO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_conf`] module"]
#[doc(alias = "FIFO_CONF")]
pub type FifoConf = crate::Reg<fifo_conf::FifoConfSpec>;
#[doc = "FIFO configuration register"]
pub mod fifo_conf;
#[doc = "DATA (r) register accessor: Rx FIFO read data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Rx FIFO read data"]
pub mod data;
#[doc = "INT_RAW (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "INT_RAW")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_ST (r) register accessor: Status of captured I2C communication events\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "Status of captured I2C communication events"]
pub mod int_st;
#[doc = "SDA_HOLD (rw) register accessor: Configures the hold time after a negative SCL edge\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_hold`] module"]
#[doc(alias = "SDA_HOLD")]
pub type SdaHold = crate::Reg<sda_hold::SdaHoldSpec>;
#[doc = "Configures the hold time after a negative SCL edge"]
pub mod sda_hold;
#[doc = "SDA_SAMPLE (rw) register accessor: Configures the sample time after a positive SCL edge\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_sample::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_sample::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_sample`] module"]
#[doc(alias = "SDA_SAMPLE")]
pub type SdaSample = crate::Reg<sda_sample::SdaSampleSpec>;
#[doc = "Configures the sample time after a positive SCL edge"]
pub mod sda_sample;
#[doc = "SCL_HIGH_PERIOD (rw) register accessor: Configures the high level width of SCL\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_high_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_high_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_high_period`] module"]
#[doc(alias = "SCL_HIGH_PERIOD")]
pub type SclHighPeriod = crate::Reg<scl_high_period::SclHighPeriodSpec>;
#[doc = "Configures the high level width of SCL"]
pub mod scl_high_period;
#[doc = "SCL_START_HOLD (rw) register accessor: Configures the delay between the SDA and SCL negative edge for a start condition\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_start_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_start_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_start_hold`] module"]
#[doc(alias = "SCL_START_HOLD")]
pub type SclStartHold = crate::Reg<scl_start_hold::SclStartHoldSpec>;
#[doc = "Configures the delay between the SDA and SCL negative edge for a start condition"]
pub mod scl_start_hold;
#[doc = "SCL_RSTART_SETUP (rw) register accessor: Configures the delay between the positive edge of SCL and the negative edge of SDA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_rstart_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_rstart_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_rstart_setup`] module"]
#[doc(alias = "SCL_RSTART_SETUP")]
pub type SclRstartSetup = crate::Reg<scl_rstart_setup::SclRstartSetupSpec>;
#[doc = "Configures the delay between the positive edge of SCL and the negative edge of SDA"]
pub mod scl_rstart_setup;
#[doc = "SCL_STOP_HOLD (rw) register accessor: Configures the delay after the SCL clock edge for a stop condition\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_stop_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_stop_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_stop_hold`] module"]
#[doc(alias = "SCL_STOP_HOLD")]
pub type SclStopHold = crate::Reg<scl_stop_hold::SclStopHoldSpec>;
#[doc = "Configures the delay after the SCL clock edge for a stop condition"]
pub mod scl_stop_hold;
#[doc = "SCL_STOP_SETUP (rw) register accessor: Configures the delay between the SDA and SCL positive edge for a stop condition\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_stop_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_stop_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_stop_setup`] module"]
#[doc(alias = "SCL_STOP_SETUP")]
pub type SclStopSetup = crate::Reg<scl_stop_setup::SclStopSetupSpec>;
#[doc = "Configures the delay between the SDA and SCL positive edge for a stop condition"]
pub mod scl_stop_setup;
#[doc = "FILTER_CFG (rw) register accessor: SCL and SDA filter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_cfg`] module"]
#[doc(alias = "FILTER_CFG")]
pub type FilterCfg = crate::Reg<filter_cfg::FilterCfgSpec>;
#[doc = "SCL and SDA filter configuration register"]
pub mod filter_cfg;
#[doc = "CLK_CONF (rw) register accessor: I2C CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
#[doc(alias = "CLK_CONF")]
pub type ClkConf = crate::Reg<clk_conf::ClkConfSpec>;
#[doc = "I2C CLK configuration register"]
pub mod clk_conf;
#[doc = "COMD (rw) register accessor: I2C command register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`comd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comd`] module"]
#[doc(alias = "COMD")]
pub type Comd = crate::Reg<comd::ComdSpec>;
#[doc = "I2C command register %s"]
pub mod comd;
#[doc = "SCL_ST_TIME_OUT (rw) register accessor: SCL status time out register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_st_time_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_st_time_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_st_time_out`] module"]
#[doc(alias = "SCL_ST_TIME_OUT")]
pub type SclStTimeOut = crate::Reg<scl_st_time_out::SclStTimeOutSpec>;
#[doc = "SCL status time out register"]
pub mod scl_st_time_out;
#[doc = "SCL_MAIN_ST_TIME_OUT (rw) register accessor: SCL main status time out register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_main_st_time_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_main_st_time_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_main_st_time_out`] module"]
#[doc(alias = "SCL_MAIN_ST_TIME_OUT")]
pub type SclMainStTimeOut = crate::Reg<scl_main_st_time_out::SclMainStTimeOutSpec>;
#[doc = "SCL main status time out register"]
pub mod scl_main_st_time_out;
#[doc = "SCL_SP_CONF (rw) register accessor: Power configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_sp_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_sp_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_sp_conf`] module"]
#[doc(alias = "SCL_SP_CONF")]
pub type SclSpConf = crate::Reg<scl_sp_conf::SclSpConfSpec>;
#[doc = "Power configuration register"]
pub mod scl_sp_conf;
#[doc = "SCL_STRETCH_CONF (rw) register accessor: Set SCL stretch of I2C slave\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_stretch_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_stretch_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_stretch_conf`] module"]
#[doc(alias = "SCL_STRETCH_CONF")]
pub type SclStretchConf = crate::Reg<scl_stretch_conf::SclStretchConfSpec>;
#[doc = "Set SCL stretch of I2C slave"]
pub mod scl_stretch_conf;
#[doc = "DATE (rw) register accessor: Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version register"]
pub mod date;
#[doc = "TXFIFO_START_ADDR (r) register accessor: I2C TXFIFO base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifo_start_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifo_start_addr`] module"]
#[doc(alias = "TXFIFO_START_ADDR")]
pub type TxfifoStartAddr = crate::Reg<txfifo_start_addr::TxfifoStartAddrSpec>;
#[doc = "I2C TXFIFO base address register"]
pub mod txfifo_start_addr;
#[doc = "RXFIFO_START_ADDR (r) register accessor: I2C RXFIFO base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifo_start_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifo_start_addr`] module"]
#[doc(alias = "RXFIFO_START_ADDR")]
pub type RxfifoStartAddr = crate::Reg<rxfifo_start_addr::RxfifoStartAddrSpec>;
#[doc = "I2C RXFIFO base address register"]
pub mod rxfifo_start_addr;
