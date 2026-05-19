#[doc = "Register `PIN1` reader"]
pub type R = crate::R<Pin1Spec>;
#[doc = "Register `PIN1` writer"]
pub type W = crate::W<Pin1Spec>;
#[doc = "Field `REG_GPIO_PIN1_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin1WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN1_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin1WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN1_INT_TYPE` reader - Reserved"]
pub type RegGpioPin1IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN1_INT_TYPE` writer - Reserved"]
pub type RegGpioPin1IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN1_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin1PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN1_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin1PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI1_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi1Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin1_wakeup_enable(&self) -> RegGpioPin1WakeupEnableR {
        RegGpioPin1WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin1_int_type(&self) -> RegGpioPin1IntTypeR {
        RegGpioPin1IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin1_pad_driver(&self) -> RegGpioPin1PadDriverR {
        RegGpioPin1PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin1_wakeup_enable(&mut self) -> RegGpioPin1WakeupEnableW<'_, Pin1Spec> {
        RegGpioPin1WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin1_int_type(&mut self) -> RegGpioPin1IntTypeW<'_, Pin1Spec> {
        RegGpioPin1IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin1_pad_driver(&mut self) -> RegGpioPin1PadDriverW<'_, Pin1Spec> {
        RegGpioPin1PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi1_pin0_edge_wakeup_clr(&mut self) -> RegGpi1Pin0EdgeWakeupClrW<'_, Pin1Spec> {
        RegGpi1Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin1Spec;
impl crate::RegisterSpec for Pin1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin1::R`](R) reader structure"]
impl crate::Readable for Pin1Spec {}
#[doc = "`write(|w| ..)` method takes [`pin1::W`](W) writer structure"]
impl crate::Writable for Pin1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN1 to value 0"]
impl crate::Resettable for Pin1Spec {}
