#[doc = "Register `APB2OTP_BLK5_W6` reader"]
pub type R = crate::R<Apb2otpBlk5W6Spec>;
#[doc = "Field `APB2OTP_BLOCK5_W6` reader - Otp block5 word6 data."]
pub type Apb2otpBlock5W6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block5 word6 data."]
    #[inline(always)]
    pub fn apb2otp_block5_w6(&self) -> Apb2otpBlock5W6R {
        Apb2otpBlock5W6R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block5 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk5W6Spec;
impl crate::RegisterSpec for Apb2otpBlk5W6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk5_w6::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk5W6Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK5_W6 to value 0"]
impl crate::Resettable for Apb2otpBlk5W6Spec {}
