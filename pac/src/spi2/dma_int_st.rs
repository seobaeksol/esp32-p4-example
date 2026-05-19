#[doc = "Register `DMA_INT_ST` reader"]
pub type R = crate::R<DmaIntStSpec>;
#[doc = "Field `DMA_INFIFO_FULL_ERR` reader - The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DmaInfifoFullErrR = crate::BitReader;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR` reader - The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DmaOutfifoEmptyErrR = crate::BitReader;
#[doc = "Field `SLV_EX_QPI` reader - The status bit for SPI slave Ex_QPI interrupt."]
pub type SlvExQpiR = crate::BitReader;
#[doc = "Field `SLV_EN_QPI` reader - The status bit for SPI slave En_QPI interrupt."]
pub type SlvEnQpiR = crate::BitReader;
#[doc = "Field `SLV_CMD7` reader - The status bit for SPI slave CMD7 interrupt."]
pub type SlvCmd7R = crate::BitReader;
#[doc = "Field `SLV_CMD8` reader - The status bit for SPI slave CMD8 interrupt."]
pub type SlvCmd8R = crate::BitReader;
#[doc = "Field `SLV_CMD9` reader - The status bit for SPI slave CMD9 interrupt."]
pub type SlvCmd9R = crate::BitReader;
#[doc = "Field `SLV_CMDA` reader - The status bit for SPI slave CMDA interrupt."]
pub type SlvCmdaR = crate::BitReader;
#[doc = "Field `SLV_RD_DMA_DONE` reader - The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SlvRdDmaDoneR = crate::BitReader;
#[doc = "Field `SLV_WR_DMA_DONE` reader - The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SlvWrDmaDoneR = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE` reader - The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SlvRdBufDoneR = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE` reader - The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SlvWrBufDoneR = crate::BitReader;
#[doc = "Field `TRANS_DONE` reader - The status bit for SPI_TRANS_DONE_INT interrupt."]
pub type TransDoneR = crate::BitReader;
#[doc = "Field `DMA_SEG_TRANS_DONE` reader - The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DmaSegTransDoneR = crate::BitReader;
#[doc = "Field `SEG_MAGIC_ERR` reader - The status bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SegMagicErrR = crate::BitReader;
#[doc = "Field `SLV_BUF_ADDR_ERR` reader - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SlvBufAddrErrR = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR` reader - The status bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SlvCmdErrR = crate::BitReader;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR` reader - The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MstRxAfifoWfullErrR = crate::BitReader;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR` reader - The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MstTxAfifoRemptyErrR = crate::BitReader;
#[doc = "Field `APP2` reader - The status bit for SPI_APP2_INT interrupt."]
pub type App2R = crate::BitReader;
#[doc = "Field `APP1` reader - The status bit for SPI_APP1_INT interrupt."]
pub type App1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_err(&self) -> DmaInfifoFullErrR {
        DmaInfifoFullErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err(&self) -> DmaOutfifoEmptyErrR {
        DmaOutfifoEmptyErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn slv_ex_qpi(&self) -> SlvExQpiR {
        SlvExQpiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn slv_en_qpi(&self) -> SlvEnQpiR {
        SlvEnQpiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7(&self) -> SlvCmd7R {
        SlvCmd7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8(&self) -> SlvCmd8R {
        SlvCmd8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9(&self) -> SlvCmd9R {
        SlvCmd9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda(&self) -> SlvCmdaR {
        SlvCmdaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_dma_done(&self) -> SlvRdDmaDoneR {
        SlvRdDmaDoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_dma_done(&self) -> SlvWrDmaDoneR {
        SlvWrDmaDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SlvRdBufDoneR {
        SlvRdBufDoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SlvWrBufDoneR {
        SlvWrBufDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn trans_done(&self) -> TransDoneR {
        TransDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn dma_seg_trans_done(&self) -> DmaSegTransDoneR {
        DmaSegTransDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn seg_magic_err(&self) -> SegMagicErrR {
        SegMagicErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_buf_addr_err(&self) -> SlvBufAddrErrR {
        SlvBufAddrErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The status bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd_err(&self) -> SlvCmdErrR {
        SlvCmdErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err(&self) -> MstRxAfifoWfullErrR {
        MstRxAfifoWfullErrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err(&self) -> MstTxAfifoRemptyErrR {
        MstTxAfifoRemptyErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The status bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn app2(&self) -> App2R {
        App2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The status bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn app1(&self) -> App1R {
        App1R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "SPI interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntStSpec;
impl crate::RegisterSpec for DmaIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_st::R`](R) reader structure"]
impl crate::Readable for DmaIntStSpec {}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DmaIntStSpec {}
