#[doc = "Register `TXFIFO_START_ADDR` reader"]
pub type R = crate::R<TxfifoStartAddrSpec>;
#[doc = "Field `TXFIFO_START_ADDR` reader - Represents the I2C txfifo first address."]
pub type TxfifoStartAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the I2C txfifo first address."]
    #[inline(always)]
    pub fn txfifo_start_addr(&self) -> TxfifoStartAddrR {
        TxfifoStartAddrR::new(self.bits)
    }
}
#[doc = "I2C TXFIFO base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifo_start_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifoStartAddrSpec;
impl crate::RegisterSpec for TxfifoStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifo_start_addr::R`](R) reader structure"]
impl crate::Readable for TxfifoStartAddrSpec {}
#[doc = "`reset()` method sets TXFIFO_START_ADDR to value 0"]
impl crate::Resettable for TxfifoStartAddrSpec {}
