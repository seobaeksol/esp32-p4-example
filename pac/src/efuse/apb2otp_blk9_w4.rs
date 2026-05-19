#[doc = "Register `APB2OTP_BLK9_W4` reader"]
pub type R = crate::R<Apb2otpBlk9W4Spec>;
#[doc = "Field `APB2OTP_BLOCK9_W4` reader - Otp block9 word4 data."]
pub type Apb2otpBlock9W4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word4 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w4(&self) -> Apb2otpBlock9W4R {
        Apb2otpBlock9W4R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block9 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk9W4Spec;
impl crate::RegisterSpec for Apb2otpBlk9W4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk9_w4::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk9W4Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK9_W4 to value 0"]
impl crate::Resettable for Apb2otpBlk9W4Spec {}
