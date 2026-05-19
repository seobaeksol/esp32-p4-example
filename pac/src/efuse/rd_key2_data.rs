#[doc = "Register `RD_KEY2_DATA%s` reader"]
pub type R = crate::R<RdKey2DataSpec>;
#[doc = "Field `KEY2_DATA` reader - Represents the zeroth 32-bit of key2."]
pub type Key2DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of key2."]
    #[inline(always)]
    pub fn key2_data(&self) -> Key2DataR {
        Key2DataR::new(self.bits)
    }
}
#[doc = "Represents rd_key2_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key2_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdKey2DataSpec;
impl crate::RegisterSpec for RdKey2DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key2_data::R`](R) reader structure"]
impl crate::Readable for RdKey2DataSpec {}
#[doc = "`reset()` method sets RD_KEY2_DATA%s to value 0"]
impl crate::Resettable for RdKey2DataSpec {}
