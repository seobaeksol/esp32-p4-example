#[doc = "Register `RD_KEY0_DATA%s` reader"]
pub type R = crate::R<RdKey0DataSpec>;
#[doc = "Field `KEY0_DATA` reader - Represents the zeroth 32-bit of key0."]
pub type Key0DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of key0."]
    #[inline(always)]
    pub fn key0_data(&self) -> Key0DataR {
        Key0DataR::new(self.bits)
    }
}
#[doc = "Represents rd_key0_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key0_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdKey0DataSpec;
impl crate::RegisterSpec for RdKey0DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key0_data::R`](R) reader structure"]
impl crate::Readable for RdKey0DataSpec {}
#[doc = "`reset()` method sets RD_KEY0_DATA%s to value 0"]
impl crate::Resettable for RdKey0DataSpec {}
