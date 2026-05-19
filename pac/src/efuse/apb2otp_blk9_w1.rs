#[doc = "Register `APB2OTP_BLK9_W1` reader"]
pub type R = crate::R<Apb2otpBlk9W1Spec>;
#[doc = "Field `APB2OTP_BLOCK9_W1` reader - Otp block9 word1 data."]
pub type Apb2otpBlock9W1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word1 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w1(&self) -> Apb2otpBlock9W1R {
        Apb2otpBlock9W1R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block9 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk9W1Spec;
impl crate::RegisterSpec for Apb2otpBlk9W1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk9_w1::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk9W1Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK9_W1 to value 0"]
impl crate::Resettable for Apb2otpBlk9W1Spec {}
