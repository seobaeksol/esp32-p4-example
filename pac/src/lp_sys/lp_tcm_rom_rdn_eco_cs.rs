#[doc = "Register `LP_TCM_ROM_RDN_ECO_CS` reader"]
pub type R = crate::R<LpTcmRomRdnEcoCsSpec>;
#[doc = "Register `LP_TCM_ROM_RDN_ECO_CS` writer"]
pub type W = crate::W<LpTcmRomRdnEcoCsSpec>;
#[doc = "Field `LP_TCM_ROM_RDN_ECO_EN` reader - need_des"]
pub type LpTcmRomRdnEcoEnR = crate::BitReader;
#[doc = "Field `LP_TCM_ROM_RDN_ECO_EN` writer - need_des"]
pub type LpTcmRomRdnEcoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TCM_ROM_RDN_ECO_RESULT` reader - need_des"]
pub type LpTcmRomRdnEcoResultR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_en(&self) -> LpTcmRomRdnEcoEnR {
        LpTcmRomRdnEcoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_result(&self) -> LpTcmRomRdnEcoResultR {
        LpTcmRomRdnEcoResultR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_en(&mut self) -> LpTcmRomRdnEcoEnW<'_, LpTcmRomRdnEcoCsSpec> {
        LpTcmRomRdnEcoEnW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpTcmRomRdnEcoCsSpec;
impl crate::RegisterSpec for LpTcmRomRdnEcoCsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_tcm_rom_rdn_eco_cs::R`](R) reader structure"]
impl crate::Readable for LpTcmRomRdnEcoCsSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_tcm_rom_rdn_eco_cs::W`](W) writer structure"]
impl crate::Writable for LpTcmRomRdnEcoCsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_TCM_ROM_RDN_ECO_CS to value 0"]
impl crate::Resettable for LpTcmRomRdnEcoCsSpec {}
