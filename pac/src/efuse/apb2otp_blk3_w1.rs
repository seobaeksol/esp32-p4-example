#[doc = "Register `APB2OTP_BLK3_W1` reader"]
pub type R = crate::R<Apb2otpBlk3W1Spec>;
#[doc = "Field `APB2OTP_BLOCK3_W1` reader - Otp block3 word1 data."]
pub type Apb2otpBlock3W1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word1 data."]
    #[inline(always)]
    pub fn apb2otp_block3_w1(&self) -> Apb2otpBlock3W1R {
        Apb2otpBlock3W1R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block3 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk3W1Spec;
impl crate::RegisterSpec for Apb2otpBlk3W1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk3_w1::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk3W1Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK3_W1 to value 0"]
impl crate::Resettable for Apb2otpBlk3W1Spec {}
