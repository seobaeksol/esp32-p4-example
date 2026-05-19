#[doc = "Register `RD_KEY4_DATA%s` reader"]
pub type R = crate::R<RdKey4DataSpec>;
#[doc = "Field `KEY4_DATA` reader - Represents the zeroth 32-bit of key4."]
pub type Key4DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of key4."]
    #[inline(always)]
    pub fn key4_data(&self) -> Key4DataR {
        Key4DataR::new(self.bits)
    }
}
#[doc = "Represents rd_key4_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key4_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdKey4DataSpec;
impl crate::RegisterSpec for RdKey4DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key4_data::R`](R) reader structure"]
impl crate::Readable for RdKey4DataSpec {}
#[doc = "`reset()` method sets RD_KEY4_DATA%s to value 0"]
impl crate::Resettable for RdKey4DataSpec {}
