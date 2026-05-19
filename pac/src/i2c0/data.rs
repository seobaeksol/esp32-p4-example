#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Field `FIFO_RDATA` reader - Represents the value of RXFIFO read data."]
pub type FifoRdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Represents the value of RXFIFO read data."]
    #[inline(always)]
    pub fn fifo_rdata(&self) -> FifoRdataR {
        FifoRdataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Rx FIFO read data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
