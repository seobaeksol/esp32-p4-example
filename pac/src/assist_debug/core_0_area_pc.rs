#[doc = "Register `CORE_0_AREA_PC` reader"]
pub type R = crate::R<Core0AreaPcSpec>;
#[doc = "Field `CORE_0_AREA_PC` reader - the stackpointer when first touch region monitor interrupt"]
pub type Core0AreaPcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - the stackpointer when first touch region monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_pc(&self) -> Core0AreaPcR {
        Core0AreaPcR::new(self.bits)
    }
}
#[doc = "core0 area pc status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_pc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0AreaPcSpec;
impl crate::RegisterSpec for Core0AreaPcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_area_pc::R`](R) reader structure"]
impl crate::Readable for Core0AreaPcSpec {}
#[doc = "`reset()` method sets CORE_0_AREA_PC to value 0"]
impl crate::Resettable for Core0AreaPcSpec {}
