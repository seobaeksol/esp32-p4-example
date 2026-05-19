#[doc = "Register `IN` reader"]
pub type R = crate::R<InSpec>;
#[doc = "Field `DATA_NEXT` reader - GPIO input register for GPIO0-31"]
pub type DataNextR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO input register for GPIO0-31"]
    #[inline(always)]
    pub fn data_next(&self) -> DataNextR {
        DataNextR::new(self.bits)
    }
}
#[doc = "GPIO input register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for InSpec {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for InSpec {}
