#[doc = "Register `PIN7` reader"]
pub type R = crate::R<Pin7Spec>;
#[doc = "Register `PIN7` writer"]
pub type W = crate::W<Pin7Spec>;
#[doc = "Field `REG_GPIO_PIN7_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin7WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN7_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin7WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN7_INT_TYPE` reader - Reserved"]
pub type RegGpioPin7IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN7_INT_TYPE` writer - Reserved"]
pub type RegGpioPin7IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN7_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin7PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN7_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin7PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI7_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi7Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin7_wakeup_enable(&self) -> RegGpioPin7WakeupEnableR {
        RegGpioPin7WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin7_int_type(&self) -> RegGpioPin7IntTypeR {
        RegGpioPin7IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin7_pad_driver(&self) -> RegGpioPin7PadDriverR {
        RegGpioPin7PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin7_wakeup_enable(&mut self) -> RegGpioPin7WakeupEnableW<'_, Pin7Spec> {
        RegGpioPin7WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin7_int_type(&mut self) -> RegGpioPin7IntTypeW<'_, Pin7Spec> {
        RegGpioPin7IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin7_pad_driver(&mut self) -> RegGpioPin7PadDriverW<'_, Pin7Spec> {
        RegGpioPin7PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi7_pin0_edge_wakeup_clr(&mut self) -> RegGpi7Pin0EdgeWakeupClrW<'_, Pin7Spec> {
        RegGpi7Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin7Spec;
impl crate::RegisterSpec for Pin7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin7::R`](R) reader structure"]
impl crate::Readable for Pin7Spec {}
#[doc = "`write(|w| ..)` method takes [`pin7::W`](W) writer structure"]
impl crate::Writable for Pin7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN7 to value 0"]
impl crate::Resettable for Pin7Spec {}
