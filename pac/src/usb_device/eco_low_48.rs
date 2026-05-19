#[doc = "Register `ECO_LOW_48` reader"]
pub type R = crate::R<EcoLow48Spec>;
#[doc = "Register `ECO_LOW_48` writer"]
pub type W = crate::W<EcoLow48Spec>;
#[doc = "Field `RND_ECO_LOW_48` reader - Reserved."]
pub type RndEcoLow48R = crate::FieldReader<u32>;
#[doc = "Field `RND_ECO_LOW_48` writer - Reserved."]
pub type RndEcoLow48W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_low_48(&self) -> RndEcoLow48R {
        RndEcoLow48R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_low_48(&mut self) -> RndEcoLow48W<'_, EcoLow48Spec> {
        RndEcoLow48W::new(self, 0)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_low_48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_low_48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoLow48Spec;
impl crate::RegisterSpec for EcoLow48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_low_48::R`](R) reader structure"]
impl crate::Readable for EcoLow48Spec {}
#[doc = "`write(|w| ..)` method takes [`eco_low_48::W`](W) writer structure"]
impl crate::Writable for EcoLow48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_LOW_48 to value 0"]
impl crate::Resettable for EcoLow48Spec {}
