#[doc = "Register `APB2OTP_BLK10_W5` reader"]
pub type R = crate::R<Apb2otpBlk10W5Spec>;
#[doc = "Field `APB2OTP_BLOCK10_W5` reader - Otp block10 word5 data."]
pub type Apb2otpBlock10W5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word5 data."]
    #[inline(always)]
    pub fn apb2otp_block10_w5(&self) -> Apb2otpBlock10W5R {
        Apb2otpBlock10W5R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block10 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk10W5Spec;
impl crate::RegisterSpec for Apb2otpBlk10W5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk10_w5::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk10W5Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK10_W5 to value 0"]
impl crate::Resettable for Apb2otpBlk10W5Spec {}
