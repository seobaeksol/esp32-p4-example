#[doc = "Register `ECO_HIGH_APB` reader"]
pub type R = crate::R<EcoHighApbSpec>;
#[doc = "Register `ECO_HIGH_APB` writer"]
pub type W = crate::W<EcoHighApbSpec>;
#[doc = "Field `RND_ECO_HIGH_APB` reader - Reserved."]
pub type RndEcoHighApbR = crate::FieldReader<u32>;
#[doc = "Field `RND_ECO_HIGH_APB` writer - Reserved."]
pub type RndEcoHighApbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_high_apb(&self) -> RndEcoHighApbR {
        RndEcoHighApbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_high_apb(&mut self) -> RndEcoHighApbW<'_, EcoHighApbSpec> {
        RndEcoHighApbW::new(self, 0)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_high_apb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_high_apb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoHighApbSpec;
impl crate::RegisterSpec for EcoHighApbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_high_apb::R`](R) reader structure"]
impl crate::Readable for EcoHighApbSpec {}
#[doc = "`write(|w| ..)` method takes [`eco_high_apb::W`](W) writer structure"]
impl crate::Writable for EcoHighApbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_HIGH_APB to value 0xffff_ffff"]
impl crate::Resettable for EcoHighApbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
