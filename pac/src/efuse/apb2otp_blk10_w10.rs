#[doc = "Register `APB2OTP_BLK10_W10` reader"]
pub type R = crate::R<Apb2otpBlk10W10Spec>;
#[doc = "Field `APB2OTP_BLOCK19_W10` reader - Otp block10 word10 data."]
pub type Apb2otpBlock19W10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word10 data."]
    #[inline(always)]
    pub fn apb2otp_block19_w10(&self) -> Apb2otpBlock19W10R {
        Apb2otpBlock19W10R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block10 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk10W10Spec;
impl crate::RegisterSpec for Apb2otpBlk10W10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk10_w10::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk10W10Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK10_W10 to value 0"]
impl crate::Resettable for Apb2otpBlk10W10Spec {}
