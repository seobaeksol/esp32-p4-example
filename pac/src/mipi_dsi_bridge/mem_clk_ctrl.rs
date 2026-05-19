#[doc = "Register `MEM_CLK_CTRL` reader"]
pub type R = crate::R<MemClkCtrlSpec>;
#[doc = "Register `MEM_CLK_CTRL` writer"]
pub type W = crate::W<MemClkCtrlSpec>;
#[doc = "Field `DSI_BRIDGE_MEM_CLK_FORCE_ON` reader - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on"]
pub type DsiBridgeMemClkForceOnR = crate::BitReader;
#[doc = "Field `DSI_BRIDGE_MEM_CLK_FORCE_ON` writer - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on"]
pub type DsiBridgeMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_MEM_CLK_FORCE_ON` reader - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on"]
pub type DsiMemClkForceOnR = crate::BitReader;
#[doc = "Field `DSI_MEM_CLK_FORCE_ON` writer - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on"]
pub type DsiMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on"]
    #[inline(always)]
    pub fn dsi_bridge_mem_clk_force_on(&self) -> DsiBridgeMemClkForceOnR {
        DsiBridgeMemClkForceOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on"]
    #[inline(always)]
    pub fn dsi_mem_clk_force_on(&self) -> DsiMemClkForceOnR {
        DsiMemClkForceOnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on"]
    #[inline(always)]
    pub fn dsi_bridge_mem_clk_force_on(&mut self) -> DsiBridgeMemClkForceOnW<'_, MemClkCtrlSpec> {
        DsiBridgeMemClkForceOnW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on"]
    #[inline(always)]
    pub fn dsi_mem_clk_force_on(&mut self) -> DsiMemClkForceOnW<'_, MemClkCtrlSpec> {
        DsiMemClkForceOnW::new(self, 1)
    }
}
#[doc = "dsi_bridge mem force on control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClkCtrlSpec;
impl crate::RegisterSpec for MemClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for MemClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for MemClkCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CLK_CTRL to value 0"]
impl crate::Resettable for MemClkCtrlSpec {}
