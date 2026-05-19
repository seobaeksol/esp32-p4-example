#[doc = "Register `APB2OTP_BLK3_W2` reader"]
pub type R = crate::R<Apb2otpBlk3W2Spec>;
#[doc = "Field `APB2OTP_BLOCK3_W2` reader - Otp block3 word2 data."]
pub type Apb2otpBlock3W2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word2 data."]
    #[inline(always)]
    pub fn apb2otp_block3_w2(&self) -> Apb2otpBlock3W2R {
        Apb2otpBlock3W2R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block3 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk3W2Spec;
impl crate::RegisterSpec for Apb2otpBlk3W2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk3_w2::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk3W2Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK3_W2 to value 0"]
impl crate::Resettable for Apb2otpBlk3W2Spec {}
