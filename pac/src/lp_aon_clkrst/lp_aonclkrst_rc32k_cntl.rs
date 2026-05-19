#[doc = "Register `LP_AONCLKRST_RC32K_CNTL` reader"]
pub type R = crate::R<LpAonclkrstRc32kCntlSpec>;
#[doc = "Register `LP_AONCLKRST_RC32K_CNTL` writer"]
pub type W = crate::W<LpAonclkrstRc32kCntlSpec>;
#[doc = "Field `LP_AONCLKRST_RC32K_DFREQ` reader - need_des"]
pub type LpAonclkrstRc32kDfreqR = crate::FieldReader<u32>;
#[doc = "Field `LP_AONCLKRST_RC32K_DFREQ` writer - need_des"]
pub type LpAonclkrstRc32kDfreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rc32k_dfreq(&self) -> LpAonclkrstRc32kDfreqR {
        LpAonclkrstRc32kDfreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rc32k_dfreq(
        &mut self,
    ) -> LpAonclkrstRc32kDfreqW<'_, LpAonclkrstRc32kCntlSpec> {
        LpAonclkrstRc32kDfreqW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_rc32k_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_rc32k_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstRc32kCntlSpec;
impl crate::RegisterSpec for LpAonclkrstRc32kCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_rc32k_cntl::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstRc32kCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_rc32k_cntl::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstRc32kCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_RC32K_CNTL to value 0x028a"]
impl crate::Resettable for LpAonclkrstRc32kCntlSpec {
    const RESET_VALUE: u32 = 0x028a;
}
