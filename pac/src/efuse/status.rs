#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `STATE` reader - Indicates the state of the eFuse state machine."]
pub type StateR = crate::FieldReader;
#[doc = "Field `OTP_LOAD_SW` reader - The value of OTP_LOAD_SW."]
pub type OtpLoadSwR = crate::BitReader;
#[doc = "Field `OTP_VDDQ_C_SYNC2` reader - The value of OTP_VDDQ_C_SYNC2."]
pub type OtpVddqCSync2R = crate::BitReader;
#[doc = "Field `OTP_STROBE_SW` reader - The value of OTP_STROBE_SW."]
pub type OtpStrobeSwR = crate::BitReader;
#[doc = "Field `OTP_CSB_SW` reader - The value of OTP_CSB_SW."]
pub type OtpCsbSwR = crate::BitReader;
#[doc = "Field `OTP_PGENB_SW` reader - The value of OTP_PGENB_SW."]
pub type OtpPgenbSwR = crate::BitReader;
#[doc = "Field `OTP_VDDQ_IS_SW` reader - The value of OTP_VDDQ_IS_SW."]
pub type OtpVddqIsSwR = crate::BitReader;
#[doc = "Field `BLK0_VALID_BIT_CNT` reader - Indicates the number of block valid bit."]
pub type Blk0ValidBitCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Indicates the state of the eFuse state machine."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - The value of OTP_LOAD_SW."]
    #[inline(always)]
    pub fn otp_load_sw(&self) -> OtpLoadSwR {
        OtpLoadSwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The value of OTP_VDDQ_C_SYNC2."]
    #[inline(always)]
    pub fn otp_vddq_c_sync2(&self) -> OtpVddqCSync2R {
        OtpVddqCSync2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The value of OTP_STROBE_SW."]
    #[inline(always)]
    pub fn otp_strobe_sw(&self) -> OtpStrobeSwR {
        OtpStrobeSwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of OTP_CSB_SW."]
    #[inline(always)]
    pub fn otp_csb_sw(&self) -> OtpCsbSwR {
        OtpCsbSwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of OTP_PGENB_SW."]
    #[inline(always)]
    pub fn otp_pgenb_sw(&self) -> OtpPgenbSwR {
        OtpPgenbSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of OTP_VDDQ_IS_SW."]
    #[inline(always)]
    pub fn otp_vddq_is_sw(&self) -> OtpVddqIsSwR {
        OtpVddqIsSwR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:19 - Indicates the number of block valid bit."]
    #[inline(always)]
    pub fn blk0_valid_bit_cnt(&self) -> Blk0ValidBitCntR {
        Blk0ValidBitCntR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[doc = "eFuse status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
