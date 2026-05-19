#[doc = "Register `T2QNR` reader"]
pub type R = crate::R<T2qnrSpec>;
#[doc = "Field `T2_QNR_VAL` reader - write this reg to configure 64 quantization coefficient in t2 table"]
pub type T2QnrValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write this reg to configure 64 quantization coefficient in t2 table"]
    #[inline(always)]
    pub fn t2_qnr_val(&self) -> T2QnrValR {
        T2QnrValR::new(self.bits)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`t2qnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T2qnrSpec;
impl crate::RegisterSpec for T2qnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t2qnr::R`](R) reader structure"]
impl crate::Readable for T2qnrSpec {}
#[doc = "`reset()` method sets T2QNR to value 0"]
impl crate::Resettable for T2qnrSpec {}
