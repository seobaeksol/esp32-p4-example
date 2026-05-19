#[doc = "Register `APB2OTP_BLK8_W11` reader"]
pub type R = crate::R<Apb2otpBlk8W11Spec>;
#[doc = "Field `APB2OTP_BLOCK8_W11` reader - Otp block8 word11 data."]
pub type Apb2otpBlock8W11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block8 word11 data."]
    #[inline(always)]
    pub fn apb2otp_block8_w11(&self) -> Apb2otpBlock8W11R {
        Apb2otpBlock8W11R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block8 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk8W11Spec;
impl crate::RegisterSpec for Apb2otpBlk8W11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk8_w11::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk8W11Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK8_W11 to value 0"]
impl crate::Resettable for Apb2otpBlk8W11Spec {}
