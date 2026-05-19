#[doc = "Register `PIN6` reader"]
pub type R = crate::R<Pin6Spec>;
#[doc = "Register `PIN6` writer"]
pub type W = crate::W<Pin6Spec>;
#[doc = "Field `REG_GPIO_PIN6_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin6WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN6_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin6WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN6_INT_TYPE` reader - Reserved"]
pub type RegGpioPin6IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN6_INT_TYPE` writer - Reserved"]
pub type RegGpioPin6IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN6_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin6PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN6_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin6PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI6_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi6Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_wakeup_enable(&self) -> RegGpioPin6WakeupEnableR {
        RegGpioPin6WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_int_type(&self) -> RegGpioPin6IntTypeR {
        RegGpioPin6IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_pad_driver(&self) -> RegGpioPin6PadDriverR {
        RegGpioPin6PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_wakeup_enable(&mut self) -> RegGpioPin6WakeupEnableW<'_, Pin6Spec> {
        RegGpioPin6WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_int_type(&mut self) -> RegGpioPin6IntTypeW<'_, Pin6Spec> {
        RegGpioPin6IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_pad_driver(&mut self) -> RegGpioPin6PadDriverW<'_, Pin6Spec> {
        RegGpioPin6PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi6_pin0_edge_wakeup_clr(&mut self) -> RegGpi6Pin0EdgeWakeupClrW<'_, Pin6Spec> {
        RegGpi6Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin6Spec;
impl crate::RegisterSpec for Pin6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin6::R`](R) reader structure"]
impl crate::Readable for Pin6Spec {}
#[doc = "`write(|w| ..)` method takes [`pin6::W`](W) writer structure"]
impl crate::Writable for Pin6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN6 to value 0"]
impl crate::Resettable for Pin6Spec {}
