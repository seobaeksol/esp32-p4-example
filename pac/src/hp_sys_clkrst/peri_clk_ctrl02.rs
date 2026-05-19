#[doc = "Register `PERI_CLK_CTRL02` reader"]
pub type R = crate::R<PeriClkCtrl02Spec>;
#[doc = "Register `PERI_CLK_CTRL02` writer"]
pub type W = crate::W<PeriClkCtrl02Spec>;
#[doc = "Field `SDIO_LS_CLK_DIV_NUM` reader - Reserved"]
pub type SdioLsClkDivNumR = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_DIV_NUM` writer - Reserved"]
pub type SdioLsClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDIO_LS_CLK_EDGE_CFG_UPDATE` writer - Reserved"]
pub type SdioLsClkEdgeCfgUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_EDGE_L` reader - Reserved"]
pub type SdioLsClkEdgeLR = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_L` writer - Reserved"]
pub type SdioLsClkEdgeLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_CLK_EDGE_H` reader - Reserved"]
pub type SdioLsClkEdgeHR = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_H` writer - Reserved"]
pub type SdioLsClkEdgeHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_CLK_EDGE_N` reader - Reserved"]
pub type SdioLsClkEdgeNR = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_N` writer - Reserved"]
pub type SdioLsClkEdgeNW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_SLF_CLK_EDGE_SEL` reader - Reserved"]
pub type SdioLsSlfClkEdgeSelR = crate::FieldReader;
#[doc = "Field `SDIO_LS_SLF_CLK_EDGE_SEL` writer - Reserved"]
pub type SdioLsSlfClkEdgeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO_LS_DRV_CLK_EDGE_SEL` reader - Reserved"]
pub type SdioLsDrvClkEdgeSelR = crate::FieldReader;
#[doc = "Field `SDIO_LS_DRV_CLK_EDGE_SEL` writer - Reserved"]
pub type SdioLsDrvClkEdgeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO_LS_SAM_CLK_EDGE_SEL` reader - Reserved"]
pub type SdioLsSamClkEdgeSelR = crate::FieldReader;
#[doc = "Field `SDIO_LS_SAM_CLK_EDGE_SEL` writer - Reserved"]
pub type SdioLsSamClkEdgeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO_LS_SLF_CLK_EN` reader - Reserved"]
pub type SdioLsSlfClkEnR = crate::BitReader;
#[doc = "Field `SDIO_LS_SLF_CLK_EN` writer - Reserved"]
pub type SdioLsSlfClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_DRV_CLK_EN` reader - Reserved"]
pub type SdioLsDrvClkEnR = crate::BitReader;
#[doc = "Field `SDIO_LS_DRV_CLK_EN` writer - Reserved"]
pub type SdioLsDrvClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_SAM_CLK_EN` reader - Reserved"]
pub type SdioLsSamClkEnR = crate::BitReader;
#[doc = "Field `SDIO_LS_SAM_CLK_EN` writer - Reserved"]
pub type SdioLsSamClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIPI_DSI_DPHY_CLK_SRC_SEL` reader - Reserved"]
pub type MipiDsiDphyClkSrcSelR = crate::FieldReader;
#[doc = "Field `MIPI_DSI_DPHY_CLK_SRC_SEL` writer - Reserved"]
pub type MipiDsiDphyClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_div_num(&self) -> SdioLsClkDivNumR {
        SdioLsClkDivNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:12 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_l(&self) -> SdioLsClkEdgeLR {
        SdioLsClkEdgeLR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_h(&self) -> SdioLsClkEdgeHR {
        SdioLsClkEdgeHR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_n(&self) -> SdioLsClkEdgeNR {
        SdioLsClkEdgeNR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_edge_sel(&self) -> SdioLsSlfClkEdgeSelR {
        SdioLsSlfClkEdgeSelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_edge_sel(&self) -> SdioLsDrvClkEdgeSelR {
        SdioLsDrvClkEdgeSelR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_edge_sel(&self) -> SdioLsSamClkEdgeSelR {
        SdioLsSamClkEdgeSelR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_en(&self) -> SdioLsSlfClkEnR {
        SdioLsSlfClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_en(&self) -> SdioLsDrvClkEnR {
        SdioLsDrvClkEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_en(&self) -> SdioLsSamClkEnR {
        SdioLsSamClkEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_clk_src_sel(&self) -> MipiDsiDphyClkSrcSelR {
        MipiDsiDphyClkSrcSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_div_num(&mut self) -> SdioLsClkDivNumW<'_, PeriClkCtrl02Spec> {
        SdioLsClkDivNumW::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_cfg_update(
        &mut self,
    ) -> SdioLsClkEdgeCfgUpdateW<'_, PeriClkCtrl02Spec> {
        SdioLsClkEdgeCfgUpdateW::new(self, 8)
    }
    #[doc = "Bits 9:12 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_l(&mut self) -> SdioLsClkEdgeLW<'_, PeriClkCtrl02Spec> {
        SdioLsClkEdgeLW::new(self, 9)
    }
    #[doc = "Bits 13:16 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_h(&mut self) -> SdioLsClkEdgeHW<'_, PeriClkCtrl02Spec> {
        SdioLsClkEdgeHW::new(self, 13)
    }
    #[doc = "Bits 17:20 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_n(&mut self) -> SdioLsClkEdgeNW<'_, PeriClkCtrl02Spec> {
        SdioLsClkEdgeNW::new(self, 17)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_edge_sel(&mut self) -> SdioLsSlfClkEdgeSelW<'_, PeriClkCtrl02Spec> {
        SdioLsSlfClkEdgeSelW::new(self, 21)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_edge_sel(&mut self) -> SdioLsDrvClkEdgeSelW<'_, PeriClkCtrl02Spec> {
        SdioLsDrvClkEdgeSelW::new(self, 23)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_edge_sel(&mut self) -> SdioLsSamClkEdgeSelW<'_, PeriClkCtrl02Spec> {
        SdioLsSamClkEdgeSelW::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_en(&mut self) -> SdioLsSlfClkEnW<'_, PeriClkCtrl02Spec> {
        SdioLsSlfClkEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_en(&mut self) -> SdioLsDrvClkEnW<'_, PeriClkCtrl02Spec> {
        SdioLsDrvClkEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_en(&mut self) -> SdioLsSamClkEnW<'_, PeriClkCtrl02Spec> {
        SdioLsSamClkEnW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_clk_src_sel(&mut self) -> MipiDsiDphyClkSrcSelW<'_, PeriClkCtrl02Spec> {
        MipiDsiDphyClkSrcSelW::new(self, 30)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl02Spec;
impl crate::RegisterSpec for PeriClkCtrl02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl02::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl02Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl02::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl02Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL02 to value 0"]
impl crate::Resettable for PeriClkCtrl02Spec {}
