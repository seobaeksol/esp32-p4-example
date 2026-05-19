#[doc = "Register `ICM_RDN_ECO_HIGH` reader"]
pub type R = crate::R<IcmRdnEcoHighSpec>;
#[doc = "Register `ICM_RDN_ECO_HIGH` writer"]
pub type W = crate::W<IcmRdnEcoHighSpec>;
#[doc = "Field `RDN_ECO_HIGH` reader - "]
pub type RdnEcoHighR = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_HIGH` writer - "]
pub type RdnEcoHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> RdnEcoHighR {
        RdnEcoHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdn_eco_high(&mut self) -> RdnEcoHighW<'_, IcmRdnEcoHighSpec> {
        RdnEcoHighW::new(self, 0)
    }
}
#[doc = "RDN ECO high\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_rdn_eco_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_rdn_eco_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmRdnEcoHighSpec;
impl crate::RegisterSpec for IcmRdnEcoHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_rdn_eco_high::R`](R) reader structure"]
impl crate::Readable for IcmRdnEcoHighSpec {}
#[doc = "`write(|w| ..)` method takes [`icm_rdn_eco_high::W`](W) writer structure"]
impl crate::Writable for IcmRdnEcoHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_RDN_ECO_HIGH to value 0"]
impl crate::Resettable for IcmRdnEcoHighSpec {}
