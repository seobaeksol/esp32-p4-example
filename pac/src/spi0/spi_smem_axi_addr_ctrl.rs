#[doc = "Register `SPI_SMEM_AXI_ADDR_CTRL` reader"]
pub type R = crate::R<SpiSmemAxiAddrCtrlSpec>;
#[doc = "Field `ALL_FIFO_EMPTY` reader - The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others."]
pub type AllFifoEmptyR = crate::BitReader;
#[doc = "Field `SPI_RDATA_AFIFO_REMPTY` reader - 1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending."]
pub type SpiRdataAfifoRemptyR = crate::BitReader;
#[doc = "Field `SPI_RADDR_AFIFO_REMPTY` reader - 1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending."]
pub type SpiRaddrAfifoRemptyR = crate::BitReader;
#[doc = "Field `SPI_WDATA_AFIFO_REMPTY` reader - 1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending."]
pub type SpiWdataAfifoRemptyR = crate::BitReader;
#[doc = "Field `SPI_WBLEN_AFIFO_REMPTY` reader - 1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending."]
pub type SpiWblenAfifoRemptyR = crate::BitReader;
#[doc = "Field `SPI_ALL_AXI_TRANS_AFIFO_EMPTY` reader - This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE."]
pub type SpiAllAxiTransAfifoEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 26 - The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others."]
    #[inline(always)]
    pub fn all_fifo_empty(&self) -> AllFifoEmptyR {
        AllFifoEmptyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending."]
    #[inline(always)]
    pub fn spi_rdata_afifo_rempty(&self) -> SpiRdataAfifoRemptyR {
        SpiRdataAfifoRemptyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending."]
    #[inline(always)]
    pub fn spi_raddr_afifo_rempty(&self) -> SpiRaddrAfifoRemptyR {
        SpiRaddrAfifoRemptyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending."]
    #[inline(always)]
    pub fn spi_wdata_afifo_rempty(&self) -> SpiWdataAfifoRemptyR {
        SpiWdataAfifoRemptyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending."]
    #[inline(always)]
    pub fn spi_wblen_afifo_rempty(&self) -> SpiWblenAfifoRemptyR {
        SpiWblenAfifoRemptyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE."]
    #[inline(always)]
    pub fn spi_all_axi_trans_afifo_empty(&self) -> SpiAllAxiTransAfifoEmptyR {
        SpiAllAxiTransAfifoEmptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SPI0 AXI address control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_axi_addr_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSmemAxiAddrCtrlSpec;
impl crate::RegisterSpec for SpiSmemAxiAddrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_axi_addr_ctrl::R`](R) reader structure"]
impl crate::Readable for SpiSmemAxiAddrCtrlSpec {}
#[doc = "`reset()` method sets SPI_SMEM_AXI_ADDR_CTRL to value 0xfc00_0000"]
impl crate::Resettable for SpiSmemAxiAddrCtrlSpec {
    const RESET_VALUE: u32 = 0xfc00_0000;
}
