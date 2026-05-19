#[doc = "Register `PIN10` reader"]
pub type R = crate::R<Pin10Spec>;
#[doc = "Register `PIN10` writer"]
pub type W = crate::W<Pin10Spec>;
#[doc = "Field `REG_GPIO_PIN10_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin10WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN10_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin10WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN10_INT_TYPE` reader - Reserved"]
pub type RegGpioPin10IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN10_INT_TYPE` writer - Reserved"]
pub type RegGpioPin10IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN10_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin10PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN10_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin10PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI10_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi10Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin10_wakeup_enable(&self) -> RegGpioPin10WakeupEnableR {
        RegGpioPin10WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin10_int_type(&self) -> RegGpioPin10IntTypeR {
        RegGpioPin10IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin10_pad_driver(&self) -> RegGpioPin10PadDriverR {
        RegGpioPin10PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin10_wakeup_enable(&mut self) -> RegGpioPin10WakeupEnableW<'_, Pin10Spec> {
        RegGpioPin10WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin10_int_type(&mut self) -> RegGpioPin10IntTypeW<'_, Pin10Spec> {
        RegGpioPin10IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin10_pad_driver(&mut self) -> RegGpioPin10PadDriverW<'_, Pin10Spec> {
        RegGpioPin10PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi10_pin0_edge_wakeup_clr(&mut self) -> RegGpi10Pin0EdgeWakeupClrW<'_, Pin10Spec> {
        RegGpi10Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin10Spec;
impl crate::RegisterSpec for Pin10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin10::R`](R) reader structure"]
impl crate::Readable for Pin10Spec {}
#[doc = "`write(|w| ..)` method takes [`pin10::W`](W) writer structure"]
impl crate::Writable for Pin10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN10 to value 0"]
impl crate::Resettable for Pin10Spec {}
