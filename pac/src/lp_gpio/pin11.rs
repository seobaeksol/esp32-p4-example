#[doc = "Register `PIN11` reader"]
pub type R = crate::R<Pin11Spec>;
#[doc = "Register `PIN11` writer"]
pub type W = crate::W<Pin11Spec>;
#[doc = "Field `REG_GPIO_PIN11_WAKEUP_ENABLE` reader - Reserved"]
pub type RegGpioPin11WakeupEnableR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN11_WAKEUP_ENABLE` writer - Reserved"]
pub type RegGpioPin11WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN11_INT_TYPE` reader - Reserved"]
pub type RegGpioPin11IntTypeR = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN11_INT_TYPE` writer - Reserved"]
pub type RegGpioPin11IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN11_PAD_DRIVER` reader - Reserved"]
pub type RegGpioPin11PadDriverR = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN11_PAD_DRIVER` writer - Reserved"]
pub type RegGpioPin11PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI11_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type RegGpi11Pin0EdgeWakeupClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin11_wakeup_enable(&self) -> RegGpioPin11WakeupEnableR {
        RegGpioPin11WakeupEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin11_int_type(&self) -> RegGpioPin11IntTypeR {
        RegGpioPin11IntTypeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin11_pad_driver(&self) -> RegGpioPin11PadDriverR {
        RegGpioPin11PadDriverR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin11_wakeup_enable(&mut self) -> RegGpioPin11WakeupEnableW<'_, Pin11Spec> {
        RegGpioPin11WakeupEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin11_int_type(&mut self) -> RegGpioPin11IntTypeW<'_, Pin11Spec> {
        RegGpioPin11IntTypeW::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin11_pad_driver(&mut self) -> RegGpioPin11PadDriverW<'_, Pin11Spec> {
        RegGpioPin11PadDriverW::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    pub fn reg_gpi11_pin0_edge_wakeup_clr(&mut self) -> RegGpi11Pin0EdgeWakeupClrW<'_, Pin11Spec> {
        RegGpi11Pin0EdgeWakeupClrW::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pin11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin11Spec;
impl crate::RegisterSpec for Pin11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin11::R`](R) reader structure"]
impl crate::Readable for Pin11Spec {}
#[doc = "`write(|w| ..)` method takes [`pin11::W`](W) writer structure"]
impl crate::Writable for Pin11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN11 to value 0"]
impl crate::Resettable for Pin11Spec {}
