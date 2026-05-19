#[doc = "Register `FIFO_STATUS` reader"]
pub type R = crate::R<FifoStatusSpec>;
#[doc = "Field `FIFO_EMPTY` reader - Represent whether the fifo is empty. \\\\1: empty \\\\0: not empty"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `WORK_STATUS` reader - Represent trace work status: \\\\0: idle state \\\\1: working state\\\\ 2: wait state due to hart halted or havereset \\\\3: lost state"]
pub type WorkStatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Represent whether the fifo is empty. \\\\1: empty \\\\0: not empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Represent trace work status: \\\\0: idle state \\\\1: working state\\\\ 2: wait state due to hart halted or havereset \\\\3: lost state"]
    #[inline(always)]
    pub fn work_status(&self) -> WorkStatusR {
        WorkStatusR::new(((self.bits >> 1) & 3) as u8)
    }
}
#[doc = "fifo status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoStatusSpec;
impl crate::RegisterSpec for FifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_status::R`](R) reader structure"]
impl crate::Readable for FifoStatusSpec {}
#[doc = "`reset()` method sets FIFO_STATUS to value 0x01"]
impl crate::Resettable for FifoStatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
