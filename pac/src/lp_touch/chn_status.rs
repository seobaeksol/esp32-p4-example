#[doc = "Register `CHN_STATUS` reader"]
pub type R = crate::R<ChnStatusSpec>;
#[doc = "Field `PAD_ACTIVE` reader - need_des"]
pub type PadActiveR = crate::FieldReader<u16>;
#[doc = "Field `MEAS_DONE` reader - need_des"]
pub type MeasDoneR = crate::BitReader;
#[doc = "Field `SCAN_CURR` reader - need_des"]
pub type ScanCurrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14 - need_des"]
    #[inline(always)]
    pub fn pad_active(&self) -> PadActiveR {
        PadActiveR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn meas_done(&self) -> MeasDoneR {
        MeasDoneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - need_des"]
    #[inline(always)]
    pub fn scan_curr(&self) -> ScanCurrR {
        ScanCurrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnStatusSpec;
impl crate::RegisterSpec for ChnStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn_status::R`](R) reader structure"]
impl crate::Readable for ChnStatusSpec {}
#[doc = "`reset()` method sets CHN_STATUS to value 0x000f_0000"]
impl crate::Resettable for ChnStatusSpec {
    const RESET_VALUE: u32 = 0x000f_0000;
}
