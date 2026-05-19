#[doc = "Register `PIN5` reader"]
pub type R = crate::R<Pin5Spec>;
#[doc = "Register `PIN5` writer"]
pub type W = crate::W<Pin5Spec>;
#[doc = "Field `REG_GPIO_PIN5_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin5WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN5_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin5WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN5_INT_TYPE` reader - Reserved"]
pub type RegGpioPin5IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN5_INT_TYPE` writer - Reserved"]
pub type RegGpioPin5IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN5_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin5PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN5_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin5PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI5_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi5Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin5_wakeup_enable(&self) -> RegGpioPin5WakeupEnableR {
        RegGpioPin5WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin5_int_type(&self) -> RegGpioPin5IntTypeR {
        RegGpioPin5IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin5_pad_driver(&self) -> RegGpioPin5PadDriverR {
        RegGpioPin5PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin5_wakeup_enable(&mut self) -> RegGpioPin5WakeupEnableW<'_, Pin5Spec> {
        RegGpioPin5WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin5_int_type(&mut self) -> RegGpioPin5IntTypeW<'_, Pin5Spec> {
        RegGpioPin5IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin5_pad_driver(&mut self) -> RegGpioPin5PadDriverW<'_, Pin5Spec> {
        RegGpioPin5PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi5_pin0_edge_wakeup_clr(&mut self) -> RegGpi5Pin0EdgeWakeupClrW<'_, Pin5Spec> {
        RegGpi5Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin5Spec;
impl crate::RegisterSpec for Pin5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin5::R`](R) reader structure"]
impl crate::Readable for Pin5Spec {}
#[doc = "`write(|w| ..)` method takes [`pin5::W`](W) writer structure"]
impl crate::Writable for Pin5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN5 to value 0"]
impl crate::Resettable for Pin5Spec {}
