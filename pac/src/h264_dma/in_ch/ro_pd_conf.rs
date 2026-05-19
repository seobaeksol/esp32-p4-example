#[doc = "Register `RO_PD_CONF` reader"]
pub type R = crate::R<RoPdConfSpec>;
#[doc = "Register `RO_PD_CONF` writer"]
pub type W = crate::W<RoPdConfSpec>;
#[doc = "Field `IN_RO_RAM_CLK_FO` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type InRoRamClkFoR = crate::BitReader;
#[doc = "Field `IN_RO_RAM_CLK_FO` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type InRoRamClkFoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn in_ro_ram_clk_fo(&self) -> InRoRamClkFoR {
        InRoRamClkFoR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn in_ro_ram_clk_fo(&mut self) -> InRoRamClkFoW<'_, RoPdConfSpec> {
        InRoRamClkFoW::new(self, 6)
    }
}
#[doc = "RX CHx reorder power config register. Available on CH0\n\nYou can [`read`](crate::Reg::read) this register and get [`ro_pd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ro_pd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets RO_PD_CONF to value 0"]
impl crate::Resettable for RoPdConfSpec {}
