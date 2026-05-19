#[doc = "Register `EVT_ST4` reader"]
pub type R = crate::R<EvtSt4Spec>;
#[doc = "Register `EVT_ST4` writer"]
pub type W = crate::W<EvtSt4Spec>;
#[doc = "Field `MCPWM1_EVT_OP0_TEE2_ST` reader - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0Tee2StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP0_TEE2_ST` writer - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0Tee2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP1_TEE2_ST` reader - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1Tee2StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP1_TEE2_ST` writer - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1Tee2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP2_TEE2_ST` reader - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2Tee2StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP2_TEE2_ST` writer - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2Tee2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_CONV_CMPLT0_ST` reader - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtConvCmplt0StR = crate::BitReader;
#[doc = "Field `ADC_EVT_CONV_CMPLT0_ST` writer - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtConvCmplt0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH0_ST` reader - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqAboveThresh0StR = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH0_ST` writer - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqAboveThresh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH1_ST` reader - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqAboveThresh1StR = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH1_ST` writer - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqAboveThresh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH0_ST` reader - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqBelowThresh0StR = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH0_ST` writer - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqBelowThresh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH1_ST` reader - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqBelowThresh1StR = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH1_ST` writer - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtEqBelowThresh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_RESULT_DONE0_ST` reader - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtResultDone0StR = crate::BitReader;
#[doc = "Field `ADC_EVT_RESULT_DONE0_ST` writer - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtResultDone0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_STOPPED0_ST` reader - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtStopped0StR = crate::BitReader;
#[doc = "Field `ADC_EVT_STOPPED0_ST` writer - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtStopped0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_STARTED0_ST` reader - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtStarted0StR = crate::BitReader;
#[doc = "Field `ADC_EVT_STARTED0_ST` writer - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type AdcEvtStarted0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_DONE0_ST` reader - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone0StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE0_ST` writer - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_DONE1_ST` reader - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone1StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE1_ST` writer - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_DONE2_ST` reader - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone2StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE2_ST` writer - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_DONE3_ST` reader - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone3StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE3_ST` writer - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtDone3StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_ERR0_ST` reader - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr0StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR0_ST` writer - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_ERR1_ST` reader - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr1StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR1_ST` writer - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_ERR2_ST` reader - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr2StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR2_ST` writer - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_ERR3_ST` reader - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr3StR = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR3_ST` writer - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaEvtErr3StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_EVT_OVER_LIMIT_ST` reader - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TmpsnsrEvtOverLimitStR = crate::BitReader;
#[doc = "Field `TMPSNSR_EVT_OVER_LIMIT_ST` writer - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TmpsnsrEvtOverLimitStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_RX_DONE_ST` reader - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtRxDoneStR = crate::BitReader;
#[doc = "Field `I2S0_EVT_RX_DONE_ST` writer - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtRxDoneStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_TX_DONE_ST` reader - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtTxDoneStR = crate::BitReader;
#[doc = "Field `I2S0_EVT_TX_DONE_ST` writer - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtTxDoneStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_X_WORDS_RECEIVED_ST` reader - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtXWordsReceivedStR = crate::BitReader;
#[doc = "Field `I2S0_EVT_X_WORDS_RECEIVED_ST` writer - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtXWordsReceivedStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_X_WORDS_SENT_ST` reader - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtXWordsSentStR = crate::BitReader;
#[doc = "Field `I2S0_EVT_X_WORDS_SENT_ST` writer - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0EvtXWordsSentStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_RX_DONE_ST` reader - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtRxDoneStR = crate::BitReader;
#[doc = "Field `I2S1_EVT_RX_DONE_ST` writer - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtRxDoneStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_TX_DONE_ST` reader - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtTxDoneStR = crate::BitReader;
#[doc = "Field `I2S1_EVT_TX_DONE_ST` writer - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtTxDoneStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_X_WORDS_RECEIVED_ST` reader - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtXWordsReceivedStR = crate::BitReader;
#[doc = "Field `I2S1_EVT_X_WORDS_RECEIVED_ST` writer - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtXWordsReceivedStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_X_WORDS_SENT_ST` reader - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtXWordsSentStR = crate::BitReader;
#[doc = "Field `I2S1_EVT_X_WORDS_SENT_ST` writer - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1EvtXWordsSentStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_EVT_RX_DONE_ST` reader - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtRxDoneStR = crate::BitReader;
#[doc = "Field `I2S2_EVT_RX_DONE_ST` writer - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtRxDoneStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_EVT_TX_DONE_ST` reader - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtTxDoneStR = crate::BitReader;
#[doc = "Field `I2S2_EVT_TX_DONE_ST` writer - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtTxDoneStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_EVT_X_WORDS_RECEIVED_ST` reader - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtXWordsReceivedStR = crate::BitReader;
#[doc = "Field `I2S2_EVT_X_WORDS_RECEIVED_ST` writer - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtXWordsReceivedStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_EVT_X_WORDS_SENT_ST` reader - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtXWordsSentStR = crate::BitReader;
#[doc = "Field `I2S2_EVT_X_WORDS_SENT_ST` writer - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2EvtXWordsSentStW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_tee2_st(&self) -> Mcpwm1EvtOp0Tee2StR {
        Mcpwm1EvtOp0Tee2StR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_tee2_st(&self) -> Mcpwm1EvtOp1Tee2StR {
        Mcpwm1EvtOp1Tee2StR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_tee2_st(&self) -> Mcpwm1EvtOp2Tee2StR {
        Mcpwm1EvtOp2Tee2StR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_conv_cmplt0_st(&self) -> AdcEvtConvCmplt0StR {
        AdcEvtConvCmplt0StR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_above_thresh0_st(&self) -> AdcEvtEqAboveThresh0StR {
        AdcEvtEqAboveThresh0StR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_above_thresh1_st(&self) -> AdcEvtEqAboveThresh1StR {
        AdcEvtEqAboveThresh1StR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_below_thresh0_st(&self) -> AdcEvtEqBelowThresh0StR {
        AdcEvtEqBelowThresh0StR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_below_thresh1_st(&self) -> AdcEvtEqBelowThresh1StR {
        AdcEvtEqBelowThresh1StR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_result_done0_st(&self) -> AdcEvtResultDone0StR {
        AdcEvtResultDone0StR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_stopped0_st(&self) -> AdcEvtStopped0StR {
        AdcEvtStopped0StR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_started0_st(&self) -> AdcEvtStarted0StR {
        AdcEvtStarted0StR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done0_st(&self) -> RegdmaEvtDone0StR {
        RegdmaEvtDone0StR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done1_st(&self) -> RegdmaEvtDone1StR {
        RegdmaEvtDone1StR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done2_st(&self) -> RegdmaEvtDone2StR {
        RegdmaEvtDone2StR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done3_st(&self) -> RegdmaEvtDone3StR {
        RegdmaEvtDone3StR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err0_st(&self) -> RegdmaEvtErr0StR {
        RegdmaEvtErr0StR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err1_st(&self) -> RegdmaEvtErr1StR {
        RegdmaEvtErr1StR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err2_st(&self) -> RegdmaEvtErr2StR {
        RegdmaEvtErr2StR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err3_st(&self) -> RegdmaEvtErr3StR {
        RegdmaEvtErr3StR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_evt_over_limit_st(&self) -> TmpsnsrEvtOverLimitStR {
        TmpsnsrEvtOverLimitStR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_rx_done_st(&self) -> I2s0EvtRxDoneStR {
        I2s0EvtRxDoneStR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_tx_done_st(&self) -> I2s0EvtTxDoneStR {
        I2s0EvtTxDoneStR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_received_st(&self) -> I2s0EvtXWordsReceivedStR {
        I2s0EvtXWordsReceivedStR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_sent_st(&self) -> I2s0EvtXWordsSentStR {
        I2s0EvtXWordsSentStR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_rx_done_st(&self) -> I2s1EvtRxDoneStR {
        I2s1EvtRxDoneStR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_tx_done_st(&self) -> I2s1EvtTxDoneStR {
        I2s1EvtTxDoneStR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_received_st(&self) -> I2s1EvtXWordsReceivedStR {
        I2s1EvtXWordsReceivedStR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_sent_st(&self) -> I2s1EvtXWordsSentStR {
        I2s1EvtXWordsSentStR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_rx_done_st(&self) -> I2s2EvtRxDoneStR {
        I2s2EvtRxDoneStR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_tx_done_st(&self) -> I2s2EvtTxDoneStR {
        I2s2EvtTxDoneStR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_x_words_received_st(&self) -> I2s2EvtXWordsReceivedStR {
        I2s2EvtXWordsReceivedStR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_x_words_sent_st(&self) -> I2s2EvtXWordsSentStR {
        I2s2EvtXWordsSentStR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_tee2_st(&mut self) -> Mcpwm1EvtOp0Tee2StW<'_, EvtSt4Spec> {
        Mcpwm1EvtOp0Tee2StW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_tee2_st(&mut self) -> Mcpwm1EvtOp1Tee2StW<'_, EvtSt4Spec> {
        Mcpwm1EvtOp1Tee2StW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_tee2_st(&mut self) -> Mcpwm1EvtOp2Tee2StW<'_, EvtSt4Spec> {
        Mcpwm1EvtOp2Tee2StW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_conv_cmplt0_st(&mut self) -> AdcEvtConvCmplt0StW<'_, EvtSt4Spec> {
        AdcEvtConvCmplt0StW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_above_thresh0_st(&mut self) -> AdcEvtEqAboveThresh0StW<'_, EvtSt4Spec> {
        AdcEvtEqAboveThresh0StW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_above_thresh1_st(&mut self) -> AdcEvtEqAboveThresh1StW<'_, EvtSt4Spec> {
        AdcEvtEqAboveThresh1StW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_below_thresh0_st(&mut self) -> AdcEvtEqBelowThresh0StW<'_, EvtSt4Spec> {
        AdcEvtEqBelowThresh0StW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_below_thresh1_st(&mut self) -> AdcEvtEqBelowThresh1StW<'_, EvtSt4Spec> {
        AdcEvtEqBelowThresh1StW::new(self, 7)
    }
    #[doc = "Bit 8 - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_result_done0_st(&mut self) -> AdcEvtResultDone0StW<'_, EvtSt4Spec> {
        AdcEvtResultDone0StW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_stopped0_st(&mut self) -> AdcEvtStopped0StW<'_, EvtSt4Spec> {
        AdcEvtStopped0StW::new(self, 9)
    }
    #[doc = "Bit 10 - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_started0_st(&mut self) -> AdcEvtStarted0StW<'_, EvtSt4Spec> {
        AdcEvtStarted0StW::new(self, 10)
    }
    #[doc = "Bit 11 - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done0_st(&mut self) -> RegdmaEvtDone0StW<'_, EvtSt4Spec> {
        RegdmaEvtDone0StW::new(self, 11)
    }
    #[doc = "Bit 12 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done1_st(&mut self) -> RegdmaEvtDone1StW<'_, EvtSt4Spec> {
        RegdmaEvtDone1StW::new(self, 12)
    }
    #[doc = "Bit 13 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done2_st(&mut self) -> RegdmaEvtDone2StW<'_, EvtSt4Spec> {
        RegdmaEvtDone2StW::new(self, 13)
    }
    #[doc = "Bit 14 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done3_st(&mut self) -> RegdmaEvtDone3StW<'_, EvtSt4Spec> {
        RegdmaEvtDone3StW::new(self, 14)
    }
    #[doc = "Bit 15 - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err0_st(&mut self) -> RegdmaEvtErr0StW<'_, EvtSt4Spec> {
        RegdmaEvtErr0StW::new(self, 15)
    }
    #[doc = "Bit 16 - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err1_st(&mut self) -> RegdmaEvtErr1StW<'_, EvtSt4Spec> {
        RegdmaEvtErr1StW::new(self, 16)
    }
    #[doc = "Bit 17 - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err2_st(&mut self) -> RegdmaEvtErr2StW<'_, EvtSt4Spec> {
        RegdmaEvtErr2StW::new(self, 17)
    }
    #[doc = "Bit 18 - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err3_st(&mut self) -> RegdmaEvtErr3StW<'_, EvtSt4Spec> {
        RegdmaEvtErr3StW::new(self, 18)
    }
    #[doc = "Bit 19 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_evt_over_limit_st(&mut self) -> TmpsnsrEvtOverLimitStW<'_, EvtSt4Spec> {
        TmpsnsrEvtOverLimitStW::new(self, 19)
    }
    #[doc = "Bit 20 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_rx_done_st(&mut self) -> I2s0EvtRxDoneStW<'_, EvtSt4Spec> {
        I2s0EvtRxDoneStW::new(self, 20)
    }
    #[doc = "Bit 21 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_tx_done_st(&mut self) -> I2s0EvtTxDoneStW<'_, EvtSt4Spec> {
        I2s0EvtTxDoneStW::new(self, 21)
    }
    #[doc = "Bit 22 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_received_st(&mut self) -> I2s0EvtXWordsReceivedStW<'_, EvtSt4Spec> {
        I2s0EvtXWordsReceivedStW::new(self, 22)
    }
    #[doc = "Bit 23 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_sent_st(&mut self) -> I2s0EvtXWordsSentStW<'_, EvtSt4Spec> {
        I2s0EvtXWordsSentStW::new(self, 23)
    }
    #[doc = "Bit 24 - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_rx_done_st(&mut self) -> I2s1EvtRxDoneStW<'_, EvtSt4Spec> {
        I2s1EvtRxDoneStW::new(self, 24)
    }
    #[doc = "Bit 25 - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_tx_done_st(&mut self) -> I2s1EvtTxDoneStW<'_, EvtSt4Spec> {
        I2s1EvtTxDoneStW::new(self, 25)
    }
    #[doc = "Bit 26 - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_received_st(&mut self) -> I2s1EvtXWordsReceivedStW<'_, EvtSt4Spec> {
        I2s1EvtXWordsReceivedStW::new(self, 26)
    }
    #[doc = "Bit 27 - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_sent_st(&mut self) -> I2s1EvtXWordsSentStW<'_, EvtSt4Spec> {
        I2s1EvtXWordsSentStW::new(self, 27)
    }
    #[doc = "Bit 28 - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_rx_done_st(&mut self) -> I2s2EvtRxDoneStW<'_, EvtSt4Spec> {
        I2s2EvtRxDoneStW::new(self, 28)
    }
    #[doc = "Bit 29 - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_tx_done_st(&mut self) -> I2s2EvtTxDoneStW<'_, EvtSt4Spec> {
        I2s2EvtTxDoneStW::new(self, 29)
    }
    #[doc = "Bit 30 - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_x_words_received_st(&mut self) -> I2s2EvtXWordsReceivedStW<'_, EvtSt4Spec> {
        I2s2EvtXWordsReceivedStW::new(self, 30)
    }
    #[doc = "Bit 31 - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_x_words_sent_st(&mut self) -> I2s2EvtXWordsSentStW<'_, EvtSt4Spec> {
        I2s2EvtXWordsSentStW::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtSt4Spec;
impl crate::RegisterSpec for EvtSt4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st4::R`](R) reader structure"]
impl crate::Readable for EvtSt4Spec {}
#[doc = "`write(|w| ..)` method takes [`evt_st4::W`](W) writer structure"]
impl crate::Writable for EvtSt4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST4 to value 0"]
impl crate::Resettable for EvtSt4Spec {}
