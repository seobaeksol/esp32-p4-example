#[doc = "Register `APB2OTP_BLK7_W8` reader"]
pub type R = crate::R<Apb2otpBlk7W8Spec>;
#[doc = "Field `APB2OTP_BLOCK7_W8` reader - Otp block7 word8 data."]
pub type Apb2otpBlock7W8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block7 word8 data."]
    #[inline(always)]
    pub fn apb2otp_block7_w8(&self) -> Apb2otpBlock7W8R {
        Apb2otpBlock7W8R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block7 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk7W8Spec;
impl crate::RegisterSpec for Apb2otpBlk7W8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk7_w8::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk7W8Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK7_W8 to value 0"]
impl crate::Resettable for Apb2otpBlk7W8Spec {}
