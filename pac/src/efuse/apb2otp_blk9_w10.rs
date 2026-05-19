#[doc = "Register `APB2OTP_BLK9_W10` reader"]
pub type R = crate::R<Apb2otpBlk9W10Spec>;
#[doc = "Field `APB2OTP_BLOCK9_W10` reader - Otp block9 word10 data."]
pub type Apb2otpBlock9W10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word10 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w10(&self) -> Apb2otpBlock9W10R {
        Apb2otpBlock9W10R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block9 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk9W10Spec;
impl crate::RegisterSpec for Apb2otpBlk9W10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk9_w10::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk9W10Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK9_W10 to value 0"]
impl crate::Resettable for Apb2otpBlk9W10Spec {}
