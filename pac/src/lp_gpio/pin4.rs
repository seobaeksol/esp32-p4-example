#[doc = "Register `PIN4` reader"]
pub type R = crate::R<Pin4Spec>;
#[doc = "Register `PIN4` writer"]
pub type W = crate::W<Pin4Spec>;
#[doc = "Field `REG_GPIO_PIN4_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin4WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN4_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin4WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN4_INT_TYPE` reader - Reserved"]
pub type RegGpioPin4IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN4_INT_TYPE` writer - Reserved"]
pub type RegGpioPin4IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN4_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin4PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN4_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin4PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI4_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi4Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin4_wakeup_enable(&self) -> RegGpioPin4WakeupEnableR {
        RegGpioPin4WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin4_int_type(&self) -> RegGpioPin4IntTypeR {
        RegGpioPin4IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin4_pad_driver(&self) -> RegGpioPin4PadDriverR {
        RegGpioPin4PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin4_wakeup_enable(&mut self) -> RegGpioPin4WakeupEnableW<'_, Pin4Spec> {
        RegGpioPin4WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin4_int_type(&mut self) -> RegGpioPin4IntTypeW<'_, Pin4Spec> {
        RegGpioPin4IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin4_pad_driver(&mut self) -> RegGpioPin4PadDriverW<'_, Pin4Spec> {
        RegGpioPin4PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi4_pin0_edge_wakeup_clr(&mut self) -> RegGpi4Pin0EdgeWakeupClrW<'_, Pin4Spec> {
        RegGpi4Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin4Spec;
impl crate::RegisterSpec for Pin4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin4::R`](R) reader structure"]
impl crate::Readable for Pin4Spec {}
#[doc = "`write(|w| ..)` method takes [`pin4::W`](W) writer structure"]
impl crate::Writable for Pin4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN4 to value 0"]
impl crate::Resettable for Pin4Spec {}
