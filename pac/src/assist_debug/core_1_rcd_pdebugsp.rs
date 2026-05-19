#[doc = "Register `CORE_1_RCD_PDEBUGSP` reader"]
pub type R = crate::R<Core1RcdPdebugspSpec>;
#[doc = "Field `CORE_1_RCD_PDEBUGSP` reader - recorded sp"]
pub type Core1RcdPdebugspR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - recorded sp"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugsp(&self) -> Core1RcdPdebugspR {
        Core1RcdPdebugspR::new(self.bits)
    }
}
#[doc = "record status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_rcd_pdebugsp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1RcdPdebugspSpec;
impl crate::RegisterSpec for Core1RcdPdebugspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_rcd_pdebugsp::R`](R) reader structure"]
impl crate::Readable for Core1RcdPdebugspSpec {}
#[doc = "`reset()` method sets CORE_1_RCD_PDEBUGSP to value 0"]
impl crate::Resettable for Core1RcdPdebugspSpec {}
