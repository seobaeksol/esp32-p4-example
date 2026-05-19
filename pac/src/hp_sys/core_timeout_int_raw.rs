#[doc = "Register `CORE_TIMEOUT_INT_RAW` reader"]
pub type R = crate::R<CoreTimeoutIntRawSpec>;
#[doc = "Register `CORE_TIMEOUT_INT_RAW` writer"]
pub type W = crate::W<CoreTimeoutIntRawSpec>;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core0 ahb timeout"]
pub type Core0AhbTimeoutIntRawR = crate::BitReader;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core0 ahb timeout"]
pub type Core0AhbTimeoutIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core1 ahb timeout"]
pub type Core1AhbTimeoutIntRawR = crate::BitReader;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core1 ahb timeout"]
pub type Core1AhbTimeoutIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core0 ibus timeout"]
pub type Core0IbusTimeoutIntRawR = crate::BitReader;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core0 ibus timeout"]
pub type Core0IbusTimeoutIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core1 ibus timeout"]
pub type Core1IbusTimeoutIntRawR = crate::BitReader;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core1 ibus timeout"]
pub type Core1IbusTimeoutIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core0 dbus timeout"]
pub type Core0DbusTimeoutIntRawR = crate::BitReader;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core0 dbus timeout"]
pub type Core0DbusTimeoutIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core1 dbus timeout"]
pub type Core1DbusTimeoutIntRawR = crate::BitReader;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core1 dbus timeout"]
pub type Core1DbusTimeoutIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of hp core0 ahb timeout"]
    #[inline(always)]
    pub fn core0_ahb_timeout_int_raw(&self) -> Core0AhbTimeoutIntRawR {
        Core0AhbTimeoutIntRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of hp core1 ahb timeout"]
    #[inline(always)]
    pub fn core1_ahb_timeout_int_raw(&self) -> Core1AhbTimeoutIntRawR {
        Core1AhbTimeoutIntRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the raw interrupt status of hp core0 ibus timeout"]
    #[inline(always)]
    pub fn core0_ibus_timeout_int_raw(&self) -> Core0IbusTimeoutIntRawR {
        Core0IbusTimeoutIntRawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the raw interrupt status of hp core1 ibus timeout"]
    #[inline(always)]
    pub fn core1_ibus_timeout_int_raw(&self) -> Core1IbusTimeoutIntRawR {
        Core1IbusTimeoutIntRawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the raw interrupt status of hp core0 dbus timeout"]
    #[inline(always)]
    pub fn core0_dbus_timeout_int_raw(&self) -> Core0DbusTimeoutIntRawR {
        Core0DbusTimeoutIntRawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the raw interrupt status of hp core1 dbus timeout"]
    #[inline(always)]
    pub fn core1_dbus_timeout_int_raw(&self) -> Core1DbusTimeoutIntRawR {
        Core1DbusTimeoutIntRawR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the raw interrupt status of hp core0 ahb timeout"]
    #[inline(always)]
    pub fn core0_ahb_timeout_int_raw(
        &mut self,
    ) -> Core0AhbTimeoutIntRawW<'_, CoreTimeoutIntRawSpec> {
        Core0AhbTimeoutIntRawW::new(self, 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of hp core1 ahb timeout"]
    #[inline(always)]
    pub fn core1_ahb_timeout_int_raw(
        &mut self,
    ) -> Core1AhbTimeoutIntRawW<'_, CoreTimeoutIntRawSpec> {
        Core1AhbTimeoutIntRawW::new(self, 1)
    }
    #[doc = "Bit 2 - the raw interrupt status of hp core0 ibus timeout"]
    #[inline(always)]
    pub fn core0_ibus_timeout_int_raw(
        &mut self,
    ) -> Core0IbusTimeoutIntRawW<'_, CoreTimeoutIntRawSpec> {
        Core0IbusTimeoutIntRawW::new(self, 2)
    }
    #[doc = "Bit 3 - the raw interrupt status of hp core1 ibus timeout"]
    #[inline(always)]
    pub fn core1_ibus_timeout_int_raw(
        &mut self,
    ) -> Core1IbusTimeoutIntRawW<'_, CoreTimeoutIntRawSpec> {
        Core1IbusTimeoutIntRawW::new(self, 3)
    }
    #[doc = "Bit 4 - the raw interrupt status of hp core0 dbus timeout"]
    #[inline(always)]
    pub fn core0_dbus_timeout_int_raw(
        &mut self,
    ) -> Core0DbusTimeoutIntRawW<'_, CoreTimeoutIntRawSpec> {
        Core0DbusTimeoutIntRawW::new(self, 4)
    }
    #[doc = "Bit 5 - the raw interrupt status of hp core1 dbus timeout"]
    #[inline(always)]
    pub fn core1_dbus_timeout_int_raw(
        &mut self,
    ) -> Core1DbusTimeoutIntRawW<'_, CoreTimeoutIntRawSpec> {
        Core1DbusTimeoutIntRawW::new(self, 5)
    }
}
#[doc = "Hp core bus timeout interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_timeout_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_timeout_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreTimeoutIntRawSpec;
impl crate::RegisterSpec for CoreTimeoutIntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_timeout_int_raw::R`](R) reader structure"]
impl crate::Readable for CoreTimeoutIntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`core_timeout_int_raw::W`](W) writer structure"]
impl crate::Writable for CoreTimeoutIntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_TIMEOUT_INT_RAW to value 0"]
impl crate::Resettable for CoreTimeoutIntRawSpec {}
