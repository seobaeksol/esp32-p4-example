#[doc = "Register `APB2OTP_BLK9_W9` reader"]
pub type R = crate::R<Apb2otpBlk9W9Spec>;
#[doc = "Field `APB2OTP_BLOCK9_W9` reader - Otp block9 word9 data."]
pub type Apb2otpBlock9W9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word9 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w9(&self) -> Apb2otpBlock9W9R {
        Apb2otpBlock9W9R::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block9 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpBlk9W9Spec;
impl crate::RegisterSpec for Apb2otpBlk9W9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk9_w9::R`](R) reader structure"]
impl crate::Readable for Apb2otpBlk9W9Spec {}
#[doc = "`reset()` method sets APB2OTP_BLK9_W9 to value 0"]
impl crate::Resettable for Apb2otpBlk9W9Spec {}
