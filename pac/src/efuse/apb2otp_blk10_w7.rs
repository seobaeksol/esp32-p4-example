#[doc = "Register `APB2OTP_BLK10_W7` reader"]
pub type R = crate::R<Apb2otpBlk10W7Spec>;
#[doc = "Field `APB2OTP_BLOCK10_W7` reader - Otp block10 word7 data."]
pub type Apb2otpBlock10W7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word7 data."]
    #[inline(always)]
    pub fn apb2otp_block10_w7(&self) -> Apb2otpBlock10W7R {
        Apb2otpBlock10W7R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block10 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk10W7Spec;
impl crate::RegisterSpec for Apb2otpBlk10W7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk10_w7::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk10W7Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK10_W7 to value 0"]
impl crate::Resettable for Apb2otpBlk10W7Spec {}
