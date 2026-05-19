#[doc = "Register `EVT_ST5` reader"]
pub type R = crate::R<EvtSt5Spec>;
#[doc = "Register `EVT_ST5` writer"]
pub type W = crate::W<EvtSt5Spec>;
#[doc = "Field `ULP_EVT_ERR_INTR_ST` reader - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpEvtErrIntrStR = crate::BitReader;
#[doc = "Field `ULP_EVT_ERR_INTR_ST` writer - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpEvtErrIntrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_HALT_ST` reader - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpEvtHaltStR = crate::BitReader;
#[doc = "Field `ULP_EVT_HALT_ST` writer - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpEvtHaltStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_START_INTR_ST` reader - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpEvtStartIntrStR = crate::BitReader;
#[doc = "Field `ULP_EVT_START_INTR_ST` writer - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpEvtStartIntrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_TICK_ST` reader - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcEvtTickStR = crate::BitReader;
#[doc = "Field `RTC_EVT_TICK_ST` writer - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcEvtTickStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_OVF_ST` reader - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcEvtOvfStR = crate::BitReader;
#[doc = "Field `RTC_EVT_OVF_ST` writer - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcEvtOvfStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_CMP_ST` reader - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcEvtCmpStR = crate::BitReader;
#[doc = "Field `RTC_EVT_CMP_ST` writer - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcEvtCmpStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH0_ST` reader - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInDoneCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH0_ST` writer - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInDoneCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH1_ST` reader - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInDoneCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH1_ST` writer - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInDoneCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH2_ST` reader - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInDoneCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH2_ST` writer - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInDoneCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST` reader - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInSucEofCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST` writer - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInSucEofCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST` reader - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInSucEofCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST` writer - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInSucEofCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST` reader - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInSucEofCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST` writer - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInSucEofCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST` reader - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoEmptyCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST` writer - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoEmptyCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST` reader - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoEmptyCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST` writer - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoEmptyCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST` reader - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoEmptyCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST` writer - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoEmptyCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST` reader - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoFullCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST` writer - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoFullCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST` reader - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoFullCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST` writer - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoFullCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST` reader - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoFullCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST` writer - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtInFifoFullCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH0_ST` reader - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutDoneCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH0_ST` writer - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutDoneCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH1_ST` reader - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutDoneCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH1_ST` writer - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutDoneCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH2_ST` reader - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutDoneCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH2_ST` writer - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutDoneCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH0_ST` reader - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutEofCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH0_ST` writer - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutEofCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH1_ST` reader - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutEofCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH1_ST` writer - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutEofCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH2_ST` reader - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutEofCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH2_ST` writer - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutEofCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST` reader - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutTotalEofCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST` writer - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutTotalEofCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST` reader - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutTotalEofCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST` writer - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutTotalEofCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST` reader - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutTotalEofCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST` writer - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutTotalEofCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST` reader - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoEmptyCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST` writer - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoEmptyCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST` reader - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoEmptyCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST` writer - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoEmptyCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST` reader - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoEmptyCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST` writer - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoEmptyCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST` reader - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoFullCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST` writer - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoFullCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST` reader - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoFullCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST` writer - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbEvtOutFifoFullCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st(&self) -> UlpEvtErrIntrStR {
        UlpEvtErrIntrStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_halt_st(&self) -> UlpEvtHaltStR {
        UlpEvtHaltStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st(&self) -> UlpEvtStartIntrStR {
        UlpEvtStartIntrStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_tick_st(&self) -> RtcEvtTickStR {
        RtcEvtTickStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st(&self) -> RtcEvtOvfStR {
        RtcEvtOvfStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st(&self) -> RtcEvtCmpStR {
        RtcEvtCmpStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch0_st(&self) -> PdmaAhbEvtInDoneCh0StR {
        PdmaAhbEvtInDoneCh0StR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch1_st(&self) -> PdmaAhbEvtInDoneCh1StR {
        PdmaAhbEvtInDoneCh1StR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch2_st(&self) -> PdmaAhbEvtInDoneCh2StR {
        PdmaAhbEvtInDoneCh2StR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st(&self) -> PdmaAhbEvtInSucEofCh0StR {
        PdmaAhbEvtInSucEofCh0StR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st(&self) -> PdmaAhbEvtInSucEofCh1StR {
        PdmaAhbEvtInSucEofCh1StR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st(&self) -> PdmaAhbEvtInSucEofCh2StR {
        PdmaAhbEvtInSucEofCh2StR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch0_st(&self) -> PdmaAhbEvtInFifoEmptyCh0StR {
        PdmaAhbEvtInFifoEmptyCh0StR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch1_st(&self) -> PdmaAhbEvtInFifoEmptyCh1StR {
        PdmaAhbEvtInFifoEmptyCh1StR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch2_st(&self) -> PdmaAhbEvtInFifoEmptyCh2StR {
        PdmaAhbEvtInFifoEmptyCh2StR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch0_st(&self) -> PdmaAhbEvtInFifoFullCh0StR {
        PdmaAhbEvtInFifoFullCh0StR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch1_st(&self) -> PdmaAhbEvtInFifoFullCh1StR {
        PdmaAhbEvtInFifoFullCh1StR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch2_st(&self) -> PdmaAhbEvtInFifoFullCh2StR {
        PdmaAhbEvtInFifoFullCh2StR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch0_st(&self) -> PdmaAhbEvtOutDoneCh0StR {
        PdmaAhbEvtOutDoneCh0StR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch1_st(&self) -> PdmaAhbEvtOutDoneCh1StR {
        PdmaAhbEvtOutDoneCh1StR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch2_st(&self) -> PdmaAhbEvtOutDoneCh2StR {
        PdmaAhbEvtOutDoneCh2StR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch0_st(&self) -> PdmaAhbEvtOutEofCh0StR {
        PdmaAhbEvtOutEofCh0StR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch1_st(&self) -> PdmaAhbEvtOutEofCh1StR {
        PdmaAhbEvtOutEofCh1StR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch2_st(&self) -> PdmaAhbEvtOutEofCh2StR {
        PdmaAhbEvtOutEofCh2StR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch0_st(&self) -> PdmaAhbEvtOutTotalEofCh0StR {
        PdmaAhbEvtOutTotalEofCh0StR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch1_st(&self) -> PdmaAhbEvtOutTotalEofCh1StR {
        PdmaAhbEvtOutTotalEofCh1StR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch2_st(&self) -> PdmaAhbEvtOutTotalEofCh2StR {
        PdmaAhbEvtOutTotalEofCh2StR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch0_st(&self) -> PdmaAhbEvtOutFifoEmptyCh0StR {
        PdmaAhbEvtOutFifoEmptyCh0StR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch1_st(&self) -> PdmaAhbEvtOutFifoEmptyCh1StR {
        PdmaAhbEvtOutFifoEmptyCh1StR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch2_st(&self) -> PdmaAhbEvtOutFifoEmptyCh2StR {
        PdmaAhbEvtOutFifoEmptyCh2StR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch0_st(&self) -> PdmaAhbEvtOutFifoFullCh0StR {
        PdmaAhbEvtOutFifoFullCh0StR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch1_st(&self) -> PdmaAhbEvtOutFifoFullCh1StR {
        PdmaAhbEvtOutFifoFullCh1StR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st(&mut self) -> UlpEvtErrIntrStW<'_, EvtSt5Spec> {
        UlpEvtErrIntrStW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_halt_st(&mut self) -> UlpEvtHaltStW<'_, EvtSt5Spec> {
        UlpEvtHaltStW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st(&mut self) -> UlpEvtStartIntrStW<'_, EvtSt5Spec> {
        UlpEvtStartIntrStW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_tick_st(&mut self) -> RtcEvtTickStW<'_, EvtSt5Spec> {
        RtcEvtTickStW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st(&mut self) -> RtcEvtOvfStW<'_, EvtSt5Spec> {
        RtcEvtOvfStW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st(&mut self) -> RtcEvtCmpStW<'_, EvtSt5Spec> {
        RtcEvtCmpStW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch0_st(&mut self) -> PdmaAhbEvtInDoneCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInDoneCh0StW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch1_st(&mut self) -> PdmaAhbEvtInDoneCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInDoneCh1StW::new(self, 7)
    }
    #[doc = "Bit 8 - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch2_st(&mut self) -> PdmaAhbEvtInDoneCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInDoneCh2StW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st(&mut self) -> PdmaAhbEvtInSucEofCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInSucEofCh0StW::new(self, 9)
    }
    #[doc = "Bit 10 - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st(&mut self) -> PdmaAhbEvtInSucEofCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInSucEofCh1StW::new(self, 10)
    }
    #[doc = "Bit 11 - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st(&mut self) -> PdmaAhbEvtInSucEofCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInSucEofCh2StW::new(self, 11)
    }
    #[doc = "Bit 12 - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch0_st(
        &mut self,
    ) -> PdmaAhbEvtInFifoEmptyCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInFifoEmptyCh0StW::new(self, 12)
    }
    #[doc = "Bit 13 - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch1_st(
        &mut self,
    ) -> PdmaAhbEvtInFifoEmptyCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInFifoEmptyCh1StW::new(self, 13)
    }
    #[doc = "Bit 14 - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch2_st(
        &mut self,
    ) -> PdmaAhbEvtInFifoEmptyCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInFifoEmptyCh2StW::new(self, 14)
    }
    #[doc = "Bit 15 - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch0_st(
        &mut self,
    ) -> PdmaAhbEvtInFifoFullCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInFifoFullCh0StW::new(self, 15)
    }
    #[doc = "Bit 16 - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch1_st(
        &mut self,
    ) -> PdmaAhbEvtInFifoFullCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInFifoFullCh1StW::new(self, 16)
    }
    #[doc = "Bit 17 - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch2_st(
        &mut self,
    ) -> PdmaAhbEvtInFifoFullCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtInFifoFullCh2StW::new(self, 17)
    }
    #[doc = "Bit 18 - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch0_st(&mut self) -> PdmaAhbEvtOutDoneCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutDoneCh0StW::new(self, 18)
    }
    #[doc = "Bit 19 - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch1_st(&mut self) -> PdmaAhbEvtOutDoneCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutDoneCh1StW::new(self, 19)
    }
    #[doc = "Bit 20 - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch2_st(&mut self) -> PdmaAhbEvtOutDoneCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutDoneCh2StW::new(self, 20)
    }
    #[doc = "Bit 21 - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch0_st(&mut self) -> PdmaAhbEvtOutEofCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutEofCh0StW::new(self, 21)
    }
    #[doc = "Bit 22 - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch1_st(&mut self) -> PdmaAhbEvtOutEofCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutEofCh1StW::new(self, 22)
    }
    #[doc = "Bit 23 - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch2_st(&mut self) -> PdmaAhbEvtOutEofCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutEofCh2StW::new(self, 23)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch0_st(
        &mut self,
    ) -> PdmaAhbEvtOutTotalEofCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutTotalEofCh0StW::new(self, 24)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch1_st(
        &mut self,
    ) -> PdmaAhbEvtOutTotalEofCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutTotalEofCh1StW::new(self, 25)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch2_st(
        &mut self,
    ) -> PdmaAhbEvtOutTotalEofCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutTotalEofCh2StW::new(self, 26)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch0_st(
        &mut self,
    ) -> PdmaAhbEvtOutFifoEmptyCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutFifoEmptyCh0StW::new(self, 27)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch1_st(
        &mut self,
    ) -> PdmaAhbEvtOutFifoEmptyCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutFifoEmptyCh1StW::new(self, 28)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch2_st(
        &mut self,
    ) -> PdmaAhbEvtOutFifoEmptyCh2StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutFifoEmptyCh2StW::new(self, 29)
    }
    #[doc = "Bit 30 - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch0_st(
        &mut self,
    ) -> PdmaAhbEvtOutFifoFullCh0StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutFifoFullCh0StW::new(self, 30)
    }
    #[doc = "Bit 31 - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch1_st(
        &mut self,
    ) -> PdmaAhbEvtOutFifoFullCh1StW<'_, EvtSt5Spec> {
        PdmaAhbEvtOutFifoFullCh1StW::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtSt5Spec;
impl crate::RegisterSpec for EvtSt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st5::R`](R) reader structure"]
impl crate::Readable for EvtSt5Spec {}
#[doc = "`write(|w| ..)` method takes [`evt_st5::W`](W) writer structure"]
impl crate::Writable for EvtSt5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST5 to value 0"]
impl crate::Resettable for EvtSt5Spec {}
