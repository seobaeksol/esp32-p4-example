#[doc = "Register `DMA_INT_RAW` reader"]
pub type R = crate::R<DmaIntRawSpec>;
#[doc = "Register `DMA_INT_RAW` writer"]
pub type W = crate::W<DmaIntRawSpec>;
#[doc = "Field `DMA_INFIFO_FULL_ERR` reader - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
pub type DmaInfifoFullErrR = crate::BitReader;
#[doc = "Field `DMA_INFIFO_FULL_ERR` writer - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
pub type DmaInfifoFullErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR` reader - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
pub type DmaOutfifoEmptyErrR = crate::BitReader;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR` writer - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
pub type DmaOutfifoEmptyErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EX_QPI` reader - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
pub type SlvExQpiR = crate::BitReader;
#[doc = "Field `SLV_EX_QPI` writer - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
pub type SlvExQpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EN_QPI` reader - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
pub type SlvEnQpiR = crate::BitReader;
#[doc = "Field `SLV_EN_QPI` writer - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
pub type SlvEnQpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD7` reader - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
pub type SlvCmd7R = crate::BitReader;
#[doc = "Field `SLV_CMD7` writer - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
pub type SlvCmd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD8` reader - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
pub type SlvCmd8R = crate::BitReader;
#[doc = "Field `SLV_CMD8` writer - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
pub type SlvCmd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD9` reader - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
pub type SlvCmd9R = crate::BitReader;
#[doc = "Field `SLV_CMD9` writer - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
pub type SlvCmd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMDA` reader - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
pub type SlvCmdaR = crate::BitReader;
#[doc = "Field `SLV_CMDA` writer - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
pub type SlvCmdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_DMA_DONE` reader - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
pub type SlvRdDmaDoneR = crate::BitReader;
#[doc = "Field `SLV_RD_DMA_DONE` writer - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
pub type SlvRdDmaDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_DMA_DONE` reader - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
pub type SlvWrDmaDoneR = crate::BitReader;
#[doc = "Field `SLV_WR_DMA_DONE` writer - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
pub type SlvWrDmaDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_BUF_DONE` reader - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
pub type SlvRdBufDoneR = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE` writer - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
pub type SlvRdBufDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE` reader - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
pub type SlvWrBufDoneR = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE` writer - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
pub type SlvWrBufDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE` reader - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
pub type TransDoneR = crate::BitReader;
#[doc = "Field `TRANS_DONE` writer - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
pub type TransDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SEG_TRANS_DONE` reader - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
pub type DmaSegTransDoneR = crate::BitReader;
#[doc = "Field `DMA_SEG_TRANS_DONE` writer - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
pub type DmaSegTransDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEG_MAGIC_ERR` reader - The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others."]
pub type SegMagicErrR = crate::BitReader;
#[doc = "Field `SEG_MAGIC_ERR` writer - The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others."]
pub type SegMagicErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_BUF_ADDR_ERR` reader - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
pub type SlvBufAddrErrR = crate::BitReader;
#[doc = "Field `SLV_BUF_ADDR_ERR` writer - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
pub type SlvBufAddrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD_ERR` reader - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
pub type SlvCmdErrR = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR` writer - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
pub type SlvCmdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR` reader - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
pub type MstRxAfifoWfullErrR = crate::BitReader;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR` writer - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
pub type MstRxAfifoWfullErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR` reader - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
pub type MstTxAfifoRemptyErrR = crate::BitReader;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR` writer - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
pub type MstTxAfifoRemptyErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP2` reader - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
pub type App2R = crate::BitReader;
#[doc = "Field `APP2` writer - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
pub type App2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP1` reader - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
pub type App1R = crate::BitReader;
#[doc = "Field `APP1` writer - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
pub type App1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
    #[inline(always)]
    pub fn dma_infifo_full_err(&self) -> DmaInfifoFullErrR {
        DmaInfifoFullErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err(&self) -> DmaOutfifoEmptyErrR {
        DmaOutfifoEmptyErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_ex_qpi(&self) -> SlvExQpiR {
        SlvExQpiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_en_qpi(&self) -> SlvEnQpiR {
        SlvEnQpiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd7(&self) -> SlvCmd7R {
        SlvCmd7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd8(&self) -> SlvCmd8R {
        SlvCmd8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd9(&self) -> SlvCmd9R {
        SlvCmd9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmda(&self) -> SlvCmdaR {
        SlvCmdaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_rd_dma_done(&self) -> SlvRdDmaDoneR {
        SlvRdDmaDoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_wr_dma_done(&self) -> SlvWrDmaDoneR {
        SlvWrDmaDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SlvRdBufDoneR {
        SlvRdBufDoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SlvWrBufDoneR {
        SlvWrBufDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
    #[inline(always)]
    pub fn trans_done(&self) -> TransDoneR {
        TransDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
    #[inline(always)]
    pub fn dma_seg_trans_done(&self) -> DmaSegTransDoneR {
        DmaSegTransDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others."]
    #[inline(always)]
    pub fn seg_magic_err(&self) -> SegMagicErrR {
        SegMagicErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
    #[inline(always)]
    pub fn slv_buf_addr_err(&self) -> SlvBufAddrErrR {
        SlvBufAddrErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd_err(&self) -> SlvCmdErrR {
        SlvCmdErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err(&self) -> MstRxAfifoWfullErrR {
        MstRxAfifoWfullErrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err(&self) -> MstTxAfifoRemptyErrR {
        MstTxAfifoRemptyErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    pub fn app2(&self) -> App2R {
        App2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    pub fn app1(&self) -> App1R {
        App1R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
    #[inline(always)]
    pub fn dma_infifo_full_err(&mut self) -> DmaInfifoFullErrW<'_, DmaIntRawSpec> {
        DmaInfifoFullErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err(&mut self) -> DmaOutfifoEmptyErrW<'_, DmaIntRawSpec> {
        DmaOutfifoEmptyErrW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_ex_qpi(&mut self) -> SlvExQpiW<'_, DmaIntRawSpec> {
        SlvExQpiW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_en_qpi(&mut self) -> SlvEnQpiW<'_, DmaIntRawSpec> {
        SlvEnQpiW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd7(&mut self) -> SlvCmd7W<'_, DmaIntRawSpec> {
        SlvCmd7W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd8(&mut self) -> SlvCmd8W<'_, DmaIntRawSpec> {
        SlvCmd8W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd9(&mut self) -> SlvCmd9W<'_, DmaIntRawSpec> {
        SlvCmd9W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_cmda(&mut self) -> SlvCmdaW<'_, DmaIntRawSpec> {
        SlvCmdaW::new(self, 7)
    }
    #[doc = "Bit 8 - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_rd_dma_done(&mut self) -> SlvRdDmaDoneW<'_, DmaIntRawSpec> {
        SlvRdDmaDoneW::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_wr_dma_done(&mut self) -> SlvWrDmaDoneW<'_, DmaIntRawSpec> {
        SlvWrDmaDoneW::new(self, 9)
    }
    #[doc = "Bit 10 - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&mut self) -> SlvRdBufDoneW<'_, DmaIntRawSpec> {
        SlvRdBufDoneW::new(self, 10)
    }
    #[doc = "Bit 11 - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&mut self) -> SlvWrBufDoneW<'_, DmaIntRawSpec> {
        SlvWrBufDoneW::new(self, 11)
    }
    #[doc = "Bit 12 - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
    #[inline(always)]
    pub fn trans_done(&mut self) -> TransDoneW<'_, DmaIntRawSpec> {
        TransDoneW::new(self, 12)
    }
    #[doc = "Bit 13 - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
    #[inline(always)]
    pub fn dma_seg_trans_done(&mut self) -> DmaSegTransDoneW<'_, DmaIntRawSpec> {
        DmaSegTransDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others."]
    #[inline(always)]
    pub fn seg_magic_err(&mut self) -> SegMagicErrW<'_, DmaIntRawSpec> {
        SegMagicErrW::new(self, 14)
    }
    #[doc = "Bit 15 - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
    #[inline(always)]
    pub fn slv_buf_addr_err(&mut self) -> SlvBufAddrErrW<'_, DmaIntRawSpec> {
        SlvBufAddrErrW::new(self, 15)
    }
    #[doc = "Bit 16 - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
    #[inline(always)]
    pub fn slv_cmd_err(&mut self) -> SlvCmdErrW<'_, DmaIntRawSpec> {
        SlvCmdErrW::new(self, 16)
    }
    #[doc = "Bit 17 - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err(&mut self) -> MstRxAfifoWfullErrW<'_, DmaIntRawSpec> {
        MstRxAfifoWfullErrW::new(self, 17)
    }
    #[doc = "Bit 18 - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err(&mut self) -> MstTxAfifoRemptyErrW<'_, DmaIntRawSpec> {
        MstTxAfifoRemptyErrW::new(self, 18)
    }
    #[doc = "Bit 19 - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    pub fn app2(&mut self) -> App2W<'_, DmaIntRawSpec> {
        App2W::new(self, 19)
    }
    #[doc = "Bit 20 - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    pub fn app1(&mut self) -> App1W<'_, DmaIntRawSpec> {
        App1W::new(self, 20)
    }
}
#[doc = "SPI interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntRawSpec;
impl crate::RegisterSpec for DmaIntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_raw::R`](R) reader structure"]
impl crate::Readable for DmaIntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_raw::W`](W) writer structure"]
impl crate::Writable for DmaIntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_RAW to value 0"]
impl crate::Resettable for DmaIntRawSpec {}
