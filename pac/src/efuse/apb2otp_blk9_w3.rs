#[doc = "Register `APB2OTP_BLK9_W3` reader"]
pub type R = crate::R<Apb2otpBlk9W3Spec>;
#[doc = "Field `APB2OTP_BLOCK9_W3` reader - Otp block9 word3 data."]
pub type Apb2otpBlock9W3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word3 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w3(&self) -> Apb2otpBlock9W3R {
        Apb2otpBlock9W3R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block9 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk9W3Spec;
impl crate::RegisterSpec for Apb2otpBlk9W3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk9_w3::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk9W3Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK9_W3 to value 0"]
impl crate::Resettable for Apb2otpBlk9W3Spec {}
