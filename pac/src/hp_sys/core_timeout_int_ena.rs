#[doc = "Register `CORE_TIMEOUT_INT_ENA` reader"]
pub type R = crate::R<CoreTimeoutIntEnaSpec>;
#[doc = "Register `CORE_TIMEOUT_INT_ENA` writer"]
pub type W = crate::W<CoreTimeoutIntEnaSpec>;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_ahb_timeout int"]
pub type Core0AhbTimeoutIntEnaR = crate::BitReader;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_ahb_timeout int"]
pub type Core0AhbTimeoutIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_ahb_timeout int"]
pub type Core1AhbTimeoutIntEnaR = crate::BitReader;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_ahb_timeout int"]
pub type Core1AhbTimeoutIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_ibus_timeout int"]
pub type Core0IbusTimeoutIntEnaR = crate::BitReader;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_ibus_timeout int"]
pub type Core0IbusTimeoutIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_ibus_timeout int"]
pub type Core1IbusTimeoutIntEnaR = crate::BitReader;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_ibus_timeout int"]
pub type Core1IbusTimeoutIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_dbus_timeout int"]
pub type Core0DbusTimeoutIntEnaR = crate::BitReader;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_dbus_timeout int"]
pub type Core0DbusTimeoutIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_dbus_timeout int"]
pub type Core1DbusTimeoutIntEnaR = crate::BitReader;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_dbus_timeout int"]
pub type Core1DbusTimeoutIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable hp_core0_ahb_timeout int"]
    #[inline(always)]
    pub fn core0_ahb_timeout_int_ena(&self) -> Core0AhbTimeoutIntEnaR {
        Core0AhbTimeoutIntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable hp_core1_ahb_timeout int"]
    #[inline(always)]
    pub fn core1_ahb_timeout_int_ena(&self) -> Core1AhbTimeoutIntEnaR {
        Core1AhbTimeoutIntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable hp_core0_ibus_timeout int"]
    #[inline(always)]
    pub fn core0_ibus_timeout_int_ena(&self) -> Core0IbusTimeoutIntEnaR {
        Core0IbusTimeoutIntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable hp_core1_ibus_timeout int"]
    #[inline(always)]
    pub fn core1_ibus_timeout_int_ena(&self) -> Core1IbusTimeoutIntEnaR {
        Core1IbusTimeoutIntEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable hp_core0_dbus_timeout int"]
    #[inline(always)]
    pub fn core0_dbus_timeout_int_ena(&self) -> Core0DbusTimeoutIntEnaR {
        Core0DbusTimeoutIntEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable hp_core1_dbus_timeout int"]
    #[inline(always)]
    pub fn core1_dbus_timeout_int_ena(&self) -> Core1DbusTimeoutIntEnaR {
        Core1DbusTimeoutIntEnaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable hp_core0_ahb_timeout int"]
    #[inline(always)]
    pub fn core0_ahb_timeout_int_ena(
        &mut self,
    ) -> Core0AhbTimeoutIntEnaW<'_, CoreTimeoutIntEnaSpec> {
        Core0AhbTimeoutIntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable hp_core1_ahb_timeout int"]
    #[inline(always)]
    pub fn core1_ahb_timeout_int_ena(
        &mut self,
    ) -> Core1AhbTimeoutIntEnaW<'_, CoreTimeoutIntEnaSpec> {
        Core1AhbTimeoutIntEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable hp_core0_ibus_timeout int"]
    #[inline(always)]
    pub fn core0_ibus_timeout_int_ena(
        &mut self,
    ) -> Core0IbusTimeoutIntEnaW<'_, CoreTimeoutIntEnaSpec> {
        Core0IbusTimeoutIntEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable hp_core1_ibus_timeout int"]
    #[inline(always)]
    pub fn core1_ibus_timeout_int_ena(
        &mut self,
    ) -> Core1IbusTimeoutIntEnaW<'_, CoreTimeoutIntEnaSpec> {
        Core1IbusTimeoutIntEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable hp_core0_dbus_timeout int"]
    #[inline(always)]
    pub fn core0_dbus_timeout_int_ena(
        &mut self,
    ) -> Core0DbusTimeoutIntEnaW<'_, CoreTimeoutIntEnaSpec> {
        Core0DbusTimeoutIntEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable hp_core1_dbus_timeout int"]
    #[inline(always)]
    pub fn core1_dbus_timeout_int_ena(
        &mut self,
    ) -> Core1DbusTimeoutIntEnaW<'_, CoreTimeoutIntEnaSpec> {
        Core1DbusTimeoutIntEnaW::new(self, 5)
    }
}
#[doc = "masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_timeout_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_timeout_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreTimeoutIntEnaSpec;
impl crate::RegisterSpec for CoreTimeoutIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_timeout_int_ena::R`](R) reader structure"]
impl crate::Readable for CoreTimeoutIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`core_timeout_int_ena::W`](W) writer structure"]
impl crate::Writable for CoreTimeoutIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_TIMEOUT_INT_ENA to value 0"]
impl crate::Resettable for CoreTimeoutIntEnaSpec {}
