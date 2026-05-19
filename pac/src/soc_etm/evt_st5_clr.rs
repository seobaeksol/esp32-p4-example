#[doc = "Register `EVT_ST5_CLR` writer"]
pub type W = crate::W<EvtSt5ClrSpec>;
#[doc = "Field `ULP_EVT_ERR_INTR_ST_CLR` writer - Configures whether or not to clear ULP_evt_err_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type UlpEvtErrIntrStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_HALT_ST_CLR` writer - Configures whether or not to clear ULP_evt_halt trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type UlpEvtHaltStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_START_INTR_ST_CLR` writer - Configures whether or not to clear ULP_evt_start_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type UlpEvtStartIntrStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_TICK_ST_CLR` writer - Configures whether or not to clear RTC_evt_tick trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RtcEvtTickStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_OVF_ST_CLR` writer - Configures whether or not to clear RTC_evt_ovf trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RtcEvtOvfStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_CMP_ST_CLR` writer - Configures whether or not to clear RTC_evt_cmp trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RtcEvtCmpStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInDoneCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInDoneCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInDoneCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInSucEofCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInSucEofCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInSucEofCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInFifoEmptyCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInFifoEmptyCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInFifoEmptyCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInFifoFullCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInFifoFullCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtInFifoFullCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutDoneCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutDoneCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutDoneCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutEofCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutEofCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutEofCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutTotalEofCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutTotalEofCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutTotalEofCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutFifoEmptyCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutFifoEmptyCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutFifoEmptyCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutFifoFullCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbEvtOutFifoFullCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear ULP_evt_err_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st_clr(&mut self) -> UlpEvtErrIntrStClrW<'_, EvtSt5ClrSpec> {
        UlpEvtErrIntrStClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear ULP_evt_halt trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_evt_halt_st_clr(&mut self) -> UlpEvtHaltStClrW<'_, EvtSt5ClrSpec> {
        UlpEvtHaltStClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear ULP_evt_start_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st_clr(&mut self) -> UlpEvtStartIntrStClrW<'_, EvtSt5ClrSpec> {
        UlpEvtStartIntrStClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear RTC_evt_tick trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_evt_tick_st_clr(&mut self) -> RtcEvtTickStClrW<'_, EvtSt5ClrSpec> {
        RtcEvtTickStClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear RTC_evt_ovf trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st_clr(&mut self) -> RtcEvtOvfStClrW<'_, EvtSt5ClrSpec> {
        RtcEvtOvfStClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear RTC_evt_cmp trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st_clr(&mut self) -> RtcEvtCmpStClrW<'_, EvtSt5ClrSpec> {
        RtcEvtCmpStClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInDoneCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInDoneCh0StClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInDoneCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInDoneCh1StClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInDoneCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInDoneCh2StClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInSucEofCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInSucEofCh0StClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInSucEofCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInSucEofCh1StClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInSucEofCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInSucEofCh2StClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInFifoEmptyCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInFifoEmptyCh0StClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInFifoEmptyCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInFifoEmptyCh1StClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInFifoEmptyCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInFifoEmptyCh2StClrW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInFifoFullCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInFifoFullCh0StClrW::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInFifoFullCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInFifoFullCh1StClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtInFifoFullCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtInFifoFullCh2StClrW::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutDoneCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutDoneCh0StClrW::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutDoneCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutDoneCh1StClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutDoneCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutDoneCh2StClrW::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutEofCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutEofCh0StClrW::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutEofCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutEofCh1StClrW::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutEofCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutEofCh2StClrW::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutTotalEofCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutTotalEofCh0StClrW::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutTotalEofCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutTotalEofCh1StClrW::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutTotalEofCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutTotalEofCh2StClrW::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutFifoEmptyCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutFifoEmptyCh0StClrW::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutFifoEmptyCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutFifoEmptyCh1StClrW::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutFifoEmptyCh2StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutFifoEmptyCh2StClrW::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutFifoFullCh0StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutFifoFullCh0StClrW::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbEvtOutFifoFullCh1StClrW<'_, EvtSt5ClrSpec> {
        PdmaAhbEvtOutFifoFullCh1StClrW::new(self, 31)
    }
}
#[doc = "Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st5_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtSt5ClrSpec;
impl crate::RegisterSpec for EvtSt5ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evt_st5_clr::W`](W) writer structure"]
impl crate::Writable for EvtSt5ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST5_CLR to value 0"]
impl crate::Resettable for EvtSt5ClrSpec {}
