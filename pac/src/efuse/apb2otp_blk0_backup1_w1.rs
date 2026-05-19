#[doc = "Register `APB2OTP_BLK0_BACKUP1_W1` reader"]
pub type R = crate::R<Apb2otpBlk0Backup1W1Spec>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP1_W1` reader - Otp block0 backup1 word1 data."]
pub type Apb2otpBlock0Backup1W1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup1 word1 data."]
    #[inline(always)]
    pub fn apb2otp_block0_backup1_w1(&self) -> Apb2otpBlock0Backup1W1R {
        Apb2otpBlock0Backup1W1R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block0 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk0Backup1W1Spec;
impl crate::RegisterSpec for Apb2otpBlk0Backup1W1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk0_backup1_w1::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk0Backup1W1Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK0_BACKUP1_W1 to value 0"]
impl crate::Resettable for Apb2otpBlk0Backup1W1Spec {}
