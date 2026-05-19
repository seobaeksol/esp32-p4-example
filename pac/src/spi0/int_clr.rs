#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `SLV_ST_END` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SlvStEndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_ST_END` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MstStEndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ECC_ERR` writer - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type EccErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PMS_REJECT` writer - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PmsRejectW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR` writer - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AxiRaddrErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR` writer - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AxiWrFlashErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_WADDR_ERR` writer - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AxiWaddrErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_TRANS_OVF` writer - The clear bit for SPI_MEM_RX_TRANS_OVF_INT interrupt."]
pub type RxTransOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_TRANS_UDF` writer - The clear bit for SPI_MEM_TX_TRANS_UDF_INT interrupt."]
pub type TxTransUdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DQS0_AFIFO_OVF` writer - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
pub type Dqs0AfifoOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DQS1_AFIFO_OVF` writer - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
pub type Dqs1AfifoOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUS_FIFO1_UDF` writer - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
pub type BusFifo1UdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUS_FIFO0_UDF` writer - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
pub type BusFifo0UdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end(&mut self) -> SlvStEndW<'_, IntClrSpec> {
        SlvStEndW::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end(&mut self) -> MstStEndW<'_, IntClrSpec> {
        MstStEndW::new(self, 4)
    }
    #[doc = "Bit 5 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> EccErrW<'_, IntClrSpec> {
        EccErrW::new(self, 5)
    }
    #[doc = "Bit 6 - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn pms_reject(&mut self) -> PmsRejectW<'_, IntClrSpec> {
        PmsRejectW::new(self, 6)
    }
    #[doc = "Bit 7 - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_raddr_err(&mut self) -> AxiRaddrErrW<'_, IntClrSpec> {
        AxiRaddrErrW::new(self, 7)
    }
    #[doc = "Bit 8 - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&mut self) -> AxiWrFlashErrW<'_, IntClrSpec> {
        AxiWrFlashErrW::new(self, 8)
    }
    #[doc = "Bit 9 - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err(&mut self) -> AxiWaddrErrW<'_, IntClrSpec> {
        AxiWaddrErrW::new(self, 9)
    }
    #[doc = "Bit 26 - The clear bit for SPI_MEM_RX_TRANS_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rx_trans_ovf(&mut self) -> RxTransOvfW<'_, IntClrSpec> {
        RxTransOvfW::new(self, 26)
    }
    #[doc = "Bit 27 - The clear bit for SPI_MEM_TX_TRANS_UDF_INT interrupt."]
    #[inline(always)]
    pub fn tx_trans_udf(&mut self) -> TxTransUdfW<'_, IntClrSpec> {
        TxTransUdfW::new(self, 27)
    }
    #[doc = "Bit 28 - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&mut self) -> Dqs0AfifoOvfW<'_, IntClrSpec> {
        Dqs0AfifoOvfW::new(self, 28)
    }
    #[doc = "Bit 29 - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&mut self) -> Dqs1AfifoOvfW<'_, IntClrSpec> {
        Dqs1AfifoOvfW::new(self, 29)
    }
    #[doc = "Bit 30 - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo1_udf(&mut self) -> BusFifo1UdfW<'_, IntClrSpec> {
        BusFifo1UdfW::new(self, 30)
    }
    #[doc = "Bit 31 - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo0_udf(&mut self) -> BusFifo0UdfW<'_, IntClrSpec> {
        BusFifo0UdfW::new(self, 31)
    }
}
#[doc = "SPI0 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfc00_03f8;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
