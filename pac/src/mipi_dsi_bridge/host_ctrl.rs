#[doc = "Register `HOST_CTRL` reader"]
pub type R = crate::R<HostCtrlSpec>;
#[doc = "Register `HOST_CTRL` writer"]
pub type W = crate::W<HostCtrlSpec>;
#[doc = "Field `DSI_CFG_REF_CLK_EN` reader - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
pub type DsiCfgRefClkEnR = crate::BitReader;
#[doc = "Field `DSI_CFG_REF_CLK_EN` writer - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
pub type DsiCfgRefClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_cfg_ref_clk_en(&self) -> DsiCfgRefClkEnR {
        DsiCfgRefClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_cfg_ref_clk_en(&mut self) -> DsiCfgRefClkEnW<'_, HostCtrlSpec> {
        DsiCfgRefClkEnW::new(self, 0)
    }
}
#[doc = "dsi_bridge host control register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtrlSpec;
impl crate::RegisterSpec for HostCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctrl::R`](R) reader structure"]
impl crate::Readable for HostCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl::W`](W) writer structure"]
impl crate::Writable for HostCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_CTRL to value 0x01"]
impl crate::Resettable for HostCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
