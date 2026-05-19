#[doc = "Register `TASK_ST6` reader"]
pub type R = crate::R<TaskSt6Spec>;
#[doc = "Register `TASK_ST6` writer"]
pub type W = crate::W<TaskSt6Spec>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH2_ST` reader - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskInStartCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH2_ST` writer - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskInStartCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH0_ST` reader - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskOutStartCh0StR = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH0_ST` writer - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskOutStartCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH1_ST` reader - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskOutStartCh1StR = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH1_ST` writer - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskOutStartCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH2_ST` reader - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskOutStartCh2StR = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH2_ST` writer - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PdmaAxiTaskOutStartCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` reader - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PmuTaskSleepReqStR = crate::BitReader;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` writer - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PmuTaskSleepReqStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH0_ST` reader - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInStartCh0StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_START_CH0_ST` writer - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInStartCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH1_ST` reader - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInStartCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_START_CH1_ST` writer - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInStartCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH0_ST` reader - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInDscrReadyCh0StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH0_ST` writer - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInDscrReadyCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH1_ST` reader - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInDscrReadyCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH1_ST` writer - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskInDscrReadyCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH0_ST` reader - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutStartCh0StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_START_CH0_ST` writer - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutStartCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH1_ST` reader - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutStartCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_START_CH1_ST` writer - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutStartCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH2_ST` reader - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutStartCh2StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_START_CH2_ST` writer - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutStartCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH0_ST` reader - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutDscrReadyCh0StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH0_ST` writer - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutDscrReadyCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH1_ST` reader - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutDscrReadyCh1StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH1_ST` writer - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutDscrReadyCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH2_ST` reader - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutDscrReadyCh2StR = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH2_ST` writer - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Dma2dTaskOutDscrReadyCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch2_st(&self) -> PdmaAxiTaskInStartCh2StR {
        PdmaAxiTaskInStartCh2StR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch0_st(&self) -> PdmaAxiTaskOutStartCh0StR {
        PdmaAxiTaskOutStartCh0StR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch1_st(&self) -> PdmaAxiTaskOutStartCh1StR {
        PdmaAxiTaskOutStartCh1StR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch2_st(&self) -> PdmaAxiTaskOutStartCh2StR {
        PdmaAxiTaskOutStartCh2StR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&self) -> PmuTaskSleepReqStR {
        PmuTaskSleepReqStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch0_st(&self) -> Dma2dTaskInStartCh0StR {
        Dma2dTaskInStartCh0StR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch1_st(&self) -> Dma2dTaskInStartCh1StR {
        Dma2dTaskInStartCh1StR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch0_st(&self) -> Dma2dTaskInDscrReadyCh0StR {
        Dma2dTaskInDscrReadyCh0StR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch1_st(&self) -> Dma2dTaskInDscrReadyCh1StR {
        Dma2dTaskInDscrReadyCh1StR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch0_st(&self) -> Dma2dTaskOutStartCh0StR {
        Dma2dTaskOutStartCh0StR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch1_st(&self) -> Dma2dTaskOutStartCh1StR {
        Dma2dTaskOutStartCh1StR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch2_st(&self) -> Dma2dTaskOutStartCh2StR {
        Dma2dTaskOutStartCh2StR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch0_st(&self) -> Dma2dTaskOutDscrReadyCh0StR {
        Dma2dTaskOutDscrReadyCh0StR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch1_st(&self) -> Dma2dTaskOutDscrReadyCh1StR {
        Dma2dTaskOutDscrReadyCh1StR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch2_st(&self) -> Dma2dTaskOutDscrReadyCh2StR {
        Dma2dTaskOutDscrReadyCh2StR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch2_st(&mut self) -> PdmaAxiTaskInStartCh2StW<'_, TaskSt6Spec> {
        PdmaAxiTaskInStartCh2StW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch0_st(&mut self) -> PdmaAxiTaskOutStartCh0StW<'_, TaskSt6Spec> {
        PdmaAxiTaskOutStartCh0StW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch1_st(&mut self) -> PdmaAxiTaskOutStartCh1StW<'_, TaskSt6Spec> {
        PdmaAxiTaskOutStartCh1StW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch2_st(&mut self) -> PdmaAxiTaskOutStartCh2StW<'_, TaskSt6Spec> {
        PdmaAxiTaskOutStartCh2StW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&mut self) -> PmuTaskSleepReqStW<'_, TaskSt6Spec> {
        PmuTaskSleepReqStW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch0_st(&mut self) -> Dma2dTaskInStartCh0StW<'_, TaskSt6Spec> {
        Dma2dTaskInStartCh0StW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch1_st(&mut self) -> Dma2dTaskInStartCh1StW<'_, TaskSt6Spec> {
        Dma2dTaskInStartCh1StW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch0_st(
        &mut self,
    ) -> Dma2dTaskInDscrReadyCh0StW<'_, TaskSt6Spec> {
        Dma2dTaskInDscrReadyCh0StW::new(self, 7)
    }
    #[doc = "Bit 8 - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch1_st(
        &mut self,
    ) -> Dma2dTaskInDscrReadyCh1StW<'_, TaskSt6Spec> {
        Dma2dTaskInDscrReadyCh1StW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch0_st(&mut self) -> Dma2dTaskOutStartCh0StW<'_, TaskSt6Spec> {
        Dma2dTaskOutStartCh0StW::new(self, 9)
    }
    #[doc = "Bit 10 - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch1_st(&mut self) -> Dma2dTaskOutStartCh1StW<'_, TaskSt6Spec> {
        Dma2dTaskOutStartCh1StW::new(self, 10)
    }
    #[doc = "Bit 11 - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch2_st(&mut self) -> Dma2dTaskOutStartCh2StW<'_, TaskSt6Spec> {
        Dma2dTaskOutStartCh2StW::new(self, 11)
    }
    #[doc = "Bit 12 - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch0_st(
        &mut self,
    ) -> Dma2dTaskOutDscrReadyCh0StW<'_, TaskSt6Spec> {
        Dma2dTaskOutDscrReadyCh0StW::new(self, 12)
    }
    #[doc = "Bit 13 - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch1_st(
        &mut self,
    ) -> Dma2dTaskOutDscrReadyCh1StW<'_, TaskSt6Spec> {
        Dma2dTaskOutDscrReadyCh1StW::new(self, 13)
    }
    #[doc = "Bit 14 - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch2_st(
        &mut self,
    ) -> Dma2dTaskOutDscrReadyCh2StW<'_, TaskSt6Spec> {
        Dma2dTaskOutDscrReadyCh2StW::new(self, 14)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskSt6Spec;
impl crate::RegisterSpec for TaskSt6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st6::R`](R) reader structure"]
impl crate::Readable for TaskSt6Spec {}
#[doc = "`write(|w| ..)` method takes [`task_st6::W`](W) writer structure"]
impl crate::Writable for TaskSt6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST6 to value 0"]
impl crate::Resettable for TaskSt6Spec {}
