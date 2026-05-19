#[doc = "Register `AXI_ERR_ADDR` reader"]
pub type R = crate::R<AxiErrAddrSpec>;
#[doc = "Field `AXI_ERR_ADDR` reader - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
pub type AxiErrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26 - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
    #[inline(always)]
    pub fn axi_err_addr(&self) -> AxiErrAddrR {
        AxiErrAddrR::new(self.bits & 0x07ff_ffff)
    }
}
#[doc = "SPI0 AXI request error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiErrAddrSpec;
impl crate::RegisterSpec for AxiErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_err_addr::R`](R) reader structure"]
impl crate::Readable for AxiErrAddrSpec {}
#[doc = "`reset()` method sets AXI_ERR_ADDR to value 0"]
impl crate::Resettable for AxiErrAddrSpec {}
