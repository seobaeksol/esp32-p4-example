#[doc = "Register `BYPASS_CHAIN` reader"]
pub type R = crate::R<BypassChainSpec>;
#[doc = "Register `BYPASS_CHAIN` writer"]
pub type W = crate::W<BypassChainSpec>;
#[doc = "Field `CLK_CHAIN_EN` reader - needs field desc"]
pub type ClkChainEnR = crate::FieldReader<u32>;
#[doc = "Field `CLK_CHAIN_EN` writer - needs field desc"]
pub type ClkChainEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - needs field desc"]
    #[inline(always)]
    pub fn clk_chain_en(&self) -> ClkChainEnR {
        ClkChainEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - needs field desc"]
    #[inline(always)]
    pub fn clk_chain_en(&mut self) -> ClkChainEnW<'_, BypassChainSpec> {
        ClkChainEnW::new(self, 0)
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`bypass_chain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bypass_chain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BypassChainSpec;
impl crate::RegisterSpec for BypassChainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bypass_chain::R`](R) reader structure"]
impl crate::Readable for BypassChainSpec {}
#[doc = "`write(|w| ..)` method takes [`bypass_chain::W`](W) writer structure"]
impl crate::Writable for BypassChainSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BYPASS_CHAIN to value 0xffff_ffff"]
impl crate::Resettable for BypassChainSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
