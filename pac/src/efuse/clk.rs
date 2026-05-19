#[doc = "Register `CLK` reader"]
pub type R = crate::R<ClkSpec>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<ClkSpec>;
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to force eFuse SRAM into power-saving mode."]
pub type MemForcePdR = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to force eFuse SRAM into power-saving mode."]
pub type MemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MemClkForceOnR = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PU` reader - Set this bit to force eFuse SRAM into working mode."]
pub type MemForcePuR = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - Set this bit to force eFuse SRAM into working mode."]
pub type MemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Set this bit to force enable eFuse register configuration clock signal."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Set this bit to force enable eFuse register configuration clock signal."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MemForcePdR {
        MemForcePdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit and force to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MemClkForceOnR {
        MemClkForceOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MemForcePuR {
        MemForcePuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to force enable eFuse register configuration clock signal."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MemForcePdW<'_, ClkSpec> {
        MemForcePdW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit and force to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MemClkForceOnW<'_, ClkSpec> {
        MemClkForceOnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MemForcePuW<'_, ClkSpec> {
        MemForcePuW::new(self, 2)
    }
    #[doc = "Bit 16 - Set this bit to force enable eFuse register configuration clock signal."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, ClkSpec> {
        EnW::new(self, 16)
    }
}
#[doc = "eFuse clcok configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSpec;
impl crate::RegisterSpec for ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for ClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for ClkSpec {}
