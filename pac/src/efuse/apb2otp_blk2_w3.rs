#[doc = "Register `APB2OTP_BLK2_W3` reader"]
pub type R = crate::R<Apb2otpBlk2W3Spec>;
#[doc = "Field `APB2OTP_BLOCK2_W3` reader - Otp block2 word3 data."]
pub type Apb2otpBlock2W3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block2 word3 data."]
    #[inline(always)]
    pub fn apb2otp_block2_w3(&self) -> Apb2otpBlock2W3R {
        Apb2otpBlock2W3R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block2 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk2W3Spec;
impl crate::RegisterSpec for Apb2otpBlk2W3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk2_w3::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk2W3Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK2_W3 to value 0"]
impl crate::Resettable for Apb2otpBlk2W3Spec {}
