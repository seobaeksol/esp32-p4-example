#[doc = "Register `ECO_LOW` reader"]
pub type R = crate::R<EcoLowSpec>;
#[doc = "Register `ECO_LOW` writer"]
pub type W = crate::W<EcoLowSpec>;
#[doc = "Field `RND_ECO_LOW` reader - Reserved."]
pub type RndEcoLowR = crate::FieldReader<u32>;
#[doc = "Field `RND_ECO_LOW` writer - Reserved."]
pub type RndEcoLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_low(&self) -> RndEcoLowR {
        RndEcoLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_low(&mut self) -> RndEcoLowW<'_, EcoLowSpec> {
        RndEcoLowW::new(self, 0)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoLowSpec;
impl crate::RegisterSpec for EcoLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_low::R`](R) reader structure"]
impl crate::Readable for EcoLowSpec {}
#[doc = "`write(|w| ..)` method takes [`eco_low::W`](W) writer structure"]
impl crate::Writable for EcoLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_LOW to value 0"]
impl crate::Resettable for EcoLowSpec {}
