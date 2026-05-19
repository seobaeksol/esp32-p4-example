#[doc = "Register `STATUS_17` reader"]
pub type R = crate::R<Status17Spec>;
#[doc = "Field `DCAP_LPF` reader - Reserved"]
pub type DcapLpfR = crate::FieldReader;
#[doc = "Field `DRES_LPF` reader - need_des"]
pub type DresLpfR = crate::FieldReader;
#[doc = "Field `DRV_LS` reader - need_des"]
pub type DrvLsR = crate::FieldReader;
#[doc = "Field `DRV_HS` reader - need_des"]
pub type DrvHsR = crate::FieldReader;
#[doc = "Field `DBIAS` reader - need_des"]
pub type DbiasR = crate::FieldReader;
#[doc = "Field `RTC_FREQ_SCAN_CNT` reader - need_des"]
pub type RtcFreqScanCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn dcap_lpf(&self) -> DcapLpfR {
        DcapLpfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn dres_lpf(&self) -> DresLpfR {
        DresLpfR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn drv_ls(&self) -> DrvLsR {
        DrvLsR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn drv_hs(&self) -> DrvHsR {
        DrvHsR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn dbias(&self) -> DbiasR {
        DbiasR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    pub fn rtc_freq_scan_cnt(&self) -> RtcFreqScanCntR {
        RtcFreqScanCntR::new(((self.bits >> 23) & 3) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status17Spec;
impl crate::RegisterSpec for Status17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_17::R`](R) reader structure"]
impl crate::Readable for Status17Spec {}
#[doc = "`reset()` method sets STATUS_17 to value 0"]
impl crate::Resettable for Status17Spec {}
