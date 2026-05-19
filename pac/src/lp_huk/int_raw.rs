#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Field `PREP_DONE` reader - The raw interrupt status bit for the huk_prep_done_int interrupt"]
pub type PrepDoneR = crate::BitReader;
#[doc = "Field `PROC_DONE` reader - The raw interrupt status bit for the huk_proc_done_int interrupt"]
pub type ProcDoneR = crate::BitReader;
#[doc = "Field `POST_DONE` reader - The raw interrupt status bit for the huk_post_done_int interrupt"]
pub type PostDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the huk_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done(&self) -> PrepDoneR {
        PrepDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the huk_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done(&self) -> ProcDoneR {
        ProcDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the huk_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done(&self) -> PostDoneR {
        PostDoneR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "HUK Generator interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
