#[doc = "Register `LC_DMA_INT_ST` reader"]
pub type R = crate::R<LcDmaIntStSpec>;
#[doc = "Field `LCD_VSYNC_INT_ST` reader - The status bit for LCD frame end interrupt."]
pub type LcdVsyncIntStR = crate::BitReader;
#[doc = "Field `LCD_TRANS_DONE_INT_ST` reader - The status bit for lcd transfer end interrupt."]
pub type LcdTransDoneIntStR = crate::BitReader;
#[doc = "Field `CAM_VSYNC_INT_ST` reader - The status bit for Camera frame end interrupt."]
pub type CamVsyncIntStR = crate::BitReader;
#[doc = "Field `CAM_HS_INT_ST` reader - The status bit for Camera transfer end interrupt."]
pub type CamHsIntStR = crate::BitReader;
#[doc = "Field `LCD_UNDERRUN_INT_ST` reader - The status bit for LCD underrun interrupt"]
pub type LcdUnderrunIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_st(&self) -> LcdVsyncIntStR {
        LcdVsyncIntStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_st(&self) -> LcdTransDoneIntStR {
        LcdTransDoneIntStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_st(&self) -> CamVsyncIntStR {
        CamVsyncIntStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for Camera transfer end interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_st(&self) -> CamHsIntStR {
        CamHsIntStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for LCD underrun interrupt"]
    #[inline(always)]
    pub fn lcd_underrun_int_st(&self) -> LcdUnderrunIntStR {
        LcdUnderrunIntStR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "LCDCAM interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_dma_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcDmaIntStSpec;
impl crate::RegisterSpec for LcDmaIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_dma_int_st::R`](R) reader structure"]
impl crate::Readable for LcDmaIntStSpec {}
#[doc = "`reset()` method sets LC_DMA_INT_ST to value 0"]
impl crate::Resettable for LcDmaIntStSpec {}
