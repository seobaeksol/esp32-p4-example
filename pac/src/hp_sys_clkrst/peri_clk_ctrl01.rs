#[doc = "Register `PERI_CLK_CTRL01` reader"]
pub type R = crate::R<PeriClkCtrl01Spec>;
#[doc = "Register `PERI_CLK_CTRL01` writer"]
pub type W = crate::W<PeriClkCtrl01Spec>;
#[doc = "Field `EMAC_RX_CLK_DIV_NUM` reader - Reserved"]
pub type EmacRxClkDivNumR = crate::FieldReader;
#[doc = "Field `EMAC_RX_CLK_DIV_NUM` writer - Reserved"]
pub type EmacRxClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMAC_TX_CLK_SRC_SEL` reader - Reserved"]
pub type EmacTxClkSrcSelR = crate::BitReader;
#[doc = "Field `EMAC_TX_CLK_SRC_SEL` writer - Reserved"]
pub type EmacTxClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_TX_CLK_EN` reader - Reserved"]
pub type EmacTxClkEnR = crate::BitReader;
#[doc = "Field `EMAC_TX_CLK_EN` writer - Reserved"]
pub type EmacTxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_TX_CLK_DIV_NUM` reader - Reserved"]
pub type EmacTxClkDivNumR = crate::FieldReader;
#[doc = "Field `EMAC_TX_CLK_DIV_NUM` writer - Reserved"]
pub type EmacTxClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMAC_PTP_REF_CLK_SRC_SEL` reader - Reserved"]
pub type EmacPtpRefClkSrcSelR = crate::BitReader;
#[doc = "Field `EMAC_PTP_REF_CLK_SRC_SEL` writer - Reserved"]
pub type EmacPtpRefClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_PTP_REF_CLK_EN` reader - Reserved"]
pub type EmacPtpRefClkEnR = crate::BitReader;
#[doc = "Field `EMAC_PTP_REF_CLK_EN` writer - Reserved"]
pub type EmacPtpRefClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_UNUSED0_CLK_EN` reader - Reserved"]
pub type EmacUnused0ClkEnR = crate::BitReader;
#[doc = "Field `EMAC_UNUSED0_CLK_EN` writer - Reserved"]
pub type EmacUnused0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_UNUSED1_CLK_EN` reader - Reserved"]
pub type EmacUnused1ClkEnR = crate::BitReader;
#[doc = "Field `EMAC_UNUSED1_CLK_EN` writer - Reserved"]
pub type EmacUnused1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HS_MODE` reader - Reserved"]
pub type SdioHsModeR = crate::BitReader;
#[doc = "Field `SDIO_HS_MODE` writer - Reserved"]
pub type SdioHsModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_SRC_SEL` reader - Reserved"]
pub type SdioLsClkSrcSelR = crate::BitReader;
#[doc = "Field `SDIO_LS_CLK_SRC_SEL` writer - Reserved"]
pub type SdioLsClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_EN` reader - Reserved"]
pub type SdioLsClkEnR = crate::BitReader;
#[doc = "Field `SDIO_LS_CLK_EN` writer - Reserved"]
pub type SdioLsClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_div_num(&self) -> EmacRxClkDivNumR {
        EmacRxClkDivNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_src_sel(&self) -> EmacTxClkSrcSelR {
        EmacTxClkSrcSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_en(&self) -> EmacTxClkEnR {
        EmacTxClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_div_num(&self) -> EmacTxClkDivNumR {
        EmacTxClkDivNumR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_src_sel(&self) -> EmacPtpRefClkSrcSelR {
        EmacPtpRefClkSrcSelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_en(&self) -> EmacPtpRefClkEnR {
        EmacPtpRefClkEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn emac_unused0_clk_en(&self) -> EmacUnused0ClkEnR {
        EmacUnused0ClkEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn emac_unused1_clk_en(&self) -> EmacUnused1ClkEnR {
        EmacUnused1ClkEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn sdio_hs_mode(&self) -> SdioHsModeR {
        SdioHsModeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_src_sel(&self) -> SdioLsClkSrcSelR {
        SdioLsClkSrcSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_en(&self) -> SdioLsClkEnR {
        SdioLsClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_div_num(&mut self) -> EmacRxClkDivNumW<'_, PeriClkCtrl01Spec> {
        EmacRxClkDivNumW::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_src_sel(&mut self) -> EmacTxClkSrcSelW<'_, PeriClkCtrl01Spec> {
        EmacTxClkSrcSelW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_en(&mut self) -> EmacTxClkEnW<'_, PeriClkCtrl01Spec> {
        EmacTxClkEnW::new(self, 9)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_div_num(&mut self) -> EmacTxClkDivNumW<'_, PeriClkCtrl01Spec> {
        EmacTxClkDivNumW::new(self, 10)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_src_sel(&mut self) -> EmacPtpRefClkSrcSelW<'_, PeriClkCtrl01Spec> {
        EmacPtpRefClkSrcSelW::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_en(&mut self) -> EmacPtpRefClkEnW<'_, PeriClkCtrl01Spec> {
        EmacPtpRefClkEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn emac_unused0_clk_en(&mut self) -> EmacUnused0ClkEnW<'_, PeriClkCtrl01Spec> {
        EmacUnused0ClkEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn emac_unused1_clk_en(&mut self) -> EmacUnused1ClkEnW<'_, PeriClkCtrl01Spec> {
        EmacUnused1ClkEnW::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn sdio_hs_mode(&mut self) -> SdioHsModeW<'_, PeriClkCtrl01Spec> {
        SdioHsModeW::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_src_sel(&mut self) -> SdioLsClkSrcSelW<'_, PeriClkCtrl01Spec> {
        SdioLsClkSrcSelW::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_en(&mut self) -> SdioLsClkEnW<'_, PeriClkCtrl01Spec> {
        SdioLsClkEnW::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl01Spec;
impl crate::RegisterSpec for PeriClkCtrl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl01::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl01Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl01::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL01 to value 0x0401"]
impl crate::Resettable for PeriClkCtrl01Spec {
    const RESET_VALUE: u32 = 0x0401;
}
