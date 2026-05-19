#[doc = "Register `LP_TCM_RAM_RDN_ECO_HIGH` reader"]
pub type R = crate::R<LpTcmRamRdnEcoHighSpec>;
#[doc = "Register `LP_TCM_RAM_RDN_ECO_HIGH` writer"]
pub type W = crate::W<LpTcmRamRdnEcoHighSpec>;
#[doc = "Field `LP_TCM_RAM_RDN_ECO_HIGH` reader - need_des"]
pub type LpTcmRamRdnEcoHighR = crate::FieldReader<u32>;
#[doc = "Field `LP_TCM_RAM_RDN_ECO_HIGH` writer - need_des"]
pub type LpTcmRamRdnEcoHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_high(&self) -> LpTcmRamRdnEcoHighR {
        LpTcmRamRdnEcoHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_high(&mut self) -> LpTcmRamRdnEcoHighW<'_, LpTcmRamRdnEcoHighSpec> {
        LpTcmRamRdnEcoHighW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpTcmRamRdnEcoHighSpec;
impl crate::RegisterSpec for LpTcmRamRdnEcoHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_tcm_ram_rdn_eco_high::R`](R) reader structure"]
impl crate::Readable for LpTcmRamRdnEcoHighSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_tcm_ram_rdn_eco_high::W`](W) writer structure"]
impl crate::Writable for LpTcmRamRdnEcoHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_TCM_RAM_RDN_ECO_HIGH to value 0xffff_ffff"]
impl crate::Resettable for LpTcmRamRdnEcoHighSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
