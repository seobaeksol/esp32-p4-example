#[doc = "Register `TASK_ST5_CLR` writer"]
pub type W = crate::W<TaskSt5ClrSpec>;
#[doc = "Field `REGDMA_TASK_START0_ST_CLR` writer - Configures whether or not to clear REGDMA_task_start0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RegdmaTaskStart0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START1_ST_CLR` writer - Configures whether or not to clear REGDMA_task_start1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RegdmaTaskStart1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START2_ST_CLR` writer - Configures whether or not to clear REGDMA_task_start2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RegdmaTaskStart2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START3_ST_CLR` writer - Configures whether or not to clear REGDMA_task_start3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RegdmaTaskStart3StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_START_SAMPLE_ST_CLR` writer - Configures whether or not to clear TMPSNSR_task_start_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type TmpsnsrTaskStartSampleStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_STOP_SAMPLE_ST_CLR` writer - Configures whether or not to clear TMPSNSR_task_stop_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type TmpsnsrTaskStopSampleStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_RX_ST_CLR` writer - Configures whether or not to clear I2S0_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s0TaskStartRxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_TX_ST_CLR` writer - Configures whether or not to clear I2S0_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s0TaskStartTxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_RX_ST_CLR` writer - Configures whether or not to clear I2S0_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s0TaskStopRxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_TX_ST_CLR` writer - Configures whether or not to clear I2S0_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s0TaskStopTxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_START_RX_ST_CLR` writer - Configures whether or not to clear I2S1_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s1TaskStartRxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_START_TX_ST_CLR` writer - Configures whether or not to clear I2S1_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s1TaskStartTxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_STOP_RX_ST_CLR` writer - Configures whether or not to clear I2S1_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s1TaskStopRxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TASK_STOP_TX_ST_CLR` writer - Configures whether or not to clear I2S1_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s1TaskStopTxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_START_RX_ST_CLR` writer - Configures whether or not to clear I2S2_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s2TaskStartRxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_START_TX_ST_CLR` writer - Configures whether or not to clear I2S2_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s2TaskStartTxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_STOP_RX_ST_CLR` writer - Configures whether or not to clear I2S2_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s2TaskStopRxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_TASK_STOP_TX_ST_CLR` writer - Configures whether or not to clear I2S2_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2s2TaskStopTxStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_WAKEUP_CPU_ST_CLR` writer - Configures whether or not to clear ULP_task_wakeup_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type UlpTaskWakeupCpuStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_INT_CPU_ST_CLR` writer - Configures whether or not to clear ULP_task_int_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type UlpTaskIntCpuStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_START_ST_CLR` writer - Configures whether or not to clear RTC_task_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RtcTaskStartStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_STOP_ST_CLR` writer - Configures whether or not to clear RTC_task_stop trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RtcTaskStopStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_CLR_ST_CLR` writer - Configures whether or not to clear RTC_task_clr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RtcTaskClrStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_TRIGGERFLW_ST_CLR` writer - Configures whether or not to clear RTC_task_triggerflw trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RtcTaskTriggerflwStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbTaskInStartCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbTaskInStartCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_IN_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbTaskInStartCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbTaskOutStartCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbTaskOutStartCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAhbTaskOutStartCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiTaskInStartCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiTaskInStartCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear REGDMA_task_start0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn regdma_task_start0_st_clr(&mut self) -> RegdmaTaskStart0StClrW<'_, TaskSt5ClrSpec> {
        RegdmaTaskStart0StClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear REGDMA_task_start1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn regdma_task_start1_st_clr(&mut self) -> RegdmaTaskStart1StClrW<'_, TaskSt5ClrSpec> {
        RegdmaTaskStart1StClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear REGDMA_task_start2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn regdma_task_start2_st_clr(&mut self) -> RegdmaTaskStart2StClrW<'_, TaskSt5ClrSpec> {
        RegdmaTaskStart2StClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear REGDMA_task_start3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn regdma_task_start3_st_clr(&mut self) -> RegdmaTaskStart3StClrW<'_, TaskSt5ClrSpec> {
        RegdmaTaskStart3StClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear TMPSNSR_task_start_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn tmpsnsr_task_start_sample_st_clr(
        &mut self,
    ) -> TmpsnsrTaskStartSampleStClrW<'_, TaskSt5ClrSpec> {
        TmpsnsrTaskStartSampleStClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear TMPSNSR_task_stop_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn tmpsnsr_task_stop_sample_st_clr(
        &mut self,
    ) -> TmpsnsrTaskStopSampleStClrW<'_, TaskSt5ClrSpec> {
        TmpsnsrTaskStopSampleStClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear I2S0_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_start_rx_st_clr(&mut self) -> I2s0TaskStartRxStClrW<'_, TaskSt5ClrSpec> {
        I2s0TaskStartRxStClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear I2S0_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_start_tx_st_clr(&mut self) -> I2s0TaskStartTxStClrW<'_, TaskSt5ClrSpec> {
        I2s0TaskStartTxStClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear I2S0_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_stop_rx_st_clr(&mut self) -> I2s0TaskStopRxStClrW<'_, TaskSt5ClrSpec> {
        I2s0TaskStopRxStClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear I2S0_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_stop_tx_st_clr(&mut self) -> I2s0TaskStopTxStClrW<'_, TaskSt5ClrSpec> {
        I2s0TaskStopTxStClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear I2S1_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s1_task_start_rx_st_clr(&mut self) -> I2s1TaskStartRxStClrW<'_, TaskSt5ClrSpec> {
        I2s1TaskStartRxStClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear I2S1_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s1_task_start_tx_st_clr(&mut self) -> I2s1TaskStartTxStClrW<'_, TaskSt5ClrSpec> {
        I2s1TaskStartTxStClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear I2S1_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s1_task_stop_rx_st_clr(&mut self) -> I2s1TaskStopRxStClrW<'_, TaskSt5ClrSpec> {
        I2s1TaskStopRxStClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear I2S1_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s1_task_stop_tx_st_clr(&mut self) -> I2s1TaskStopTxStClrW<'_, TaskSt5ClrSpec> {
        I2s1TaskStopTxStClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear I2S2_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s2_task_start_rx_st_clr(&mut self) -> I2s2TaskStartRxStClrW<'_, TaskSt5ClrSpec> {
        I2s2TaskStartRxStClrW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear I2S2_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s2_task_start_tx_st_clr(&mut self) -> I2s2TaskStartTxStClrW<'_, TaskSt5ClrSpec> {
        I2s2TaskStartTxStClrW::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear I2S2_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s2_task_stop_rx_st_clr(&mut self) -> I2s2TaskStopRxStClrW<'_, TaskSt5ClrSpec> {
        I2s2TaskStopRxStClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear I2S2_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s2_task_stop_tx_st_clr(&mut self) -> I2s2TaskStopTxStClrW<'_, TaskSt5ClrSpec> {
        I2s2TaskStopTxStClrW::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to clear ULP_task_wakeup_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st_clr(&mut self) -> UlpTaskWakeupCpuStClrW<'_, TaskSt5ClrSpec> {
        UlpTaskWakeupCpuStClrW::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to clear ULP_task_int_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st_clr(&mut self) -> UlpTaskIntCpuStClrW<'_, TaskSt5ClrSpec> {
        UlpTaskIntCpuStClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to clear RTC_task_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_start_st_clr(&mut self) -> RtcTaskStartStClrW<'_, TaskSt5ClrSpec> {
        RtcTaskStartStClrW::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to clear RTC_task_stop trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_stop_st_clr(&mut self) -> RtcTaskStopStClrW<'_, TaskSt5ClrSpec> {
        RtcTaskStopStClrW::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to clear RTC_task_clr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_clr_st_clr(&mut self) -> RtcTaskClrStClrW<'_, TaskSt5ClrSpec> {
        RtcTaskClrStClrW::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to clear RTC_task_triggerflw trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st_clr(&mut self) -> RtcTaskTriggerflwStClrW<'_, TaskSt5ClrSpec> {
        RtcTaskTriggerflwStClrW::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to clear PDMA_AHB_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbTaskInStartCh0StClrW<'_, TaskSt5ClrSpec> {
        PdmaAhbTaskInStartCh0StClrW::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to clear PDMA_AHB_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbTaskInStartCh1StClrW<'_, TaskSt5ClrSpec> {
        PdmaAhbTaskInStartCh1StClrW::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to clear PDMA_AHB_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_task_in_start_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbTaskInStartCh2StClrW<'_, TaskSt5ClrSpec> {
        PdmaAhbTaskInStartCh2StClrW::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to clear PDMA_AHB_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch0_st_clr(
        &mut self,
    ) -> PdmaAhbTaskOutStartCh0StClrW<'_, TaskSt5ClrSpec> {
        PdmaAhbTaskOutStartCh0StClrW::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to clear PDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch1_st_clr(
        &mut self,
    ) -> PdmaAhbTaskOutStartCh1StClrW<'_, TaskSt5ClrSpec> {
        PdmaAhbTaskOutStartCh1StClrW::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to clear PDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_task_out_start_ch2_st_clr(
        &mut self,
    ) -> PdmaAhbTaskOutStartCh2StClrW<'_, TaskSt5ClrSpec> {
        PdmaAhbTaskOutStartCh2StClrW::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to clear PDMA_AXI_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiTaskInStartCh0StClrW<'_, TaskSt5ClrSpec> {
        PdmaAxiTaskInStartCh0StClrW::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to clear PDMA_AXI_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiTaskInStartCh1StClrW<'_, TaskSt5ClrSpec> {
        PdmaAxiTaskInStartCh1StClrW::new(self, 31)
    }
}
#[doc = "Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st5_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskSt5ClrSpec;
impl crate::RegisterSpec for TaskSt5ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`task_st5_clr::W`](W) writer structure"]
impl crate::Writable for TaskSt5ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST5_CLR to value 0"]
impl crate::Resettable for TaskSt5ClrSpec {}
