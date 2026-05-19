#[doc = "Register `CORE_TIMEOUT_INT_ST` reader"]
pub type R = crate::R<CoreTimeoutIntStSpec>;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 ahb timeout"]
pub type Core0AhbTimeoutIntStR = crate::BitReader;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 ahb timeout"]
pub type Core1AhbTimeoutIntStR = crate::BitReader;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 ibus timeout"]
pub type Core0IbusTimeoutIntStR = crate::BitReader;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 ibus timeout"]
pub type Core1IbusTimeoutIntStR = crate::BitReader;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 dbus timeout"]
pub type Core0DbusTimeoutIntStR = crate::BitReader;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 dbus timeout"]
pub type Core1DbusTimeoutIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the masked interrupt status of hp core0 ahb timeout"]
    #[inline(always)]
    pub fn core0_ahb_timeout_int_st(&self) -> Core0AhbTimeoutIntStR {
        Core0AhbTimeoutIntStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the masked interrupt status of hp core1 ahb timeout"]
    #[inline(always)]
    pub fn core1_ahb_timeout_int_st(&self) -> Core1AhbTimeoutIntStR {
        Core1AhbTimeoutIntStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the masked interrupt status of hp core0 ibus timeout"]
    #[inline(always)]
    pub fn core0_ibus_timeout_int_st(&self) -> Core0IbusTimeoutIntStR {
        Core0IbusTimeoutIntStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the masked interrupt status of hp core1 ibus timeout"]
    #[inline(always)]
    pub fn core1_ibus_timeout_int_st(&self) -> Core1IbusTimeoutIntStR {
        Core1IbusTimeoutIntStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the masked interrupt status of hp core0 dbus timeout"]
    #[inline(always)]
    pub fn core0_dbus_timeout_int_st(&self) -> Core0DbusTimeoutIntStR {
        Core0DbusTimeoutIntStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the masked interrupt status of hp core1 dbus timeout"]
    #[inline(always)]
    pub fn core1_dbus_timeout_int_st(&self) -> Core1DbusTimeoutIntStR {
        Core1DbusTimeoutIntStR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_timeout_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreTimeoutIntStSpec;
impl crate::RegisterSpec for CoreTimeoutIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_timeout_int_st::R`](R) reader structure"]
impl crate::Readable for CoreTimeoutIntStSpec {}
#[doc = "`reset()` method sets CORE_TIMEOUT_INT_ST to value 0"]
impl crate::Resettable for CoreTimeoutIntStSpec {}
