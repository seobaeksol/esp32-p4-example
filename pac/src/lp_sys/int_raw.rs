#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Field `LP_ADDRHOLE_INT_RAW` reader - the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)"]
pub type LpAddrholeIntRawR = crate::BitReader;
#[doc = "Field `IDBUS_ADDRHOLE_INT_RAW` reader - the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
pub type IdbusAddrholeIntRawR = crate::BitReader;
#[doc = "Field `LP_CORE_AHB_TIMEOUT_INT_RAW` reader - the raw interrupt status of lp core ahb bus timeout"]
pub type LpCoreAhbTimeoutIntRawR = crate::BitReader;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of lp core ibus timeout"]
pub type LpCoreIbusTimeoutIntRawR = crate::BitReader;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of lp core dbus timeout"]
pub type LpCoreDbusTimeoutIntRawR = crate::BitReader;
#[doc = "Field `ETM_TASK_ULP_INT_RAW` reader - the raw interrupt status of etm task ulp"]
pub type EtmTaskUlpIntRawR = crate::BitReader;
#[doc = "Field `SLOW_CLK_TICK_INT_RAW` reader - the raw interrupt status of slow_clk_tick"]
pub type SlowClkTickIntRawR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)"]
    #[inline(always)]
    pub fn lp_addrhole_int_raw(&self) -> LpAddrholeIntRawR {
        LpAddrholeIntRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
    #[inline(always)]
    pub fn idbus_addrhole_int_raw(&self) -> IdbusAddrholeIntRawR {
        IdbusAddrholeIntRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the raw interrupt status of lp core ahb bus timeout"]
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_raw(&self) -> LpCoreAhbTimeoutIntRawR {
        LpCoreAhbTimeoutIntRawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the raw interrupt status of lp core ibus timeout"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_raw(&self) -> LpCoreIbusTimeoutIntRawR {
        LpCoreIbusTimeoutIntRawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the raw interrupt status of lp core dbus timeout"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_raw(&self) -> LpCoreDbusTimeoutIntRawR {
        LpCoreDbusTimeoutIntRawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the raw interrupt status of etm task ulp"]
    #[inline(always)]
    pub fn etm_task_ulp_int_raw(&self) -> EtmTaskUlpIntRawR {
        EtmTaskUlpIntRawR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the raw interrupt status of slow_clk_tick"]
    #[inline(always)]
    pub fn slow_clk_tick_int_raw(&self) -> SlowClkTickIntRawR {
        SlowClkTickIntRawR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "raw interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
