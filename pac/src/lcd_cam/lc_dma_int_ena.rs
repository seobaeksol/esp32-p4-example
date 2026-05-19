#[doc = "Register `LC_DMA_INT_ENA` reader"]
pub type R = crate::R<LcDmaIntEnaSpec>;
#[doc = "Register `LC_DMA_INT_ENA` writer"]
pub type W = crate::W<LcDmaIntEnaSpec>;
#[doc = "Field `LCD_VSYNC_INT_ENA` reader - The enable bit for LCD frame end interrupt."]
pub type LcdVsyncIntEnaR = crate::BitReader;
#[doc = "Field `LCD_VSYNC_INT_ENA` writer - The enable bit for LCD frame end interrupt."]
pub type LcdVsyncIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_TRANS_DONE_INT_ENA` reader - The enable bit for lcd transfer end interrupt."]
pub type LcdTransDoneIntEnaR = crate::BitReader;
#[doc = "Field `LCD_TRANS_DONE_INT_ENA` writer - The enable bit for lcd transfer end interrupt."]
pub type LcdTransDoneIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VSYNC_INT_ENA` reader - The enable bit for Camera frame end interrupt."]
pub type CamVsyncIntEnaR = crate::BitReader;
#[doc = "Field `CAM_VSYNC_INT_ENA` writer - The enable bit for Camera frame end interrupt."]
pub type CamVsyncIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_HS_INT_ENA` reader - The enable bit for Camera line interrupt."]
pub type CamHsIntEnaR = crate::BitReader;
#[doc = "Field `CAM_HS_INT_ENA` writer - The enable bit for Camera line interrupt."]
pub type CamHsIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_UNDERRUN_INT_ENA` reader - The enable bit for LCD underrun interrupt"]
pub type LcdUnderrunIntEnaR = crate::BitReader;
#[doc = "Field `LCD_UNDERRUN_INT_ENA` writer - The enable bit for LCD underrun interrupt"]
pub type LcdUnderrunIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_ena(&self) -> LcdVsyncIntEnaR {
        LcdVsyncIntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_ena(&self) -> LcdTransDoneIntEnaR {
        LcdTransDoneIntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_ena(&self) -> CamVsyncIntEnaR {
        CamVsyncIntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for Camera line interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_ena(&self) -> CamHsIntEnaR {
        CamHsIntEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for LCD underrun interrupt"]
    #[inline(always)]
    pub fn lcd_underrun_int_ena(&self) -> LcdUnderrunIntEnaR {
        LcdUnderrunIntEnaR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_ena(&mut self) -> LcdVsyncIntEnaW<'_, LcDmaIntEnaSpec> {
        LcdVsyncIntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - The enable bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_ena(&mut self) -> LcdTransDoneIntEnaW<'_, LcDmaIntEnaSpec> {
        LcdTransDoneIntEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - The enable bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_ena(&mut self) -> CamVsyncIntEnaW<'_, LcDmaIntEnaSpec> {
        CamVsyncIntEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - The enable bit for Camera line interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_ena(&mut self) -> CamHsIntEnaW<'_, LcDmaIntEnaSpec> {
        CamHsIntEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for LCD underrun interrupt"]
    #[inline(always)]
    pub fn lcd_underrun_int_ena(&mut self) -> LcdUnderrunIntEnaW<'_, LcDmaIntEnaSpec> {
        LcdUnderrunIntEnaW::new(self, 4)
    }
}
#[doc = "LCDCAM interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_dma_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc_dma_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcDmaIntEnaSpec;
impl crate::RegisterSpec for LcDmaIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_dma_int_ena::R`](R) reader structure"]
impl crate::Readable for LcDmaIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`lc_dma_int_ena::W`](W) writer structure"]
impl crate::Writable for LcDmaIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LC_DMA_INT_ENA to value 0"]
impl crate::Resettable for LcDmaIntEnaSpec {}
