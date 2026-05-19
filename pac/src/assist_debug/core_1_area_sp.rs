#[doc = "Register `CORE_1_AREA_SP` reader"]
pub type R = crate::R<Core1AreaSpSpec>;
#[doc = "Field `CORE_1_AREA_SP` reader - the PC when first touch region monitor interrupt"]
pub type Core1AreaSpR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - the PC when first touch region monitor interrupt"]
    #[inline(always)]
    pub fn core_1_area_sp(&self) -> Core1AreaSpR {
        Core1AreaSpR::new(self.bits)
    }
}
#[doc = "core1 area sp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_area_sp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1AreaSpSpec;
impl crate::RegisterSpec for Core1AreaSpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_area_sp::R`](R) reader structure"]
impl crate::Readable for Core1AreaSpSpec {}
#[doc = "`reset()` method sets CORE_1_AREA_SP to value 0"]
impl crate::Resettable for Core1AreaSpSpec {}
