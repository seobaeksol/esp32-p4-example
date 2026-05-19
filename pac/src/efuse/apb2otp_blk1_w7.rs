#[doc = "Register `APB2OTP_BLK1_W7` reader"]
pub type R = crate::R<Apb2otpBlk1W7Spec>;
#[doc = "Field `APB2OTP_BLOCK1_W7` reader - Otp block1 word7 data."]
pub type Apb2otpBlock1W7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block1 word7 data."]
    #[inline(always)]
    pub fn apb2otp_block1_w7(&self) -> Apb2otpBlock1W7R {
        Apb2otpBlock1W7R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block1 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk1W7Spec;
impl crate::RegisterSpec for Apb2otpBlk1W7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk1_w7::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk1W7Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK1_W7 to value 0"]
impl crate::Resettable for Apb2otpBlk1W7Spec {}
