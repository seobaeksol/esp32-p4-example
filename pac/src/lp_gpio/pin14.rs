#[doc = "Register `PIN14` reader"]
pub type R = crate::R<Pin14Spec>;
#[doc = "Register `PIN14` writer"]
pub type W = crate::W<Pin14Spec>;
#[doc = "Field `REG_GPIO_PIN14_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin14WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN14_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin14WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN14_INT_TYPE` reader - Reserved"]
pub type RegGpioPin14IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN14_INT_TYPE` writer - Reserved"]
pub type RegGpioPin14IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN14_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin14PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN14_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin14PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI14_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi14Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin14_wakeup_enable(&self) -> RegGpioPin14WakeupEnableR {
        RegGpioPin14WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin14_int_type(&self) -> RegGpioPin14IntTypeR {
        RegGpioPin14IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin14_pad_driver(&self) -> RegGpioPin14PadDriverR {
        RegGpioPin14PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin14_wakeup_enable(&mut self) -> RegGpioPin14WakeupEnableW<'_, Pin14Spec> {
        RegGpioPin14WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin14_int_type(&mut self) -> RegGpioPin14IntTypeW<'_, Pin14Spec> {
        RegGpioPin14IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin14_pad_driver(&mut self) -> RegGpioPin14PadDriverW<'_, Pin14Spec> {
        RegGpioPin14PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi14_pin0_edge_wakeup_clr(&mut self) -> RegGpi14Pin0EdgeWakeupClrW<'_, Pin14Spec> {
        RegGpi14Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin14Spec;
impl crate::RegisterSpec for Pin14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin14::R`](R) reader structure"]
impl crate::Readable for Pin14Spec {}
#[doc = "`write(|w| ..)` method takes [`pin14::W`](W) writer structure"]
impl crate::Writable for Pin14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN14 to value 0"]
impl crate::Resettable for Pin14Spec {}
