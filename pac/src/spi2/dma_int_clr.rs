#[doc = "Register `DMA_INT_CLR` writer"]
pub type W = crate::W<DmaIntClrSpec>;
#[doc = "Field `DMA_INFIFO_FULL_ERR` writer - The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DmaInfifoFullErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR` writer - The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DmaOutfifoEmptyErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_EX_QPI` writer - The clear bit for SPI slave Ex_QPI interrupt."]
pub type SlvExQpiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_EN_QPI` writer - The clear bit for SPI slave En_QPI interrupt."]
pub type SlvEnQpiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_CMD7` writer - The clear bit for SPI slave CMD7 interrupt."]
pub type SlvCmd7W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_CMD8` writer - The clear bit for SPI slave CMD8 interrupt."]
pub type SlvCmd8W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_CMD9` writer - The clear bit for SPI slave CMD9 interrupt."]
pub type SlvCmd9W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_CMDA` writer - The clear bit for SPI slave CMDA interrupt."]
pub type SlvCmdaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_RD_DMA_DONE` writer - The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SlvRdDmaDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_WR_DMA_DONE` writer - The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SlvWrDmaDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_RD_BUF_DONE` writer - The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SlvRdBufDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE` writer - The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SlvWrBufDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANS_DONE` writer - The clear bit for SPI_TRANS_DONE_INT interrupt."]
pub type TransDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA_SEG_TRANS_DONE` writer - The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DmaSegTransDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SEG_MAGIC_ERR` writer - The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SegMagicErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_BUF_ADDR_ERR` writer - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SlvBufAddrErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_CMD_ERR` writer - The clear bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SlvCmdErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR` writer - The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MstRxAfifoWfullErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR` writer - The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MstTxAfifoRemptyErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APP2` writer - The clear bit for SPI_APP2_INT interrupt."]
pub type App2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APP1` writer - The clear bit for SPI_APP1_INT interrupt."]
pub type App1W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_err(&mut self) -> DmaInfifoFullErrW<'_, DmaIntClrSpec> {
        DmaInfifoFullErrW::new(self, 0)
    }
    #[doc = "Bit 1 - The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err(&mut self) -> DmaOutfifoEmptyErrW<'_, DmaIntClrSpec> {
        DmaOutfifoEmptyErrW::new(self, 1)
    }
    #[doc = "Bit 2 - The clear bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn slv_ex_qpi(&mut self) -> SlvExQpiW<'_, DmaIntClrSpec> {
        SlvExQpiW::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn slv_en_qpi(&mut self) -> SlvEnQpiW<'_, DmaIntClrSpec> {
        SlvEnQpiW::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7(&mut self) -> SlvCmd7W<'_, DmaIntClrSpec> {
        SlvCmd7W::new(self, 4)
    }
    #[doc = "Bit 5 - The clear bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8(&mut self) -> SlvCmd8W<'_, DmaIntClrSpec> {
        SlvCmd8W::new(self, 5)
    }
    #[doc = "Bit 6 - The clear bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9(&mut self) -> SlvCmd9W<'_, DmaIntClrSpec> {
        SlvCmd9W::new(self, 6)
    }
    #[doc = "Bit 7 - The clear bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda(&mut self) -> SlvCmdaW<'_, DmaIntClrSpec> {
        SlvCmdaW::new(self, 7)
    }
    #[doc = "Bit 8 - The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_dma_done(&mut self) -> SlvRdDmaDoneW<'_, DmaIntClrSpec> {
        SlvRdDmaDoneW::new(self, 8)
    }
    #[doc = "Bit 9 - The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_dma_done(&mut self) -> SlvWrDmaDoneW<'_, DmaIntClrSpec> {
        SlvWrDmaDoneW::new(self, 9)
    }
    #[doc = "Bit 10 - The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&mut self) -> SlvRdBufDoneW<'_, DmaIntClrSpec> {
        SlvRdBufDoneW::new(self, 10)
    }
    #[doc = "Bit 11 - The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&mut self) -> SlvWrBufDoneW<'_, DmaIntClrSpec> {
        SlvWrBufDoneW::new(self, 11)
    }
    #[doc = "Bit 12 - The clear bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn trans_done(&mut self) -> TransDoneW<'_, DmaIntClrSpec> {
        TransDoneW::new(self, 12)
    }
    #[doc = "Bit 13 - The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn dma_seg_trans_done(&mut self) -> DmaSegTransDoneW<'_, DmaIntClrSpec> {
        DmaSegTransDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn seg_magic_err(&mut self) -> SegMagicErrW<'_, DmaIntClrSpec> {
        SegMagicErrW::new(self, 14)
    }
    #[doc = "Bit 15 - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_buf_addr_err(&mut self) -> SlvBufAddrErrW<'_, DmaIntClrSpec> {
        SlvBufAddrErrW::new(self, 15)
    }
    #[doc = "Bit 16 - The clear bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd_err(&mut self) -> SlvCmdErrW<'_, DmaIntClrSpec> {
        SlvCmdErrW::new(self, 16)
    }
    #[doc = "Bit 17 - The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err(&mut self) -> MstRxAfifoWfullErrW<'_, DmaIntClrSpec> {
        MstRxAfifoWfullErrW::new(self, 17)
    }
    #[doc = "Bit 18 - The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err(&mut self) -> MstTxAfifoRemptyErrW<'_, DmaIntClrSpec> {
        MstTxAfifoRemptyErrW::new(self, 18)
    }
    #[doc = "Bit 19 - The clear bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn app2(&mut self) -> App2W<'_, DmaIntClrSpec> {
        App2W::new(self, 19)
    }
    #[doc = "Bit 20 - The clear bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn app1(&mut self) -> App1W<'_, DmaIntClrSpec> {
        App1W::new(self, 20)
    }
}
#[doc = "SPI interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntClrSpec;
impl crate::RegisterSpec for DmaIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int_clr::W`](W) writer structure"]
impl crate::Writable for DmaIntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x001f_ffff;
}
#[doc = "`reset()` method sets DMA_INT_CLR to value 0"]
impl crate::Resettable for DmaIntClrSpec {}
