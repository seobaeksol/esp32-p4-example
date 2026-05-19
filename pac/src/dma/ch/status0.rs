#[doc = "Register `STATUS0` reader"]
pub type R = crate::R<Status0Spec>;
#[doc = "Field `CH1_CMPLTD_BLK_TFR_SIZE` reader - NA"]
pub type Ch1CmpltdBlkTfrSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - NA"]
    #[inline(always)]
    pub fn ch1_cmpltd_blk_tfr_size(&self) -> Ch1CmpltdBlkTfrSizeR {
        Ch1CmpltdBlkTfrSizeR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status0Spec;
impl crate::RegisterSpec for Status0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status0::R`](R) reader structure"]
impl crate::Readable for Status0Spec {}
#[doc = "`reset()` method sets STATUS0 to value 0"]
impl crate::Resettable for Status0Spec {}
