#[doc = "Register `PIN0` reader"]
pub type R = crate::R<Pin0Spec>;
#[doc = "Register `PIN0` writer"]
pub type W = crate::W<Pin0Spec>;
#[doc = "Field `REG_GPIO_PIN0_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin0WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN0_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin0WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN0_INT_TYPE` reader - Reserved"]
pub type RegGpioPin0IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN0_INT_TYPE` writer - Reserved"]
pub type RegGpioPin0IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN0_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin0PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN0_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin0PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpioPin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin0_wakeup_enable(&self) -> RegGpioPin0WakeupEnableR {
        RegGpioPin0WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin0_int_type(&self) -> RegGpioPin0IntTypeR {
        RegGpioPin0IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin0_pad_driver(&self) -> RegGpioPin0PadDriverR {
        RegGpioPin0PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin0_wakeup_enable(&mut self) -> RegGpioPin0WakeupEnableW<'_, Pin0Spec> {
        RegGpioPin0WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin0_int_type(&mut self) -> RegGpioPin0IntTypeW<'_, Pin0Spec> {
        RegGpioPin0IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin0_pad_driver(&mut self) -> RegGpioPin0PadDriverW<'_, Pin0Spec> {
        RegGpioPin0PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpio_pin0_edge_wakeup_clr(&mut self) -> RegGpioPin0EdgeWakeupClrW<'_, Pin0Spec> {
        RegGpioPin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin0Spec;
impl crate::RegisterSpec for Pin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin0::R`](R) reader structure"]
impl crate::Readable for Pin0Spec {}
#[doc = "`write(|w| ..)` method takes [`pin0::W`](W) writer structure"]
impl crate::Writable for Pin0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN0 to value 0"]
impl crate::Resettable for Pin0Spec {}
