#[doc = "Register `PERI_CLK_CTRL00` reader"]
pub type R = crate::R<PeriClkCtrl00Spec>;
#[doc = "Register `PERI_CLK_CTRL00` writer"]
pub type W = crate::W<PeriClkCtrl00Spec>;
#[doc = "Field `FLASH_CLK_SRC_SEL` reader - Reserved"]
pub type FlashClkSrcSelR = crate::FieldReader;
#[doc = "Field `FLASH_CLK_SRC_SEL` writer - Reserved"]
pub type FlashClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLASH_PLL_CLK_EN` reader - Reserved"]
pub type FlashPllClkEnR = crate::BitReader;
#[doc = "Field `FLASH_PLL_CLK_EN` writer - Reserved"]
pub type FlashPllClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CORE_CLK_EN` reader - Reserved"]
pub type FlashCoreClkEnR = crate::BitReader;
#[doc = "Field `FLASH_CORE_CLK_EN` writer - Reserved"]
pub type FlashCoreClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CORE_CLK_DIV_NUM` reader - Reserved"]
pub type FlashCoreClkDivNumR = crate::FieldReader;
#[doc = "Field `FLASH_CORE_CLK_DIV_NUM` writer - Reserved"]
pub type FlashCoreClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PSRAM_CLK_SRC_SEL` reader - Reserved"]
pub type PsramClkSrcSelR = crate::FieldReader;
#[doc = "Field `PSRAM_CLK_SRC_SEL` writer - Reserved"]
pub type PsramClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSRAM_PLL_CLK_EN` reader - Reserved"]
pub type PsramPllClkEnR = crate::BitReader;
#[doc = "Field `PSRAM_PLL_CLK_EN` writer - Reserved"]
pub type PsramPllClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_CORE_CLK_EN` reader - Reserved"]
pub type PsramCoreClkEnR = crate::BitReader;
#[doc = "Field `PSRAM_CORE_CLK_EN` writer - Reserved"]
pub type PsramCoreClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_CORE_CLK_DIV_NUM` reader - Reserved"]
pub type PsramCoreClkDivNumR = crate::FieldReader;
#[doc = "Field `PSRAM_CORE_CLK_DIV_NUM` writer - Reserved"]
pub type PsramCoreClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_EMAC_REF_CLK_EN` reader - Reserved"]
pub type PadEmacRefClkEnR = crate::BitReader;
#[doc = "Field `PAD_EMAC_REF_CLK_EN` writer - Reserved"]
pub type PadEmacRefClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RMII_CLK_SRC_SEL` reader - Reserved"]
pub type EmacRmiiClkSrcSelR = crate::FieldReader;
#[doc = "Field `EMAC_RMII_CLK_SRC_SEL` writer - Reserved"]
pub type EmacRmiiClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMAC_RMII_CLK_EN` reader - Reserved"]
pub type EmacRmiiClkEnR = crate::BitReader;
#[doc = "Field `EMAC_RMII_CLK_EN` writer - Reserved"]
pub type EmacRmiiClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RX_CLK_SRC_SEL` reader - Reserved"]
pub type EmacRxClkSrcSelR = crate::BitReader;
#[doc = "Field `EMAC_RX_CLK_SRC_SEL` writer - Reserved"]
pub type EmacRxClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RX_CLK_EN` reader - Reserved"]
pub type EmacRxClkEnR = crate::BitReader;
#[doc = "Field `EMAC_RX_CLK_EN` writer - Reserved"]
pub type EmacRxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn flash_clk_src_sel(&self) -> FlashClkSrcSelR {
        FlashClkSrcSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn flash_pll_clk_en(&self) -> FlashPllClkEnR {
        FlashPllClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_en(&self) -> FlashCoreClkEnR {
        FlashCoreClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_div_num(&self) -> FlashCoreClkDivNumR {
        FlashCoreClkDivNumR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn psram_clk_src_sel(&self) -> PsramClkSrcSelR {
        PsramClkSrcSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn psram_pll_clk_en(&self) -> PsramPllClkEnR {
        PsramPllClkEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_en(&self) -> PsramCoreClkEnR {
        PsramCoreClkEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_div_num(&self) -> PsramCoreClkDivNumR {
        PsramCoreClkDivNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pad_emac_ref_clk_en(&self) -> PadEmacRefClkEnR {
        PadEmacRefClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_src_sel(&self) -> EmacRmiiClkSrcSelR {
        EmacRmiiClkSrcSelR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_en(&self) -> EmacRmiiClkEnR {
        EmacRmiiClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_src_sel(&self) -> EmacRxClkSrcSelR {
        EmacRxClkSrcSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_en(&self) -> EmacRxClkEnR {
        EmacRxClkEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn flash_clk_src_sel(&mut self) -> FlashClkSrcSelW<'_, PeriClkCtrl00Spec> {
        FlashClkSrcSelW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn flash_pll_clk_en(&mut self) -> FlashPllClkEnW<'_, PeriClkCtrl00Spec> {
        FlashPllClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_en(&mut self) -> FlashCoreClkEnW<'_, PeriClkCtrl00Spec> {
        FlashCoreClkEnW::new(self, 3)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_div_num(&mut self) -> FlashCoreClkDivNumW<'_, PeriClkCtrl00Spec> {
        FlashCoreClkDivNumW::new(self, 4)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn psram_clk_src_sel(&mut self) -> PsramClkSrcSelW<'_, PeriClkCtrl00Spec> {
        PsramClkSrcSelW::new(self, 12)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn psram_pll_clk_en(&mut self) -> PsramPllClkEnW<'_, PeriClkCtrl00Spec> {
        PsramPllClkEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_en(&mut self) -> PsramCoreClkEnW<'_, PeriClkCtrl00Spec> {
        PsramCoreClkEnW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_div_num(&mut self) -> PsramCoreClkDivNumW<'_, PeriClkCtrl00Spec> {
        PsramCoreClkDivNumW::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pad_emac_ref_clk_en(&mut self) -> PadEmacRefClkEnW<'_, PeriClkCtrl00Spec> {
        PadEmacRefClkEnW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_src_sel(&mut self) -> EmacRmiiClkSrcSelW<'_, PeriClkCtrl00Spec> {
        EmacRmiiClkSrcSelW::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_en(&mut self) -> EmacRmiiClkEnW<'_, PeriClkCtrl00Spec> {
        EmacRmiiClkEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_src_sel(&mut self) -> EmacRxClkSrcSelW<'_, PeriClkCtrl00Spec> {
        EmacRxClkSrcSelW::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_en(&mut self) -> EmacRxClkEnW<'_, PeriClkCtrl00Spec> {
        EmacRxClkEnW::new(self, 29)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl00Spec;
impl crate::RegisterSpec for PeriClkCtrl00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl00::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl00Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl00::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL00 to value 0xc03c"]
impl crate::Resettable for PeriClkCtrl00Spec {
    const RESET_VALUE: u32 = 0xc03c;
}
