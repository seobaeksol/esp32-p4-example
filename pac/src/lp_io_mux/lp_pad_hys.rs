#[doc = "Register `LP_PAD_HYS` reader"]
pub type R = crate::R<LpPadHysSpec>;
#[doc = "Register `LP_PAD_HYS` writer"]
pub type W = crate::W<LpPadHysSpec>;
#[doc = "Field `REG_LP_GPIO_HYS` reader - Reserved"]
pub type RegLpGpioHysR = crate::FieldReader<u16>;
#[doc = "Field `REG_LP_GPIO_HYS` writer - Reserved"]
pub type RegLpGpioHysW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_lp_gpio_hys(&self) -> RegLpGpioHysR {
        RegLpGpioHysR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_lp_gpio_hys(&mut self) -> RegLpGpioHysW<'_, LpPadHysSpec> {
        RegLpGpioHysW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pad_hys::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pad_hys::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpPadHysSpec;
impl crate::RegisterSpec for LpPadHysSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_pad_hys::R`](R) reader structure"]
impl crate::Readable for LpPadHysSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_pad_hys::W`](W) writer structure"]
impl crate::Writable for LpPadHysSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PAD_HYS to value 0"]
impl crate::Resettable for LpPadHysSpec {}
