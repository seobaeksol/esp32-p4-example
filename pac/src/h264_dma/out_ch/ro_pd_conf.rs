#[doc = "Register `RO_PD_CONF` reader"]
pub type R = crate::R<RoPdConfSpec>;
#[doc = "Register `RO_PD_CONF` writer"]
pub type W = crate::W<RoPdConfSpec>;
#[doc = "Field `OUT_RO_RAM_FORCE_PD` reader - dma reorder ram power down"]
pub type OutRoRamForcePdR = crate::BitReader;
#[doc = "Field `OUT_RO_RAM_FORCE_PD` writer - dma reorder ram power down"]
pub type OutRoRamForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RO_RAM_FORCE_PU` reader - dma reorder ram power up"]
pub type OutRoRamForcePuR = crate::BitReader;
#[doc = "Field `OUT_RO_RAM_FORCE_PU` writer - dma reorder ram power up"]
pub type OutRoRamForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RO_RAM_CLK_FO` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type OutRoRamClkFoR = crate::BitReader;
#[doc = "Field `OUT_RO_RAM_CLK_FO` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type OutRoRamClkFoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - dma reorder ram power down"]
    #[inline(always)]
    pub fn out_ro_ram_force_pd(&self) -> OutRoRamForcePdR {
        OutRoRamForcePdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dma reorder ram power up"]
    #[inline(always)]
    pub fn out_ro_ram_force_pu(&self) -> OutRoRamForcePuR {
        OutRoRamForcePuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn out_ro_ram_clk_fo(&self) -> OutRoRamClkFoR {
        OutRoRamClkFoR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - dma reorder ram power down"]
    #[inline(always)]
    pub fn out_ro_ram_force_pd(&mut self) -> OutRoRamForcePdW<'_, RoPdConfSpec> {
        OutRoRamForcePdW::new(self, 4)
    }
    #[doc = "Bit 5 - dma reorder ram power up"]
    #[inline(always)]
    pub fn out_ro_ram_force_pu(&mut self) -> OutRoRamForcePuW<'_, RoPdConfSpec> {
        OutRoRamForcePuW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn out_ro_ram_clk_fo(&mut self) -> OutRoRamClkFoW<'_, RoPdConfSpec> {
        OutRoRamClkFoW::new(self, 6)
    }
}
#[doc = "TX CHx reorder power config register. Available on CH0\n\nYou can [`read`](crate::Reg::read) this register and get [`ro_pd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ro_pd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoPdConfSpec;
impl crate::RegisterSpec for RoPdConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ro_pd_conf::R`](R) reader structure"]
impl crate::Readable for RoPdConfSpec {}
#[doc = "`write(|w| ..)` method takes [`ro_pd_conf::W`](W) writer structure"]
impl crate::Writable for RoPdConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RO_PD_CONF to value 0x20"]
impl crate::Resettable for RoPdConfSpec {
    const RESET_VALUE: u32 = 0x20;
}
