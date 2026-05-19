#[doc = "Register `CORE_0_AREA_SP` reader"]
pub type R = crate::R<Core0AreaSpSpec>;
#[doc = "Field `CORE_0_AREA_SP` reader - the PC when first touch region monitor interrupt"]
pub type Core0AreaSpR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - the PC when first touch region monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_sp(&self) -> Core0AreaSpR {
        Core0AreaSpR::new(self.bits)
    }
}
#[doc = "core0 area sp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_sp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0AreaSpSpec;
impl crate::RegisterSpec for Core0AreaSpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_area_sp::R`](R) reader structure"]
impl crate::Readable for Core0AreaSpSpec {}
#[doc = "`reset()` method sets CORE_0_AREA_SP to value 0"]
impl crate::Resettable for Core0AreaSpSpec {}
