#[doc = "Register `RX_DATA_PORT` reader"]
pub type R = crate::R<RxDataPortSpec>;
#[doc = "Field `RX_DATA_PORT` reader - Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
pub type RxDataPortR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
    #[inline(always)]
    pub fn rx_data_port(&self) -> RxDataPortR {
        RxDataPortR::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_port::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDataPortSpec;
impl crate::RegisterSpec for RxDataPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_port::R`](R) reader structure"]
impl crate::Readable for RxDataPortSpec {}
#[doc = "`reset()` method sets RX_DATA_PORT to value 0"]
impl crate::Resettable for RxDataPortSpec {}
