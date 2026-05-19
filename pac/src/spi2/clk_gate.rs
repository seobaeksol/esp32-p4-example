#[doc = "Register `CLK_GATE` reader"]
pub type R = crate::R<ClkGateSpec>;
#[doc = "Register `CLK_GATE` writer"]
pub type W = crate::W<ClkGateSpec>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clk gate"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clk gate"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_CLK_ACTIVE` reader - Set this bit to power on the SPI module clock."]
pub type MstClkActiveR = crate::BitReader;
#[doc = "Field `MST_CLK_ACTIVE` writer - Set this bit to power on the SPI module clock."]
pub type MstClkActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_CLK_SEL` reader - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
pub type MstClkSelR = crate::BitReader;
#[doc = "Field `MST_CLK_SEL` writer - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
pub type MstClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to power on the SPI module clock."]
    #[inline(always)]
    pub fn mst_clk_active(&self) -> MstClkActiveR {
        MstClkActiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
    #[inline(always)]
    pub fn mst_clk_sel(&self) -> MstClkSelR {
        MstClkSelR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, ClkGateSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to power on the SPI module clock."]
    #[inline(always)]
    pub fn mst_clk_active(&mut self) -> MstClkActiveW<'_, ClkGateSpec> {
        MstClkActiveW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
    #[inline(always)]
    pub fn mst_clk_sel(&mut self) -> MstClkSelW<'_, ClkGateSpec> {
        MstClkSelW::new(self, 2)
    }
}
#[doc = "SPI module clock and register clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkGateSpec;
impl crate::RegisterSpec for ClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gate::R`](R) reader structure"]
impl crate::Readable for ClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_gate::W`](W) writer structure"]
impl crate::Writable for ClkGateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_GATE to value 0"]
impl crate::Resettable for ClkGateSpec {}
