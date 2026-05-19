#[doc = "Register `TASK_ST6_CLR` writer"]
pub type W = crate::W<TaskSt6ClrSpec>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiTaskInStartCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiTaskOutStartCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiTaskOutStartCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PdmaAxiTaskOutStartCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST_CLR` writer - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PmuTaskSleepReqStClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskInStartCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskInStartCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskInDscrReadyCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskInDscrReadyCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskOutStartCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskOutStartCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskOutStartCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskOutDscrReadyCh0StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskOutDscrReadyCh1StClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type Dma2dTaskOutDscrReadyCh2StClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiTaskInStartCh2StClrW<'_, TaskSt6ClrSpec> {
        PdmaAxiTaskInStartCh2StClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch0_st_clr(
        &mut self,
    ) -> PdmaAxiTaskOutStartCh0StClrW<'_, TaskSt6ClrSpec> {
        PdmaAxiTaskOutStartCh0StClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch1_st_clr(
        &mut self,
    ) -> PdmaAxiTaskOutStartCh1StClrW<'_, TaskSt6ClrSpec> {
        PdmaAxiTaskOutStartCh1StClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch2_st_clr(
        &mut self,
    ) -> PdmaAxiTaskOutStartCh2StClrW<'_, TaskSt6ClrSpec> {
        PdmaAxiTaskOutStartCh2StClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st_clr(&mut self) -> PmuTaskSleepReqStClrW<'_, TaskSt6ClrSpec> {
        PmuTaskSleepReqStClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear DMA2D_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch0_st_clr(
        &mut self,
    ) -> Dma2dTaskInStartCh0StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskInStartCh0StClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear DMA2D_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch1_st_clr(
        &mut self,
    ) -> Dma2dTaskInStartCh1StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskInStartCh1StClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch0_st_clr(
        &mut self,
    ) -> Dma2dTaskInDscrReadyCh0StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskInDscrReadyCh0StClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch1_st_clr(
        &mut self,
    ) -> Dma2dTaskInDscrReadyCh1StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskInDscrReadyCh1StClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear DMA2D_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch0_st_clr(
        &mut self,
    ) -> Dma2dTaskOutStartCh0StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskOutStartCh0StClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear DMA2D_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch1_st_clr(
        &mut self,
    ) -> Dma2dTaskOutStartCh1StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskOutStartCh1StClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear DMA2D_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch2_st_clr(
        &mut self,
    ) -> Dma2dTaskOutStartCh2StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskOutStartCh2StClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch0_st_clr(
        &mut self,
    ) -> Dma2dTaskOutDscrReadyCh0StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskOutDscrReadyCh0StClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch1_st_clr(
        &mut self,
    ) -> Dma2dTaskOutDscrReadyCh1StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskOutDscrReadyCh1StClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch2_st_clr(
        &mut self,
    ) -> Dma2dTaskOutDscrReadyCh2StClrW<'_, TaskSt6ClrSpec> {
        Dma2dTaskOutDscrReadyCh2StClrW::new(self, 14)
    }
}
#[doc = "Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st6_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskSt6ClrSpec;
impl crate::RegisterSpec for TaskSt6ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`task_st6_clr::W`](W) writer structure"]
impl crate::Writable for TaskSt6ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST6_CLR to value 0"]
impl crate::Resettable for TaskSt6ClrSpec {}
