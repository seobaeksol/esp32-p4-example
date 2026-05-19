#[doc = "Register `RD_KEY5_DATA%s` reader"]
pub type R = crate::R<RdKey5DataSpec>;
#[doc = "Field `KEY5_DATA` reader - Represents the zeroth 32-bit of key5."]
pub type Key5DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of key5."]
    #[inline(always)]
    pub fn key5_data(&self) -> Key5DataR {
        Key5DataR::new(self.bits)
    }
}
#[doc = "Represents rd_key5_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key5_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdKey5DataSpec;
impl crate::RegisterSpec for RdKey5DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key5_data::R`](R) reader structure"]
impl crate::Readable for RdKey5DataSpec {}
#[doc = "`reset()` method sets RD_KEY5_DATA%s to value 0"]
impl crate::Resettable for RdKey5DataSpec {}
