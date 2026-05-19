#[doc = "Register `RND_ECO_CS` reader"]
pub type R = crate::R<RndEcoCsSpec>;
#[doc = "Register `RND_ECO_CS` writer"]
pub type W = crate::W<RndEcoCsSpec>;
#[doc = "Field `RND_ECO_EN` reader - N/A"]
pub type RndEcoEnR = crate::BitReader;
#[doc = "Field `RND_ECO_EN` writer - N/A"]
pub type RndEcoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RND_ECO_RESULT` reader - N/A"]
pub type RndEcoResultR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn rnd_eco_en(&self) -> RndEcoEnR {
        RndEcoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn rnd_eco_result(&self) -> RndEcoResultR {
        RndEcoResultR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn rnd_eco_en(&mut self) -> RndEcoEnW<'_, RndEcoCsSpec> {
        RndEcoEnW::new(self, 0)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RndEcoCsSpec;
impl crate::RegisterSpec for RndEcoCsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_eco_cs::R`](R) reader structure"]
impl crate::Readable for RndEcoCsSpec {}
#[doc = "`write(|w| ..)` method takes [`rnd_eco_cs::W`](W) writer structure"]
impl crate::Writable for RndEcoCsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RND_ECO_CS to value 0"]
impl crate::Resettable for RndEcoCsSpec {}
