#[doc = "Register `DMA_INT_SET` writer"]
pub type W = crate::W<DmaIntSetSpec>;
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_SET` writer - The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DmaInfifoFullErrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_SET` writer - The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DmaOutfifoEmptyErrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EX_QPI_INT_SET` writer - The software set bit for SPI slave Ex_QPI interrupt."]
pub type SlvExQpiIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EN_QPI_INT_SET` writer - The software set bit for SPI slave En_QPI interrupt."]
pub type SlvEnQpiIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD7_INT_SET` writer - The software set bit for SPI slave CMD7 interrupt."]
pub type SlvCmd7IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD8_INT_SET` writer - The software set bit for SPI slave CMD8 interrupt."]
pub type SlvCmd8IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD9_INT_SET` writer - The software set bit for SPI slave CMD9 interrupt."]
pub type SlvCmd9IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMDA_INT_SET` writer - The software set bit for SPI slave CMDA interrupt."]
pub type SlvCmdaIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_DMA_DONE_INT_SET` writer - The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SlvRdDmaDoneIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_DMA_DONE_INT_SET` writer - The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SlvWrDmaDoneIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_SET` writer - The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SlvRdBufDoneIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE_INT_SET` writer - The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SlvWrBufDoneIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE_INT_SET` writer - The software set bit for SPI_TRANS_DONE_INT interrupt."]
pub type TransDoneIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_SET` writer - The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DmaSegTransDoneIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEG_MAGIC_ERR_INT_SET` writer - The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SegMagicErrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_SET` writer - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SlvBufAddrErrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD_ERR_INT_SET` writer - The software set bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SlvCmdErrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_SET` writer - The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MstRxAfifoWfullErrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_SET` writer - The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MstTxAfifoRemptyErrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP2_INT_SET` writer - The software set bit for SPI_APP2_INT interrupt."]
pub type App2IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP1_INT_SET` writer - The software set bit for SPI_APP1_INT interrupt."]
pub type App1IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_set(&mut self) -> DmaInfifoFullErrIntSetW<'_, DmaIntSetSpec> {
        DmaInfifoFullErrIntSetW::new(self, 0)
    }
    #[doc = "Bit 1 - The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_set(
        &mut self,
    ) -> DmaOutfifoEmptyErrIntSetW<'_, DmaIntSetSpec> {
        DmaOutfifoEmptyErrIntSetW::new(self, 1)
    }
    #[doc = "Bit 2 - The software set bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn slv_ex_qpi_int_set(&mut self) -> SlvExQpiIntSetW<'_, DmaIntSetSpec> {
        SlvExQpiIntSetW::new(self, 2)
    }
    #[doc = "Bit 3 - The software set bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn slv_en_qpi_int_set(&mut self) -> SlvEnQpiIntSetW<'_, DmaIntSetSpec> {
        SlvEnQpiIntSetW::new(self, 3)
    }
    #[doc = "Bit 4 - The software set bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_set(&mut self) -> SlvCmd7IntSetW<'_, DmaIntSetSpec> {
        SlvCmd7IntSetW::new(self, 4)
    }
    #[doc = "Bit 5 - The software set bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_set(&mut self) -> SlvCmd8IntSetW<'_, DmaIntSetSpec> {
        SlvCmd8IntSetW::new(self, 5)
    }
    #[doc = "Bit 6 - The software set bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_set(&mut self) -> SlvCmd9IntSetW<'_, DmaIntSetSpec> {
        SlvCmd9IntSetW::new(self, 6)
    }
    #[doc = "Bit 7 - The software set bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_set(&mut self) -> SlvCmdaIntSetW<'_, DmaIntSetSpec> {
        SlvCmdaIntSetW::new(self, 7)
    }
    #[doc = "Bit 8 - The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_set(&mut self) -> SlvRdDmaDoneIntSetW<'_, DmaIntSetSpec> {
        SlvRdDmaDoneIntSetW::new(self, 8)
    }
    #[doc = "Bit 9 - The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_set(&mut self) -> SlvWrDmaDoneIntSetW<'_, DmaIntSetSpec> {
        SlvWrDmaDoneIntSetW::new(self, 9)
    }
    #[doc = "Bit 10 - The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_set(&mut self) -> SlvRdBufDoneIntSetW<'_, DmaIntSetSpec> {
        SlvRdBufDoneIntSetW::new(self, 10)
    }
    #[doc = "Bit 11 - The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_set(&mut self) -> SlvWrBufDoneIntSetW<'_, DmaIntSetSpec> {
        SlvWrBufDoneIntSetW::new(self, 11)
    }
    #[doc = "Bit 12 - The software set bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn trans_done_int_set(&mut self) -> TransDoneIntSetW<'_, DmaIntSetSpec> {
        TransDoneIntSetW::new(self, 12)
    }
    #[doc = "Bit 13 - The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_set(&mut self) -> DmaSegTransDoneIntSetW<'_, DmaIntSetSpec> {
        DmaSegTransDoneIntSetW::new(self, 13)
    }
    #[doc = "Bit 14 - The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn seg_magic_err_int_set(&mut self) -> SegMagicErrIntSetW<'_, DmaIntSetSpec> {
        SegMagicErrIntSetW::new(self, 14)
    }
    #[doc = "Bit 15 - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_set(&mut self) -> SlvBufAddrErrIntSetW<'_, DmaIntSetSpec> {
        SlvBufAddrErrIntSetW::new(self, 15)
    }
    #[doc = "Bit 16 - The software set bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd_err_int_set(&mut self) -> SlvCmdErrIntSetW<'_, DmaIntSetSpec> {
        SlvCmdErrIntSetW::new(self, 16)
    }
    #[doc = "Bit 17 - The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_set(
        &mut self,
    ) -> MstRxAfifoWfullErrIntSetW<'_, DmaIntSetSpec> {
        MstRxAfifoWfullErrIntSetW::new(self, 17)
    }
    #[doc = "Bit 18 - The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_set(
        &mut self,
    ) -> MstTxAfifoRemptyErrIntSetW<'_, DmaIntSetSpec> {
        MstTxAfifoRemptyErrIntSetW::new(self, 18)
    }
    #[doc = "Bit 19 - The software set bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn app2_int_set(&mut self) -> App2IntSetW<'_, DmaIntSetSpec> {
        App2IntSetW::new(self, 19)
    }
    #[doc = "Bit 20 - The software set bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn app1_int_set(&mut self) -> App1IntSetW<'_, DmaIntSetSpec> {
        App1IntSetW::new(self, 20)
    }
}
#[doc = "SPI interrupt software set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntSetSpec;
impl crate::RegisterSpec for DmaIntSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int_set::W`](W) writer structure"]
impl crate::Writable for DmaIntSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_SET to value 0"]
impl crate::Resettable for DmaIntSetSpec {}
