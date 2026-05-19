#[doc = "Register `EVT_ST7` reader"]
pub type R = crate::R<EvtSt7Spec>;
#[doc = "Register `EVT_ST7` writer"]
pub type W = crate::W<EvtSt7Spec>;
#[doc = "Field `DMA2D_EVT_IN_SUC_EOF_CH1_ST` reader - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtInSucEofCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_IN_SUC_EOF_CH1_ST` writer - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtInSucEofCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH0_ST` reader - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutDoneCh0StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH0_ST` writer - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutDoneCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH1_ST` reader - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutDoneCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH1_ST` writer - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutDoneCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH2_ST` reader - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutDoneCh2StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH2_ST` writer - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutDoneCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH0_ST` reader - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutEofCh0StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH0_ST` writer - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutEofCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH1_ST` reader - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutEofCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH1_ST` writer - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutEofCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH2_ST` reader - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutEofCh2StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH2_ST` writer - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutEofCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST` reader - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutTotalEofCh0StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST` writer - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutTotalEofCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST` reader - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutTotalEofCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST` writer - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutTotalEofCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST` reader - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutTotalEofCh2StR = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST` writer - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dEvtOutTotalEofCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_in_suc_eof_ch1_st(&self) -> Dma2dEvtInSucEofCh1StR {
        Dma2dEvtInSucEofCh1StR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch0_st(&self) -> Dma2dEvtOutDoneCh0StR {
        Dma2dEvtOutDoneCh0StR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch1_st(&self) -> Dma2dEvtOutDoneCh1StR {
        Dma2dEvtOutDoneCh1StR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch2_st(&self) -> Dma2dEvtOutDoneCh2StR {
        Dma2dEvtOutDoneCh2StR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch0_st(&self) -> Dma2dEvtOutEofCh0StR {
        Dma2dEvtOutEofCh0StR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch1_st(&self) -> Dma2dEvtOutEofCh1StR {
        Dma2dEvtOutEofCh1StR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch2_st(&self) -> Dma2dEvtOutEofCh2StR {
        Dma2dEvtOutEofCh2StR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch0_st(&self) -> Dma2dEvtOutTotalEofCh0StR {
        Dma2dEvtOutTotalEofCh0StR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch1_st(&self) -> Dma2dEvtOutTotalEofCh1StR {
        Dma2dEvtOutTotalEofCh1StR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch2_st(&self) -> Dma2dEvtOutTotalEofCh2StR {
        Dma2dEvtOutTotalEofCh2StR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_in_suc_eof_ch1_st(&mut self) -> Dma2dEvtInSucEofCh1StW<'_, EvtSt7Spec> {
        Dma2dEvtInSucEofCh1StW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch0_st(&mut self) -> Dma2dEvtOutDoneCh0StW<'_, EvtSt7Spec> {
        Dma2dEvtOutDoneCh0StW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch1_st(&mut self) -> Dma2dEvtOutDoneCh1StW<'_, EvtSt7Spec> {
        Dma2dEvtOutDoneCh1StW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch2_st(&mut self) -> Dma2dEvtOutDoneCh2StW<'_, EvtSt7Spec> {
        Dma2dEvtOutDoneCh2StW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch0_st(&mut self) -> Dma2dEvtOutEofCh0StW<'_, EvtSt7Spec> {
        Dma2dEvtOutEofCh0StW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch1_st(&mut self) -> Dma2dEvtOutEofCh1StW<'_, EvtSt7Spec> {
        Dma2dEvtOutEofCh1StW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch2_st(&mut self) -> Dma2dEvtOutEofCh2StW<'_, EvtSt7Spec> {
        Dma2dEvtOutEofCh2StW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch0_st(&mut self) -> Dma2dEvtOutTotalEofCh0StW<'_, EvtSt7Spec> {
        Dma2dEvtOutTotalEofCh0StW::new(self, 7)
    }
    #[doc = "Bit 8 - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch1_st(&mut self) -> Dma2dEvtOutTotalEofCh1StW<'_, EvtSt7Spec> {
        Dma2dEvtOutTotalEofCh1StW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch2_st(&mut self) -> Dma2dEvtOutTotalEofCh2StW<'_, EvtSt7Spec> {
        Dma2dEvtOutTotalEofCh2StW::new(self, 9)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtSt7Spec;
impl crate::RegisterSpec for EvtSt7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st7::R`](R) reader structure"]
impl crate::Readable for EvtSt7Spec {}
#[doc = "`write(|w| ..)` method takes [`evt_st7::W`](W) writer structure"]
impl crate::Writable for EvtSt7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST7 to value 0"]
impl crate::Resettable for EvtSt7Spec {}
