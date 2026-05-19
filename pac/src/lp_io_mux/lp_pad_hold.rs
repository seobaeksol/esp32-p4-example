#[doc = "Register `LP_PAD_HOLD` reader"]
pub type R = crate::R<LpPadHoldSpec>;
#[doc = "Register `LP_PAD_HOLD` writer"]
pub type W = crate::W<LpPadHoldSpec>;
#[doc = "Field `REG_LP_GPIO_HOLD` reader - Reserved"]
pub type RegLpGpioHoldR = crate::FieldReader<u16>;
#[doc = "Field `REG_LP_GPIO_HOLD` writer - Reserved"]
pub type RegLpGpioHoldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_lp_gpio_hold(&self) -> RegLpGpioHoldR {
        RegLpGpioHoldR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_lp_gpio_hold(&mut self) -> RegLpGpioHoldW<'_, LpPadHoldSpec> {
        RegLpGpioHoldW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pad_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pad_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpPadHoldSpec;
impl crate::RegisterSpec for LpPadHoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_pad_hold::R`](R) reader structure"]
impl crate::Readable for LpPadHoldSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_pad_hold::W`](W) writer structure"]
impl crate::Writable for LpPadHoldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PAD_HOLD to value 0"]
impl crate::Resettable for LpPadHoldSpec {}
