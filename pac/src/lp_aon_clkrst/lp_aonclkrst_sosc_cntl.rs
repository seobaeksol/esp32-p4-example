#[doc = "Register `LP_AONCLKRST_SOSC_CNTL` reader"]
pub type R = crate::R<LpAonclkrstSoscCntlSpec>;
#[doc = "Register `LP_AONCLKRST_SOSC_CNTL` writer"]
pub type W = crate::W<LpAonclkrstSoscCntlSpec>;
#[doc = "Field `LP_AONCLKRST_SOSC_DFREQ` reader - need_des"]
pub type LpAonclkrstSoscDfreqR = crate::FieldReader<u16>;
#[doc = "Field `LP_AONCLKRST_SOSC_DFREQ` writer - need_des"]
pub type LpAonclkrstSoscDfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_sosc_dfreq(&self) -> LpAonclkrstSoscDfreqR {
        LpAonclkrstSoscDfreqR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_sosc_dfreq(
        &mut self,
    ) -> LpAonclkrstSoscDfreqW<'_, LpAonclkrstSoscCntlSpec> {
        LpAonclkrstSoscDfreqW::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_sosc_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_sosc_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstSoscCntlSpec;
impl crate::RegisterSpec for LpAonclkrstSoscCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_sosc_cntl::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstSoscCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_sosc_cntl::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstSoscCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_SOSC_CNTL to value 0x2b00_0000"]
impl crate::Resettable for LpAonclkrstSoscCntlSpec {
    const RESET_VALUE: u32 = 0x2b00_0000;
}
