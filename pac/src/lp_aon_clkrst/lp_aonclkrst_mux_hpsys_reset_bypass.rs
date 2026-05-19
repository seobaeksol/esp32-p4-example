#[doc = "Register `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` reader"]
pub type R = crate::R<LpAonclkrstMuxHpsysResetBypassSpec>;
#[doc = "Register `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` writer"]
pub type W = crate::W<LpAonclkrstMuxHpsysResetBypassSpec>;
#[doc = "Field `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` reader - reserved"]
pub type LpAonclkrstMuxHpsysResetBypassR = crate::FieldReader<u32>;
#[doc = "Field `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` writer - reserved"]
pub type LpAonclkrstMuxHpsysResetBypassW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_mux_hpsys_reset_bypass(&self) -> LpAonclkrstMuxHpsysResetBypassR {
        LpAonclkrstMuxHpsysResetBypassR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_mux_hpsys_reset_bypass(
        &mut self,
    ) -> LpAonclkrstMuxHpsysResetBypassW<'_, LpAonclkrstMuxHpsysResetBypassSpec> {
        LpAonclkrstMuxHpsysResetBypassW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_mux_hpsys_reset_bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_mux_hpsys_reset_bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstMuxHpsysResetBypassSpec;
impl crate::RegisterSpec for LpAonclkrstMuxHpsysResetBypassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_mux_hpsys_reset_bypass::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstMuxHpsysResetBypassSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_mux_hpsys_reset_bypass::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstMuxHpsysResetBypassSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS to value 0xffff_ffff"]
impl crate::Resettable for LpAonclkrstMuxHpsysResetBypassSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
