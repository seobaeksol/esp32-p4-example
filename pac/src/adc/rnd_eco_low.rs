#[doc = "Register `RND_ECO_LOW` reader"]
pub type R = crate::R<RndEcoLowSpec>;
#[doc = "Register `RND_ECO_LOW` writer"]
pub type W = crate::W<RndEcoLowSpec>;
#[doc = "Field `RND_ECO_LOW` reader - rnd eco low"]
pub type RndEcoLowR = crate::FieldReader<u32>;
#[doc = "Field `RND_ECO_LOW` writer - rnd eco low"]
pub type RndEcoLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - rnd eco low"]
    #[inline(always)]
    pub fn rnd_eco_low(&self) -> RndEcoLowR {
        RndEcoLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rnd eco low"]
    #[inline(always)]
    pub fn rnd_eco_low(&mut self) -> RndEcoLowW<'_, RndEcoLowSpec> {
        RndEcoLowW::new(self, 0)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RndEcoLowSpec;
impl crate::RegisterSpec for RndEcoLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_eco_low::R`](R) reader structure"]
impl crate::Readable for RndEcoLowSpec {}
#[doc = "`write(|w| ..)` method takes [`rnd_eco_low::W`](W) writer structure"]
impl crate::Writable for RndEcoLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RND_ECO_LOW to value 0"]
impl crate::Resettable for RndEcoLowSpec {}
