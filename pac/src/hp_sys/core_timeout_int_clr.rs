#[doc = "Register `CORE_TIMEOUT_INT_CLR` writer"]
pub type W = crate::W<CoreTimeoutIntClrSpec>;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_ahb_timeout int"]
pub type Core0AhbTimeoutIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_ahb_timeout int"]
pub type Core1AhbTimeoutIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_ibus_timeout int"]
pub type Core0IbusTimeoutIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_ibus_timeout int"]
pub type Core1IbusTimeoutIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_dbus_timeout int"]
pub type Core0DbusTimeoutIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_dbus_timeout int"]
pub type Core1DbusTimeoutIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear hp_core0_ahb_timeout int"]
    #[inline(always)]
    pub fn core0_ahb_timeout_int_clr(
        &mut self,
    ) -> Core0AhbTimeoutIntClrW<'_, CoreTimeoutIntClrSpec> {
        Core0AhbTimeoutIntClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear hp_core1_ahb_timeout int"]
    #[inline(always)]
    pub fn core1_ahb_timeout_int_clr(
        &mut self,
    ) -> Core1AhbTimeoutIntClrW<'_, CoreTimeoutIntClrSpec> {
        Core1AhbTimeoutIntClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear hp_core0_ibus_timeout int"]
    #[inline(always)]
    pub fn core0_ibus_timeout_int_clr(
        &mut self,
    ) -> Core0IbusTimeoutIntClrW<'_, CoreTimeoutIntClrSpec> {
        Core0IbusTimeoutIntClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear hp_core1_ibus_timeout int"]
    #[inline(always)]
    pub fn core1_ibus_timeout_int_clr(
        &mut self,
    ) -> Core1IbusTimeoutIntClrW<'_, CoreTimeoutIntClrSpec> {
        Core1IbusTimeoutIntClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear hp_core0_dbus_timeout int"]
    #[inline(always)]
    pub fn core0_dbus_timeout_int_clr(
        &mut self,
    ) -> Core0DbusTimeoutIntClrW<'_, CoreTimeoutIntClrSpec> {
        Core0DbusTimeoutIntClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear hp_core1_dbus_timeout int"]
    #[inline(always)]
    pub fn core1_dbus_timeout_int_clr(
        &mut self,
    ) -> Core1DbusTimeoutIntClrW<'_, CoreTimeoutIntClrSpec> {
        Core1DbusTimeoutIntClrW::new(self, 5)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_timeout_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreTimeoutIntClrSpec;
impl crate::RegisterSpec for CoreTimeoutIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_timeout_int_clr::W`](W) writer structure"]
impl crate::Writable for CoreTimeoutIntClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_TIMEOUT_INT_CLR to value 0"]
impl crate::Resettable for CoreTimeoutIntClrSpec {}
