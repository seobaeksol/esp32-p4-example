#[doc = "Register `PG_GLITCH_CNTL` reader"]
pub type R = crate::R<PgGlitchCntlSpec>;
#[doc = "Register `PG_GLITCH_CNTL` writer"]
pub type W = crate::W<PgGlitchCntlSpec>;
#[doc = "Field `POWER_GLITCH_RESET_ENA` reader - need_des"]
pub type PowerGlitchResetEnaR = crate::BitReader;
#[doc = "Field `POWER_GLITCH_RESET_ENA` writer - need_des"]
pub type PowerGlitchResetEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn power_glitch_reset_ena(&self) -> PowerGlitchResetEnaR {
        PowerGlitchResetEnaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn power_glitch_reset_ena(&mut self) -> PowerGlitchResetEnaW<'_, PgGlitchCntlSpec> {
        PowerGlitchResetEnaW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_glitch_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_glitch_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgGlitchCntlSpec;
impl crate::RegisterSpec for PgGlitchCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pg_glitch_cntl::R`](R) reader structure"]
impl crate::Readable for PgGlitchCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`pg_glitch_cntl::W`](W) writer structure"]
impl crate::Writable for PgGlitchCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PG_GLITCH_CNTL to value 0"]
impl crate::Resettable for PgGlitchCntlSpec {}
