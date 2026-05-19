#[doc = "Register `DECODER_STATUS2` reader"]
pub type R = crate::R<DecoderStatus2Spec>;
#[doc = "Field `COMP_BLOCK_NUM` reader - Reserved"]
pub type CompBlockNumR = crate::FieldReader<u32>;
#[doc = "Field `SCAN_NUM` reader - Reserved"]
pub type ScanNumR = crate::FieldReader;
#[doc = "Field `RST_CHECK_WAIT` reader - Reserved"]
pub type RstCheckWaitR = crate::BitReader;
#[doc = "Field `SCAN_CHECK_WAIT` reader - Reserved"]
pub type ScanCheckWaitR = crate::BitReader;
#[doc = "Field `MCU_IN_PROC` reader - Reserved"]
pub type McuInProcR = crate::BitReader;
impl R {
    #[doc = "Bits 0:25 - Reserved"]
    #[inline(always)]
    pub fn comp_block_num(&self) -> CompBlockNumR {
        CompBlockNumR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:28 - Reserved"]
    #[inline(always)]
    pub fn scan_num(&self) -> ScanNumR {
        ScanNumR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn rst_check_wait(&self) -> RstCheckWaitR {
        RstCheckWaitR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn scan_check_wait(&self) -> ScanCheckWaitR {
        ScanCheckWaitR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn mcu_in_proc(&self) -> McuInProcR {
        McuInProcR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder_status2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecoderStatus2Spec;
impl crate::RegisterSpec for DecoderStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder_status2::R`](R) reader structure"]
impl crate::Readable for DecoderStatus2Spec {}
#[doc = "`reset()` method sets DECODER_STATUS2 to value 0"]
impl crate::Resettable for DecoderStatus2Spec {}
