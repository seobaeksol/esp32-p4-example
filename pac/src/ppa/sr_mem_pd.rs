#[doc = "Register `SR_MEM_PD` reader"]
pub type R = crate::R<SrMemPdSpec>;
#[doc = "Register `SR_MEM_PD` writer"]
pub type W = crate::W<SrMemPdSpec>;
#[doc = "Field `SR_MEM_CLK_ENA` reader - Set this bit to force clock enable of scaling and rotating engine's data memory."]
pub type SrMemClkEnaR = crate::BitReader;
#[doc = "Field `SR_MEM_CLK_ENA` writer - Set this bit to force clock enable of scaling and rotating engine's data memory."]
pub type SrMemClkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_MEM_FORCE_PD` reader - Set this bit to force power down scaling and rotating engine's data memory."]
pub type SrMemForcePdR = crate::BitReader;
#[doc = "Field `SR_MEM_FORCE_PD` writer - Set this bit to force power down scaling and rotating engine's data memory."]
pub type SrMemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_MEM_FORCE_PU` reader - Set this bit to force power up scaling and rotating engine's data memory."]
pub type SrMemForcePuR = crate::BitReader;
#[doc = "Field `SR_MEM_FORCE_PU` writer - Set this bit to force power up scaling and rotating engine's data memory."]
pub type SrMemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force clock enable of scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_clk_ena(&self) -> SrMemClkEnaR {
        SrMemClkEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force power down scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_force_pd(&self) -> SrMemForcePdR {
        SrMemForcePdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_force_pu(&self) -> SrMemForcePuR {
        SrMemForcePuR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force clock enable of scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_clk_ena(&mut self) -> SrMemClkEnaW<'_, SrMemPdSpec> {
        SrMemClkEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force power down scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_force_pd(&mut self) -> SrMemForcePdW<'_, SrMemPdSpec> {
        SrMemForcePdW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power up scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_force_pu(&mut self) -> SrMemForcePuW<'_, SrMemPdSpec> {
        SrMemForcePuW::new(self, 2)
    }
}
#[doc = "SR memory power done register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_mem_pd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_mem_pd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrMemPdSpec;
impl crate::RegisterSpec for SrMemPdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_mem_pd::R`](R) reader structure"]
impl crate::Readable for SrMemPdSpec {}
#[doc = "`write(|w| ..)` method takes [`sr_mem_pd::W`](W) writer structure"]
impl crate::Writable for SrMemPdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR_MEM_PD to value 0"]
impl crate::Resettable for SrMemPdSpec {}
