#[doc = "Register `LP_TCM_ROM_RDN_ECO_LOW` reader"]
pub type R = crate::R<LpTcmRomRdnEcoLowSpec>;
#[doc = "Register `LP_TCM_ROM_RDN_ECO_LOW` writer"]
pub type W = crate::W<LpTcmRomRdnEcoLowSpec>;
#[doc = "Field `LP_TCM_ROM_RDN_ECO_LOW` reader - need_des"]
pub type LpTcmRomRdnEcoLowR = crate::FieldReader<u32>;
#[doc = "Field `LP_TCM_ROM_RDN_ECO_LOW` writer - need_des"]
pub type LpTcmRomRdnEcoLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_low(&self) -> LpTcmRomRdnEcoLowR {
        LpTcmRomRdnEcoLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_low(&mut self) -> LpTcmRomRdnEcoLowW<'_, LpTcmRomRdnEcoLowSpec> {
        LpTcmRomRdnEcoLowW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpTcmRomRdnEcoLowSpec;
impl crate::RegisterSpec for LpTcmRomRdnEcoLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_tcm_rom_rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for LpTcmRomRdnEcoLowSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_tcm_rom_rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for LpTcmRomRdnEcoLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_TCM_ROM_RDN_ECO_LOW to value 0"]
impl crate::Resettable for LpTcmRomRdnEcoLowSpec {}
