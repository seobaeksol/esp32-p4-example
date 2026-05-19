#[doc = "Register `HI` reader"]
pub type R = crate::R<HiSpec>;
#[doc = "Field `HI_RO` reader - actual target value value high 20bits"]
pub type HiRoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - actual target value value high 20bits"]
    #[inline(always)]
    pub fn hi_ro(&self) -> HiRoR {
        HiRoR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "system timer comp0 actual target value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HiSpec;
impl crate::RegisterSpec for HiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hi::R`](R) reader structure"]
impl crate::Readable for HiSpec {}
#[doc = "`reset()` method sets HI to value 0"]
impl crate::Resettable for HiSpec {}
