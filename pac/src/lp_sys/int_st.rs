#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `LP_ADDRHOLE_INT_ST` reader - the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)"]
pub type LpAddrholeIntStR = crate::BitReader;
#[doc = "Field `IDBUS_ADDRHOLE_INT_ST` reader - the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
pub type IdbusAddrholeIntStR = crate::BitReader;
#[doc = "Field `LP_CORE_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of lp core ahb bus timeout"]
pub type LpCoreAhbTimeoutIntStR = crate::BitReader;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of lp core ibus timeout"]
pub type LpCoreIbusTimeoutIntStR = crate::BitReader;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of lp core dbus timeout"]
pub type LpCoreDbusTimeoutIntStR = crate::BitReader;
#[doc = "Field `ETM_TASK_ULP_INT_ST` reader - the masked interrupt status of etm task ulp"]
pub type EtmTaskUlpIntStR = crate::BitReader;
#[doc = "Field `SLOW_CLK_TICK_INT_ST` reader - the masked interrupt status of slow_clk_tick"]
pub type SlowClkTickIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)"]
    #[inline(always)]
    pub fn lp_addrhole_int_st(&self) -> LpAddrholeIntStR {
        LpAddrholeIntStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
    #[inline(always)]
    pub fn idbus_addrhole_int_st(&self) -> IdbusAddrholeIntStR {
        IdbusAddrholeIntStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the masked interrupt status of lp core ahb bus timeout"]
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_st(&self) -> LpCoreAhbTimeoutIntStR {
        LpCoreAhbTimeoutIntStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the masked interrupt status of lp core ibus timeout"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_st(&self) -> LpCoreIbusTimeoutIntStR {
        LpCoreIbusTimeoutIntStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the masked interrupt status of lp core dbus timeout"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_st(&self) -> LpCoreDbusTimeoutIntStR {
        LpCoreDbusTimeoutIntStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the masked interrupt status of etm task ulp"]
    #[inline(always)]
    pub fn etm_task_ulp_int_st(&self) -> EtmTaskUlpIntStR {
        EtmTaskUlpIntStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the masked interrupt status of slow_clk_tick"]
    #[inline(always)]
    pub fn slow_clk_tick_int_st(&self) -> SlowClkTickIntStR {
        SlowClkTickIntStR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
