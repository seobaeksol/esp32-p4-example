#[doc = "Register `INTR_RAW` reader"]
pub type R = crate::R<IntrRawSpec>;
#[doc = "Field `FIFO_OVERFLOW_INTR_RAW` reader - fifo_overflow interrupt status"]
pub type FifoOverflowIntrRawR = crate::BitReader;
#[doc = "Field `MEM_FULL_INTR_RAW` reader - mem_full interrupt status"]
pub type MemFullIntrRawR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - fifo_overflow interrupt status"]
    #[inline(always)]
    pub fn fifo_overflow_intr_raw(&self) -> FifoOverflowIntrRawR {
        FifoOverflowIntrRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mem_full interrupt status"]
    #[inline(always)]
    pub fn mem_full_intr_raw(&self) -> MemFullIntrRawR {
        MemFullIntrRawR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRawSpec;
impl crate::RegisterSpec for IntrRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_raw::R`](R) reader structure"]
impl crate::Readable for IntrRawSpec {}
#[doc = "`reset()` method sets INTR_RAW to value 0"]
impl crate::Resettable for IntrRawSpec {}
