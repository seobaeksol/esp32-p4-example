#[doc = "Register `TASK_ST5` reader"]
pub type R = crate::R<TaskSt5Spec>;
#[doc = "Register `TASK_ST5` writer"]
pub type W = crate::W<TaskSt5Spec>;
#[doc = "Field `REGDMA_TASK_START0_ST` reader - Represents REGDMA_task_start0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart0StR = crate::BitReader;
#[doc = "Field `REGDMA_TASK_START0_ST` writer - Represents REGDMA_task_start0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START1_ST` reader - Represents REGDMA_task_start1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart1StR = crate::BitReader;
#[doc = "Field `REGDMA_TASK_START1_ST` writer - Represents REGDMA_task_start1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START2_ST` reader - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart2StR = crate::BitReader;
#[doc = "Field `REGDMA_TASK_START2_ST` writer - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START3_ST` reader - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart3StR = crate::BitReader;
#[doc = "Field `REGDMA_TASK_START3_ST` writer - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RegdmaTaskStart3StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_START_SAMPLE_ST` reader - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TmpsnsrTaskStartSampleStR = crate::BitReader;
#[doc = "Field `TMPSNSR_TASK_START_SAMPLE_ST` writer - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TmpsnsrTaskStartSampleStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_STOP_SAMPLE_ST` reader - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TmpsnsrTaskStopSampleStR = crate::BitReader;
#[doc = "Field `TMPSNSR_TASK_STOP_SAMPLE_ST` writer - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TmpsnsrTaskStopSampleStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_RX_ST` reader - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStartRxStR = crate::BitReader;
#[doc = "Field `I2S0_TASK_START_RX_ST` writer - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStartRxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_TX_ST` reader - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStartTxStR = crate::BitReader;
#[doc = "Field `I2S0_TASK_START_TX_ST` writer - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStartTxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_RX_ST` reader - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStopRxStR = crate::BitReader;
#[doc = "Field `I2S0_TASK_STOP_RX_ST` writer - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStopRxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_TX_ST` reader - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStopTxStR = crate::BitReader;
#[doc = "Field `I2S0_TASK_STOP_TX_ST` writer - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s0TaskStopTxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_START_RX_ST` reader - Represents I2S1_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStartRxStR = crate::BitReader;
#[doc = "Field `I2S1_TASK_START_RX_ST` writer - Represents I2S1_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStartRxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_START_TX_ST` reader - Represents I2S1_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStartTxStR = crate::BitReader;
#[doc = "Field `I2S1_TASK_START_TX_ST` writer - Represents I2S1_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStartTxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_STOP_RX_ST` reader - Represents I2S1_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStopRxStR = crate::BitReader;
#[doc = "Field `I2S1_TASK_STOP_RX_ST` writer - Represents I2S1_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStopRxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_STOP_TX_ST` reader - Represents I2S1_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStopTxStR = crate::BitReader;
#[doc = "Field `I2S1_TASK_STOP_TX_ST` writer - Represents I2S1_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s1TaskStopTxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_START_RX_ST` reader - Represents I2S2_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStartRxStR = crate::BitReader;
#[doc = "Field `I2S2_TASK_START_RX_ST` writer - Represents I2S2_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStartRxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_START_TX_ST` reader - Represents I2S2_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStartTxStR = crate::BitReader;
#[doc = "Field `I2S2_TASK_START_TX_ST` writer - Represents I2S2_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStartTxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_STOP_RX_ST` reader - Represents I2S2_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStopRxStR = crate::BitReader;
#[doc = "Field `I2S2_TASK_STOP_RX_ST` writer - Represents I2S2_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStopRxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_STOP_TX_ST` reader - Represents I2S2_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStopTxStR = crate::BitReader;
#[doc = "Field `I2S2_TASK_STOP_TX_ST` writer - Represents I2S2_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2s2TaskStopTxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_WAKEUP_CPU_ST` reader - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpTaskWakeupCpuStR = crate::BitReader;
#[doc = "Field `ULP_TASK_WAKEUP_CPU_ST` writer - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpTaskWakeupCpuStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_INT_CPU_ST` reader - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpTaskIntCpuStR = crate::BitReader;
#[doc = "Field `ULP_TASK_INT_CPU_ST` writer - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type UlpTaskIntCpuStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_START_ST` reader - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskStartStR = crate::BitReader;
#[doc = "Field `RTC_TASK_START_ST` writer - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskStartStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_STOP_ST` reader - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskStopStR = crate::BitReader;
#[doc = "Field `RTC_TASK_STOP_ST` writer - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskStopStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_CLR_ST` reader - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskClrStR = crate::BitReader;
#[doc = "Field `RTC_TASK_CLR_ST` writer - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskClrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_TRIGGERFLW_ST` reader - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskTriggerflwStR = crate::BitReader;
#[doc = "Field `RTC_TASK_TRIGGERFLW_ST` writer - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RtcTaskTriggerflwStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH0_ST` reader - Represents PDMA_AHB_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskInStartCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH0_ST` writer - Represents PDMA_AHB_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskInStartCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH1_ST` reader - Represents PDMA_AHB_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskInStartCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH1_ST` writer - Represents PDMA_AHB_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskInStartCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH2_ST` reader - Represents PDMA_AHB_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskInStartCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH2_ST` writer - Represents PDMA_AHB_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskInStartCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH0_ST` reader - Represents PDMA_AHB_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskOutStartCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH0_ST` writer - Represents PDMA_AHB_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskOutStartCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH1_ST` reader - Represents PDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskOutStartCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH1_ST` writer - Represents PDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskOutStartCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH2_ST` reader - Represents PDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskOutStartCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH2_ST` writer - Represents PDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAhbTaskOutStartCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH0_ST` reader - Represents PDMA_AXI_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskInStartCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH0_ST` writer - Represents PDMA_AXI_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskInStartCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH1_ST` reader - Represents PDMA_AXI_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskInStartCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH1_ST` writer - Represents PDMA_AXI_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskInStartCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents REGDMA_task_start0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start0_st(&self) -> RegdmaTaskStart0StR {
        RegdmaTaskStart0StR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_task_start1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start1_st(&self) -> RegdmaTaskStart1StR {
        RegdmaTaskStart1StR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start2_st(&self) -> RegdmaTaskStart2StR {
        RegdmaTaskStart2StR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start3_st(&self) -> RegdmaTaskStart3StR {
        RegdmaTaskStart3StR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_start_sample_st(&self) -> TmpsnsrTaskStartSampleStR {
        TmpsnsrTaskStartSampleStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_stop_sample_st(&self) -> TmpsnsrTaskStopSampleStR {
        TmpsnsrTaskStopSampleStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_rx_st(&self) -> I2s0TaskStartRxStR {
        I2s0TaskStartRxStR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_tx_st(&self) -> I2s0TaskStartTxStR {
        I2s0TaskStartTxStR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_rx_st(&self) -> I2s0TaskStopRxStR {
        I2s0TaskStopRxStR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_tx_st(&self) -> I2s0TaskStopTxStR {
        I2s0TaskStopTxStR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents I2S1_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_start_rx_st(&self) -> I2s1TaskStartRxStR {
        I2s1TaskStartRxStR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents I2S1_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_start_tx_st(&self) -> I2s1TaskStartTxStR {
        I2s1TaskStartTxStR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents I2S1_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_stop_rx_st(&self) -> I2s1TaskStopRxStR {
        I2s1TaskStopRxStR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents I2S1_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_stop_tx_st(&self) -> I2s1TaskStopTxStR {
        I2s1TaskStopTxStR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents I2S2_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_start_rx_st(&self) -> I2s2TaskStartRxStR {
        I2s2TaskStartRxStR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents I2S2_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_start_tx_st(&self) -> I2s2TaskStartTxStR {
        I2s2TaskStartTxStR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents I2S2_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_stop_rx_st(&self) -> I2s2TaskStopRxStR {
        I2s2TaskStopRxStR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents I2S2_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_stop_tx_st(&self) -> I2s2TaskStopTxStR {
        I2s2TaskStopTxStR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st(&self) -> UlpTaskWakeupCpuStR {
        UlpTaskWakeupCpuStR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st(&self) -> UlpTaskIntCpuStR {
        UlpTaskIntCpuStR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_start_st(&self) -> RtcTaskStartStR {
        RtcTaskStartStR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_stop_st(&self) -> RtcTaskStopStR {
        RtcTaskStopStR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_clr_st(&self) -> RtcTaskClrStR {
        RtcTaskClrStR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st(&self) -> RtcTaskTriggerflwStR {
        RtcTaskTriggerflwStR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch0_st(&self) -> PdmaAhbTaskInStartCh0StR {
        PdmaAhbTaskInStartCh0StR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch1_st(&self) -> PdmaAhbTaskInStartCh1StR {
        PdmaAhbTaskInStartCh1StR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch2_st(&self) -> PdmaAhbTaskInStartCh2StR {
        PdmaAhbTaskInStartCh2StR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch0_st(&self) -> PdmaAhbTaskOutStartCh0StR {
        PdmaAhbTaskOutStartCh0StR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch1_st(&self) -> PdmaAhbTaskOutStartCh1StR {
        PdmaAhbTaskOutStartCh1StR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch2_st(&self) -> PdmaAhbTaskOutStartCh2StR {
        PdmaAhbTaskOutStartCh2StR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents PDMA_AXI_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch0_st(&self) -> PdmaAxiTaskInStartCh0StR {
        PdmaAxiTaskInStartCh0StR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents PDMA_AXI_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch1_st(&self) -> PdmaAxiTaskInStartCh1StR {
        PdmaAxiTaskInStartCh1StR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Represents REGDMA_task_start0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start0_st(&mut self) -> RegdmaTaskStart0StW<'_, TaskSt5Spec> {
        RegdmaTaskStart0StW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_task_start1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start1_st(&mut self) -> RegdmaTaskStart1StW<'_, TaskSt5Spec> {
        RegdmaTaskStart1StW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start2_st(&mut self) -> RegdmaTaskStart2StW<'_, TaskSt5Spec> {
        RegdmaTaskStart2StW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start3_st(&mut self) -> RegdmaTaskStart3StW<'_, TaskSt5Spec> {
        RegdmaTaskStart3StW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_start_sample_st(&mut self) -> TmpsnsrTaskStartSampleStW<'_, TaskSt5Spec> {
        TmpsnsrTaskStartSampleStW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_stop_sample_st(&mut self) -> TmpsnsrTaskStopSampleStW<'_, TaskSt5Spec> {
        TmpsnsrTaskStopSampleStW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_rx_st(&mut self) -> I2s0TaskStartRxStW<'_, TaskSt5Spec> {
        I2s0TaskStartRxStW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_tx_st(&mut self) -> I2s0TaskStartTxStW<'_, TaskSt5Spec> {
        I2s0TaskStartTxStW::new(self, 7)
    }
    #[doc = "Bit 8 - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_rx_st(&mut self) -> I2s0TaskStopRxStW<'_, TaskSt5Spec> {
        I2s0TaskStopRxStW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_tx_st(&mut self) -> I2s0TaskStopTxStW<'_, TaskSt5Spec> {
        I2s0TaskStopTxStW::new(self, 9)
    }
    #[doc = "Bit 10 - Represents I2S1_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_start_rx_st(&mut self) -> I2s1TaskStartRxStW<'_, TaskSt5Spec> {
        I2s1TaskStartRxStW::new(self, 10)
    }
    #[doc = "Bit 11 - Represents I2S1_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_start_tx_st(&mut self) -> I2s1TaskStartTxStW<'_, TaskSt5Spec> {
        I2s1TaskStartTxStW::new(self, 11)
    }
    #[doc = "Bit 12 - Represents I2S1_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_stop_rx_st(&mut self) -> I2s1TaskStopRxStW<'_, TaskSt5Spec> {
        I2s1TaskStopRxStW::new(self, 12)
    }
    #[doc = "Bit 13 - Represents I2S1_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_task_stop_tx_st(&mut self) -> I2s1TaskStopTxStW<'_, TaskSt5Spec> {
        I2s1TaskStopTxStW::new(self, 13)
    }
    #[doc = "Bit 14 - Represents I2S2_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_start_rx_st(&mut self) -> I2s2TaskStartRxStW<'_, TaskSt5Spec> {
        I2s2TaskStartRxStW::new(self, 14)
    }
    #[doc = "Bit 15 - Represents I2S2_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_start_tx_st(&mut self) -> I2s2TaskStartTxStW<'_, TaskSt5Spec> {
        I2s2TaskStartTxStW::new(self, 15)
    }
    #[doc = "Bit 16 - Represents I2S2_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_stop_rx_st(&mut self) -> I2s2TaskStopRxStW<'_, TaskSt5Spec> {
        I2s2TaskStopRxStW::new(self, 16)
    }
    #[doc = "Bit 17 - Represents I2S2_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_task_stop_tx_st(&mut self) -> I2s2TaskStopTxStW<'_, TaskSt5Spec> {
        I2s2TaskStopTxStW::new(self, 17)
    }
    #[doc = "Bit 18 - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st(&mut self) -> UlpTaskWakeupCpuStW<'_, TaskSt5Spec> {
        UlpTaskWakeupCpuStW::new(self, 18)
    }
    #[doc = "Bit 19 - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st(&mut self) -> UlpTaskIntCpuStW<'_, TaskSt5Spec> {
        UlpTaskIntCpuStW::new(self, 19)
    }
    #[doc = "Bit 20 - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_start_st(&mut self) -> RtcTaskStartStW<'_, TaskSt5Spec> {
        RtcTaskStartStW::new(self, 20)
    }
    #[doc = "Bit 21 - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_stop_st(&mut self) -> RtcTaskStopStW<'_, TaskSt5Spec> {
        RtcTaskStopStW::new(self, 21)
    }
    #[doc = "Bit 22 - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_clr_st(&mut self) -> RtcTaskClrStW<'_, TaskSt5Spec> {
        RtcTaskClrStW::new(self, 22)
    }
    #[doc = "Bit 23 - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st(&mut self) -> RtcTaskTriggerflwStW<'_, TaskSt5Spec> {
        RtcTaskTriggerflwStW::new(self, 23)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch0_st(&mut self) -> PdmaAhbTaskInStartCh0StW<'_, TaskSt5Spec> {
        PdmaAhbTaskInStartCh0StW::new(self, 24)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch1_st(&mut self) -> PdmaAhbTaskInStartCh1StW<'_, TaskSt5Spec> {
        PdmaAhbTaskInStartCh1StW::new(self, 25)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch2_st(&mut self) -> PdmaAhbTaskInStartCh2StW<'_, TaskSt5Spec> {
        PdmaAhbTaskInStartCh2StW::new(self, 26)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch0_st(&mut self) -> PdmaAhbTaskOutStartCh0StW<'_, TaskSt5Spec> {
        PdmaAhbTaskOutStartCh0StW::new(self, 27)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch1_st(&mut self) -> PdmaAhbTaskOutStartCh1StW<'_, TaskSt5Spec> {
        PdmaAhbTaskOutStartCh1StW::new(self, 28)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch2_st(&mut self) -> PdmaAhbTaskOutStartCh2StW<'_, TaskSt5Spec> {
        PdmaAhbTaskOutStartCh2StW::new(self, 29)
    }
    #[doc = "Bit 30 - Represents PDMA_AXI_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch0_st(&mut self) -> PdmaAxiTaskInStartCh0StW<'_, TaskSt5Spec> {
        PdmaAxiTaskInStartCh0StW::new(self, 30)
    }
    #[doc = "Bit 31 - Represents PDMA_AXI_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch1_st(&mut self) -> PdmaAxiTaskInStartCh1StW<'_, TaskSt5Spec> {
        PdmaAxiTaskInStartCh1StW::new(self, 31)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskSt5Spec;
impl crate::RegisterSpec for TaskSt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st5::R`](R) reader structure"]
impl crate::Readable for TaskSt5Spec {}
#[doc = "`write(|w| ..)` method takes [`task_st5::W`](W) writer structure"]
impl crate::Writable for TaskSt5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST5 to value 0"]
impl crate::Resettable for TaskSt5Spec {}
