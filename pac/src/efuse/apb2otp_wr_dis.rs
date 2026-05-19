#[doc = "Register `APB2OTP_WR_DIS` reader"]
pub type R = crate::R<Apb2otpWrDisSpec>;
#[doc = "Field `APB2OTP_BLOCK0_WR_DIS` reader - Otp block0 write disable data."]
pub type Apb2otpBlock0WrDisR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 write disable data."]
    #[inline(always)]
    pub fn apb2otp_block0_wr_dis(&self) -> Apb2otpBlock0WrDisR {
        Apb2otpBlock0WrDisR::new(self.bits)
    }
}
#[doc = "eFuse apb2otp block0 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_wr_dis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2otpWrDisSpec;
impl crate::RegisterSpec for Apb2otpWrDisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_wr_dis::R`](R) reader structure"]
impl crate::Readable for Apb2otpWrDisSpec {}
#[doc = "`reset()` method sets APB2OTP_WR_DIS to value 0"]
impl crate::Resettable for Apb2otpWrDisSpec {}
