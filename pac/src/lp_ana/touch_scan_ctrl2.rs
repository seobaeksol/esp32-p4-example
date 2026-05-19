#[doc = "Register `TOUCH_SCAN_CTRL2` reader"]
pub type R = crate::R<TouchScanCtrl2Spec>;
#[doc = "Register `TOUCH_SCAN_CTRL2` writer"]
pub type W = crate::W<TouchScanCtrl2Spec>;
#[doc = "Field `TOUCH_TIMEOUT_NUM` reader - need_des"]
pub type TouchTimeoutNumR = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_TIMEOUT_NUM` writer - need_des"]
pub type TouchTimeoutNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_TIMEOUT_EN` reader - need_des"]
pub type TouchTimeoutEnR = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT_EN` writer - need_des"]
pub type TouchTimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_OUT_RING` reader - need_des"]
pub type TouchOutRingR = crate::FieldReader;
#[doc = "Field `TOUCH_OUT_RING` writer - need_des"]
pub type TouchOutRingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FREQ_SCAN_EN` reader - need_des"]
pub type FreqScanEnR = crate::BitReader;
#[doc = "Field `FREQ_SCAN_EN` writer - need_des"]
pub type FreqScanEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQ_SCAN_CNT_LIMIT` reader - need_des"]
pub type FreqScanCntLimitR = crate::FieldReader;
#[doc = "Field `FREQ_SCAN_CNT_LIMIT` writer - need_des"]
pub type FreqScanCntLimitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:21 - need_des"]
    #[inline(always)]
    pub fn touch_timeout_num(&self) -> TouchTimeoutNumR {
        TouchTimeoutNumR::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn touch_timeout_en(&self) -> TouchTimeoutEnR {
        TouchTimeoutEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn touch_out_ring(&self) -> TouchOutRingR {
        TouchOutRingR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn freq_scan_en(&self) -> FreqScanEnR {
        FreqScanEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn freq_scan_cnt_limit(&self) -> FreqScanCntLimitR {
        FreqScanCntLimitR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:21 - need_des"]
    #[inline(always)]
    pub fn touch_timeout_num(&mut self) -> TouchTimeoutNumW<'_, TouchScanCtrl2Spec> {
        TouchTimeoutNumW::new(self, 6)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn touch_timeout_en(&mut self) -> TouchTimeoutEnW<'_, TouchScanCtrl2Spec> {
        TouchTimeoutEnW::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn touch_out_ring(&mut self) -> TouchOutRingW<'_, TouchScanCtrl2Spec> {
        TouchOutRingW::new(self, 23)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn freq_scan_en(&mut self) -> FreqScanEnW<'_, TouchScanCtrl2Spec> {
        FreqScanEnW::new(self, 27)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn freq_scan_cnt_limit(&mut self) -> FreqScanCntLimitW<'_, TouchScanCtrl2Spec> {
        FreqScanCntLimitW::new(self, 28)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_scan_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_scan_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchScanCtrl2Spec;
impl crate::RegisterSpec for TouchScanCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_scan_ctrl2::R`](R) reader structure"]
impl crate::Readable for TouchScanCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`touch_scan_ctrl2::W`](W) writer structure"]
impl crate::Writable for TouchScanCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_SCAN_CTRL2 to value 0x37bf_ffc0"]
impl crate::Resettable for TouchScanCtrl2Spec {
    const RESET_VALUE: u32 = 0x37bf_ffc0;
}
