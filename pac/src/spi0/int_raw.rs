#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `SLV_ST_END` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SlvStEndR = crate::BitReader;
#[doc = "Field `SLV_ST_END` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SlvStEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_ST_END` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MstStEndR = crate::BitReader;
#[doc = "Field `MST_ST_END` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MstStEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR` reader - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type EccErrR = crate::BitReader;
#[doc = "Field `ECC_ERR` writer - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type EccErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_REJECT` reader - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type PmsRejectR = crate::BitReader;
#[doc = "Field `PMS_REJECT` writer - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type PmsRejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR` reader - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type AxiRaddrErrR = crate::BitReader;
#[doc = "Field `AXI_RADDR_ERR` writer - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type AxiRaddrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR` reader - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type AxiWrFlashErrR = crate::BitReader;
#[doc = "Field `AXI_WR_FLASH_ERR` writer - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type AxiWrFlashErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_WADDR_ERR` reader - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type AxiWaddrErrR = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR` writer - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type AxiWaddrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRANS_OVF` reader - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
pub type RxTransOvfR = crate::BitReader;
#[doc = "Field `RX_TRANS_OVF` writer - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
pub type RxTransOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_TRANS_UDF` reader - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
pub type TxTransUdfR = crate::BitReader;
#[doc = "Field `TX_TRANS_UDF` writer - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
pub type TxTransUdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS0_AFIFO_OVF` reader - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type Dqs0AfifoOvfR = crate::BitReader;
#[doc = "Field `DQS0_AFIFO_OVF` writer - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type Dqs0AfifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS1_AFIFO_OVF` reader - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type Dqs1AfifoOvfR = crate::BitReader;
#[doc = "Field `DQS1_AFIFO_OVF` writer - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type Dqs1AfifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_FIFO1_UDF` reader - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type BusFifo1UdfR = crate::BitReader;
#[doc = "Field `BUS_FIFO1_UDF` writer - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type BusFifo1UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_FIFO0_UDF` reader - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type BusFifo0UdfR = crate::BitReader;
#[doc = "Field `BUS_FIFO0_UDF` writer - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type BusFifo0UdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn slv_st_end(&self) -> SlvStEndR {
        SlvStEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mst_st_end(&self) -> MstStEndR {
        MstStEndR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn ecc_err(&self) -> EccErrR {
        EccErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    pub fn pms_reject(&self) -> PmsRejectR {
        PmsRejectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_raddr_err(&self) -> AxiRaddrErrR {
        AxiRaddrErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&self) -> AxiWrFlashErrR {
        AxiWrFlashErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_waddr_err(&self) -> AxiWaddrErrR {
        AxiWaddrErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 26 - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
    #[inline(always)]
    pub fn rx_trans_ovf(&self) -> RxTransOvfR {
        RxTransOvfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
    #[inline(always)]
    pub fn tx_trans_udf(&self) -> TxTransUdfR {
        TxTransUdfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&self) -> Dqs0AfifoOvfR {
        Dqs0AfifoOvfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&self) -> Dqs1AfifoOvfR {
        Dqs1AfifoOvfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    pub fn bus_fifo1_udf(&self) -> BusFifo1UdfR {
        BusFifo1UdfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
    #[inline(always)]
    pub fn bus_fifo0_udf(&self) -> BusFifo0UdfR {
        BusFifo0UdfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn slv_st_end(&mut self) -> SlvStEndW<'_, IntRawSpec> {
        SlvStEndW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mst_st_end(&mut self) -> MstStEndW<'_, IntRawSpec> {
        MstStEndW::new(self, 4)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> EccErrW<'_, IntRawSpec> {
        EccErrW::new(self, 5)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    pub fn pms_reject(&mut self) -> PmsRejectW<'_, IntRawSpec> {
        PmsRejectW::new(self, 6)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_raddr_err(&mut self) -> AxiRaddrErrW<'_, IntRawSpec> {
        AxiRaddrErrW::new(self, 7)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&mut self) -> AxiWrFlashErrW<'_, IntRawSpec> {
        AxiWrFlashErrW::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_waddr_err(&mut self) -> AxiWaddrErrW<'_, IntRawSpec> {
        AxiWaddrErrW::new(self, 9)
    }
    #[doc = "Bit 26 - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
    #[inline(always)]
    pub fn rx_trans_ovf(&mut self) -> RxTransOvfW<'_, IntRawSpec> {
        RxTransOvfW::new(self, 26)
    }
    #[doc = "Bit 27 - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
    #[inline(always)]
    pub fn tx_trans_udf(&mut self) -> TxTransUdfW<'_, IntRawSpec> {
        TxTransUdfW::new(self, 27)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&mut self) -> Dqs0AfifoOvfW<'_, IntRawSpec> {
        Dqs0AfifoOvfW::new(self, 28)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&mut self) -> Dqs1AfifoOvfW<'_, IntRawSpec> {
        Dqs1AfifoOvfW::new(self, 29)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    pub fn bus_fifo1_udf(&mut self) -> BusFifo1UdfW<'_, IntRawSpec> {
        BusFifo1UdfW::new(self, 30)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
    #[inline(always)]
    pub fn bus_fifo0_udf(&mut self) -> BusFifo0UdfW<'_, IntRawSpec> {
        BusFifo0UdfW::new(self, 31)
    }
}
#[doc = "SPI0 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
