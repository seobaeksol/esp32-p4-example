#[doc = "Register `TCM_RDN_ECO_HIGH` reader"]
pub type R = crate::R<TcmRdnEcoHighSpec>;
#[doc = "Register `TCM_RDN_ECO_HIGH` writer"]
pub type W = crate::W<TcmRdnEcoHighSpec>;
#[doc = "Field `REG_HP_TCM_RDN_ECO_HIGH` reader - NA"]
pub type RegHpTcmRdnEcoHighR = crate::FieldReader<u32>;
#[doc = "Field `REG_HP_TCM_RDN_ECO_HIGH` writer - NA"]
pub type RegHpTcmRdnEcoHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_hp_tcm_rdn_eco_high(&self) -> RegHpTcmRdnEcoHighR {
        RegHpTcmRdnEcoHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_hp_tcm_rdn_eco_high(&mut self) -> RegHpTcmRdnEcoHighW<'_, TcmRdnEcoHighSpec> {
        RegHpTcmRdnEcoHighW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_rdn_eco_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_rdn_eco_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmRdnEcoHighSpec;
impl crate::RegisterSpec for TcmRdnEcoHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_rdn_eco_high::R`](R) reader structure"]
impl crate::Readable for TcmRdnEcoHighSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_rdn_eco_high::W`](W) writer structure"]
impl crate::Writable for TcmRdnEcoHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_RDN_ECO_HIGH to value 0xffff_ffff"]
impl crate::Resettable for TcmRdnEcoHighSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
