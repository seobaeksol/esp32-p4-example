#[doc = "Register `PIN12` reader"]
pub type R = crate::R<Pin12Spec>;
#[doc = "Register `PIN12` writer"]
pub type W = crate::W<Pin12Spec>;
#[doc = "Field `REG_GPIO_PIN12_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin12WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN12_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin12WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN12_INT_TYPE` reader - Reserved"]
pub type RegGpioPin12IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN12_INT_TYPE` writer - Reserved"]
pub type RegGpioPin12IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN12_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin12PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN12_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin12PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI12_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi12Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin12_wakeup_enable(&self) -> RegGpioPin12WakeupEnableR {
        RegGpioPin12WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin12_int_type(&self) -> RegGpioPin12IntTypeR {
        RegGpioPin12IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin12_pad_driver(&self) -> RegGpioPin12PadDriverR {
        RegGpioPin12PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin12_wakeup_enable(&mut self) -> RegGpioPin12WakeupEnableW<'_, Pin12Spec> {
        RegGpioPin12WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin12_int_type(&mut self) -> RegGpioPin12IntTypeW<'_, Pin12Spec> {
        RegGpioPin12IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin12_pad_driver(&mut self) -> RegGpioPin12PadDriverW<'_, Pin12Spec> {
        RegGpioPin12PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi12_pin0_edge_wakeup_clr(&mut self) -> RegGpi12Pin0EdgeWakeupClrW<'_, Pin12Spec> {
        RegGpi12Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin12Spec;
impl crate::RegisterSpec for Pin12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin12::R`](R) reader structure"]
impl crate::Readable for Pin12Spec {}
#[doc = "`write(|w| ..)` method takes [`pin12::W`](W) writer structure"]
impl crate::Writable for Pin12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN12 to value 0"]
impl crate::Resettable for Pin12Spec {}
