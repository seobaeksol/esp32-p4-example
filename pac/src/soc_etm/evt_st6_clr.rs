#[doc = "Register `EVT_ST6_CLR` writer"]
pub type W = crate::W<EvtSt6ClrSpec>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutFifoFullCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInDoneCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInDoneCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInDoneCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInSucEofCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInSucEofCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_suc_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInSucEofCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInFifoEmptyCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInFifoEmptyCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInFifoEmptyCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInFifoFullCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInFifoFullCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_in_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtInFifoFullCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutDoneCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutDoneCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutDoneCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutEofCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutEofCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutEofCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutTotalEofCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutTotalEofCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutTotalEofCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutFifoEmptyCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutFifoEmptyCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutFifoEmptyCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutFifoFullCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutFifoFullCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_evt_out_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiEvtOutFifoFullCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_EVT_SLEEP_WEEKUP_ST_CLR` writer - Configures whether or not to clear PMU_evt_sleep_weekup trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PmuEvtSleepWeekupStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_IN_DONE_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dEvtInDoneCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_IN_DONE_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dEvtInDoneCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_IN_SUC_EOF_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dEvtInSucEofCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutFifoFullCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAhbEvtOutFifoFullCh2StClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear PDMA_AXI_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_done_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInDoneCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInDoneCh0StClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear PDMA_AXI_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_done_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInDoneCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInDoneCh1StClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear PDMA_AXI_evt_in_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_done_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInDoneCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInDoneCh2StClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear PDMA_AXI_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_suc_eof_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInSucEofCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInSucEofCh0StClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear PDMA_AXI_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_suc_eof_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInSucEofCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInSucEofCh1StClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear PDMA_AXI_evt_in_suc_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_suc_eof_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInSucEofCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInSucEofCh2StClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear PDMA_AXI_evt_in_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInFifoEmptyCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInFifoEmptyCh0StClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear PDMA_AXI_evt_in_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInFifoEmptyCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInFifoEmptyCh1StClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear PDMA_AXI_evt_in_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInFifoEmptyCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInFifoEmptyCh2StClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear PDMA_AXI_evt_in_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInFifoFullCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInFifoFullCh0StClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear PDMA_AXI_evt_in_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInFifoFullCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInFifoFullCh1StClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear PDMA_AXI_evt_in_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_full_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtInFifoFullCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtInFifoFullCh2StClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear PDMA_AXI_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_done_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutDoneCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutDoneCh0StClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear PDMA_AXI_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_done_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutDoneCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutDoneCh1StClrW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear PDMA_AXI_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_done_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutDoneCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutDoneCh2StClrW::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear PDMA_AXI_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_eof_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutEofCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutEofCh0StClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear PDMA_AXI_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_eof_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutEofCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutEofCh1StClrW::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to clear PDMA_AXI_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_eof_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutEofCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutEofCh2StClrW::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to clear PDMA_AXI_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_total_eof_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutTotalEofCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutTotalEofCh0StClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to clear PDMA_AXI_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_total_eof_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutTotalEofCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutTotalEofCh1StClrW::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to clear PDMA_AXI_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_total_eof_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutTotalEofCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutTotalEofCh2StClrW::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to clear PDMA_AXI_evt_out_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutFifoEmptyCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutFifoEmptyCh0StClrW::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to clear PDMA_AXI_evt_out_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutFifoEmptyCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutFifoEmptyCh1StClrW::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to clear PDMA_AXI_evt_out_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutFifoEmptyCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutFifoEmptyCh2StClrW::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to clear PDMA_AXI_evt_out_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutFifoFullCh0StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutFifoFullCh0StClrW::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to clear PDMA_AXI_evt_out_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutFifoFullCh1StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutFifoFullCh1StClrW::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to clear PDMA_AXI_evt_out_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_full_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiEvtOutFifoFullCh2StClrW<'_, EvtSt6ClrSpec> {
        PdmaAxiEvtOutFifoFullCh2StClrW::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to clear PMU_evt_sleep_weekup trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st_clr(&mut self) -> PmuEvtSleepWeekupStClrW<'_, EvtSt6ClrSpec> {
        PmuEvtSleepWeekupStClrW::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to clear DMA2D_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_in_done_ch0_st_clr(&mut self) -> Dma2dEvtInDoneCh0StClrW<'_, EvtSt6ClrSpec> {
        Dma2dEvtInDoneCh0StClrW::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to clear DMA2D_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_in_done_ch1_st_clr(&mut self) -> Dma2dEvtInDoneCh1StClrW<'_, EvtSt6ClrSpec> {
        Dma2dEvtInDoneCh1StClrW::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to clear DMA2D_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_in_suc_eof_ch0_st_clr(
        &mut self,
    ) -> Dma2dEvtInSucEofCh0StClrW<'_, EvtSt6ClrSpec> {
        Dma2dEvtInSucEofCh0StClrW::new(self, 31)
    }
}
#[doc = "Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st6_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtSt6ClrSpec;
impl crate::RegisterSpec for EvtSt6ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evt_st6_clr::W`](W) writer structure"]
impl crate::Writable for EvtSt6ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST6_CLR to value 0"]
impl crate::Resettable for EvtSt6ClrSpec {}
