#[doc = "Register `APB2OTP_BLK0_BACKUP4_W4` reader"]
pub type R = crate::R<Apb2otpBlk0Backup4W4Spec>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP4_W4` reader - Otp block0 backup4 word4 data."]
pub type Apb2otpBlock0Backup4W4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup4 word4 data."]
    #[inline(always)]
    pub fn apb2otp_block0_backup4_w4(&self) -> Apb2otpBlock0Backup4W4R {
        Apb2otpBlock0Backup4W4R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block0 data register20.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk0Backup4W4Spec;
impl crate::RegisterSpec for Apb2otpBlk0Backup4W4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk0_backup4_w4::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk0Backup4W4Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK0_BACKUP4_W4 to value 0"]
impl crate::Resettable for Apb2otpBlk0Backup4W4Spec {}
