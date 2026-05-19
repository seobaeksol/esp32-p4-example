#[doc = "Register `ICM_RDN_ECO_CS` reader"]
pub type R = crate::R<IcmRdnEcoCsSpec>;
#[doc = "Register `ICM_RDN_ECO_CS` writer"]
pub type W = crate::W<IcmRdnEcoCsSpec>;
#[doc = "Field `RDN_ECO_EN` reader - "]
pub type RdnEcoEnR = crate::BitReader;
#[doc = "Field `RDN_ECO_EN` writer - "]
pub type RdnEcoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDN_ECO_RESULT` reader - "]
pub type RdnEcoResultR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rdn_eco_en(&self) -> RdnEcoEnR {
        RdnEcoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rdn_eco_result(&self) -> RdnEcoResultR {
        RdnEcoResultR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rdn_eco_en(&mut self) -> RdnEcoEnW<'_, IcmRdnEcoCsSpec> {
        RdnEcoEnW::new(self, 0)
    }
}
#[doc = "RDN ECO control/status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_rdn_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_rdn_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmRdnEcoCsSpec;
impl crate::RegisterSpec for IcmRdnEcoCsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_rdn_eco_cs::R`](R) reader structure"]
impl crate::Readable for IcmRdnEcoCsSpec {}
#[doc = "`write(|w| ..)` method takes [`icm_rdn_eco_cs::W`](W) writer structure"]
impl crate::Writable for IcmRdnEcoCsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_RDN_ECO_CS to value 0"]
impl crate::Resettable for IcmRdnEcoCsSpec {}
