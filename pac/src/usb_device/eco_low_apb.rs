#[doc = "Register `ECO_LOW_APB` reader"]
pub type R = crate::R<EcoLowApbSpec>;
#[doc = "Register `ECO_LOW_APB` writer"]
pub type W = crate::W<EcoLowApbSpec>;
#[doc = "Field `RND_ECO_LOW_APB` reader - Reserved."]
pub type RndEcoLowApbR = crate::FieldReader<u32>;
#[doc = "Field `RND_ECO_LOW_APB` writer - Reserved."]
pub type RndEcoLowApbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_low_apb(&self) -> RndEcoLowApbR {
        RndEcoLowApbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_low_apb(&mut self) -> RndEcoLowApbW<'_, EcoLowApbSpec> {
        RndEcoLowApbW::new(self, 0)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_low_apb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_low_apb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoLowApbSpec;
impl crate::RegisterSpec for EcoLowApbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_low_apb::R`](R) reader structure"]
impl crate::Readable for EcoLowApbSpec {}
#[doc = "`write(|w| ..)` method takes [`eco_low_apb::W`](W) writer structure"]
impl crate::Writable for EcoLowApbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_LOW_APB to value 0"]
impl crate::Resettable for EcoLowApbSpec {}
