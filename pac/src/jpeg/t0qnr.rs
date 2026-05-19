#[doc = "Register `T0QNR` reader"]
pub type R = crate::R<T0qnrSpec>;
#[doc = "Field `T0_QNR_VAL` reader - write this reg to configure 64 quantization coefficient in t0 table"]
pub type T0QnrValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write this reg to configure 64 quantization coefficient in t0 table"]
    #[inline(always)]
    pub fn t0_qnr_val(&self) -> T0QnrValR {
        T0QnrValR::new(self.bits)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`t0qnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0qnrSpec;
impl crate::RegisterSpec for T0qnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0qnr::R`](R) reader structure"]
impl crate::Readable for T0qnrSpec {}
#[doc = "`reset()` method sets T0QNR to value 0"]
impl crate::Resettable for T0qnrSpec {}
