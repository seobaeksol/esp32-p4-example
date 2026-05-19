#[doc = "Register `ECC_CTRL` reader"]
pub type R = crate::R<EccCtrlSpec>;
#[doc = "Register `ECC_CTRL` writer"]
pub type W = crate::W<EccCtrlSpec>;
#[doc = "Field `ECC_ERR_CNT` reader - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type EccErrCntR = crate::FieldReader;
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_NUM` reader - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub type SpiFmemEccErrIntNumR = crate::FieldReader;
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_NUM` writer - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub type SpiFmemEccErrIntNumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub type SpiFmemEccErrIntEnR = crate::BitReader;
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_EN` writer - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub type SpiFmemEccErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_PAGE_SIZE` reader - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SpiFmemPageSizeR = crate::FieldReader;
#[doc = "Field `SPI_FMEM_PAGE_SIZE` writer - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SpiFmemPageSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_FMEM_ECC_ADDR_EN` reader - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
pub type SpiFmemEccAddrEnR = crate::BitReader;
#[doc = "Field `SPI_FMEM_ECC_ADDR_EN` writer - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
pub type SpiFmemEccAddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_ECC_ADDR_EN` reader - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
pub type UsrEccAddrEnR = crate::BitReader;
#[doc = "Field `USR_ECC_ADDR_EN` writer - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
pub type UsrEccAddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CONTINUE_RECORD_ERR_EN` reader - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
pub type EccContinueRecordErrEnR = crate::BitReader;
#[doc = "Field `ECC_CONTINUE_RECORD_ERR_EN` writer - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
pub type EccContinueRecordErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_BITS` reader - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
pub type EccErrBitsR = crate::FieldReader;
impl R {
    #[doc = "Bits 5:10 - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn ecc_err_cnt(&self) -> EccErrCntR {
        EccErrCntR::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bits 11:16 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_num(&self) -> SpiFmemEccErrIntNumR {
        SpiFmemEccErrIntNumR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&self) -> SpiFmemEccErrIntEnR {
        SpiFmemEccErrIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn spi_fmem_page_size(&self) -> SpiFmemPageSizeR {
        SpiFmemPageSizeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn spi_fmem_ecc_addr_en(&self) -> SpiFmemEccAddrEnR {
        SpiFmemEccAddrEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
    #[inline(always)]
    pub fn usr_ecc_addr_en(&self) -> UsrEccAddrEnR {
        UsrEccAddrEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
    #[inline(always)]
    pub fn ecc_continue_record_err_en(&self) -> EccContinueRecordErrEnR {
        EccContinueRecordErrEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
    #[inline(always)]
    pub fn ecc_err_bits(&self) -> EccErrBitsR {
        EccErrBitsR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:16 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_num(&mut self) -> SpiFmemEccErrIntNumW<'_, EccCtrlSpec> {
        SpiFmemEccErrIntNumW::new(self, 11)
    }
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&mut self) -> SpiFmemEccErrIntEnW<'_, EccCtrlSpec> {
        SpiFmemEccErrIntEnW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn spi_fmem_page_size(&mut self) -> SpiFmemPageSizeW<'_, EccCtrlSpec> {
        SpiFmemPageSizeW::new(self, 18)
    }
    #[doc = "Bit 20 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn spi_fmem_ecc_addr_en(&mut self) -> SpiFmemEccAddrEnW<'_, EccCtrlSpec> {
        SpiFmemEccAddrEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
    #[inline(always)]
    pub fn usr_ecc_addr_en(&mut self) -> UsrEccAddrEnW<'_, EccCtrlSpec> {
        UsrEccAddrEnW::new(self, 21)
    }
    #[doc = "Bit 24 - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
    #[inline(always)]
    pub fn ecc_continue_record_err_en(&mut self) -> EccContinueRecordErrEnW<'_, EccCtrlSpec> {
        EccContinueRecordErrEnW::new(self, 24)
    }
}
#[doc = "MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccCtrlSpec;
impl crate::RegisterSpec for EccCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for EccCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for EccCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_CTRL to value 0x0100_5000"]
impl crate::Resettable for EccCtrlSpec {
    const RESET_VALUE: u32 = 0x0100_5000;
}
