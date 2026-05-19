#[doc = "Register `RD_USR_DATA%s` reader"]
pub type R = crate::R<RdUsrDataSpec>;
#[doc = "Field `USR_DATA` reader - Represents the zeroth 32-bit of block3 (user)."]
pub type UsrDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of block3 (user)."]
    #[inline(always)]
    pub fn usr_data(&self) -> UsrDataR {
        UsrDataR::new(self.bits)
    }
}
#[doc = "Represents rd_usr_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_usr_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdUsrDataSpec;
impl crate::RegisterSpec for RdUsrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_usr_data::R`](R) reader structure"]
impl crate::Readable for RdUsrDataSpec {}
#[doc = "`reset()` method sets RD_USR_DATA%s to value 0"]
impl crate::Resettable for RdUsrDataSpec {}
