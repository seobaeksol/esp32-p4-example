#[doc = "Register `RTCCALICFG2` reader"]
pub type R = crate::R<Rtccalicfg2Spec>;
#[doc = "Register `RTCCALICFG2` writer"]
pub type W = crate::W<Rtccalicfg2Spec>;
#[doc = "Field `RTC_CALI_TIMEOUT` reader - RTC calibration timeout indicator"]
pub type RtcCaliTimeoutR = crate::BitReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` reader - Cycles that release calibration timeout reset"]
pub type RtcCaliTimeoutRstCntR = crate::FieldReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` writer - Cycles that release calibration timeout reset"]
pub type RtcCaliTimeoutRstCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` reader - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
pub type RtcCaliTimeoutThresR = crate::FieldReader<u32>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` writer - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
pub type RtcCaliTimeoutThresW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - RTC calibration timeout indicator"]
    #[inline(always)]
    pub fn rtc_cali_timeout(&self) -> RtcCaliTimeoutR {
        RtcCaliTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - Cycles that release calibration timeout reset"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&self) -> RtcCaliTimeoutRstCntR {
        RtcCaliTimeoutRstCntR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:31 - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&self) -> RtcCaliTimeoutThresR {
        RtcCaliTimeoutThresR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:6 - Cycles that release calibration timeout reset"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&mut self) -> RtcCaliTimeoutRstCntW<'_, Rtccalicfg2Spec> {
        RtcCaliTimeoutRstCntW::new(self, 3)
    }
    #[doc = "Bits 7:31 - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&mut self) -> RtcCaliTimeoutThresW<'_, Rtccalicfg2Spec> {
        RtcCaliTimeoutThresW::new(self, 7)
    }
}
#[doc = "Timer group calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccalicfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccalicfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtccalicfg2Spec;
impl crate::RegisterSpec for Rtccalicfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccalicfg2::R`](R) reader structure"]
impl crate::Readable for Rtccalicfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtccalicfg2::W`](W) writer structure"]
impl crate::Writable for Rtccalicfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCALICFG2 to value 0xffff_ff98"]
impl crate::Resettable for Rtccalicfg2Spec {
    const RESET_VALUE: u32 = 0xffff_ff98;
}
