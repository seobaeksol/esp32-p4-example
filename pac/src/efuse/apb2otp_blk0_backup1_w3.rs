#[doc = "Register `APB2OTP_BLK0_BACKUP1_W3` reader"]
pub type R = crate::R<Apb2otpBlk0Backup1W3Spec>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP1_W3` reader - Otp block0 backup1 word3 data."]
pub type Apb2otpBlock0Backup1W3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup1 word3 data."]
    #[inline(always)]
    pub fn apb2otp_block0_backup1_w3(&self) -> Apb2otpBlock0Backup1W3R {
        Apb2otpBlock0Backup1W3R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block0 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk0Backup1W3Spec;
impl crate::RegisterSpec for Apb2otpBlk0Backup1W3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk0_backup1_w3::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk0Backup1W3Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK0_BACKUP1_W3 to value 0"]
impl crate::Resettable for Apb2otpBlk0Backup1W3Spec {}
