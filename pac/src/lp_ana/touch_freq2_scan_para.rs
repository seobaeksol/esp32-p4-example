#[doc = "Register `TOUCH_FREQ2_SCAN_PARA` reader"]
pub type R = crate::R<TouchFreq2ScanParaSpec>;
#[doc = "Register `TOUCH_FREQ2_SCAN_PARA` writer"]
pub type W = crate::W<TouchFreq2ScanParaSpec>;
#[doc = "Field `TOUCH_FREQ2_DCAP_LPF` reader - need_des"]
pub type TouchFreq2DcapLpfR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DCAP_LPF` writer - need_des"]
pub type TouchFreq2DcapLpfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TOUCH_FREQ2_DRES_LPF` reader - need_des"]
pub type TouchFreq2DresLpfR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DRES_LPF` writer - need_des"]
pub type TouchFreq2DresLpfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_FREQ2_DRV_LS` reader - need_des"]
pub type TouchFreq2DrvLsR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DRV_LS` writer - need_des"]
pub type TouchFreq2DrvLsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_FREQ2_DRV_HS` reader - need_des"]
pub type TouchFreq2DrvHsR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DRV_HS` writer - need_des"]
pub type TouchFreq2DrvHsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOUCH_FREQ2_DBIAS` reader - need_des"]
pub type TouchFreq2DbiasR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DBIAS` writer - need_des"]
pub type TouchFreq2DbiasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dcap_lpf(&self) -> TouchFreq2DcapLpfR {
        TouchFreq2DcapLpfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dres_lpf(&self) -> TouchFreq2DresLpfR {
        TouchFreq2DresLpfR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_drv_ls(&self) -> TouchFreq2DrvLsR {
        TouchFreq2DrvLsR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_drv_hs(&self) -> TouchFreq2DrvHsR {
        TouchFreq2DrvHsR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dbias(&self) -> TouchFreq2DbiasR {
        TouchFreq2DbiasR::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dcap_lpf(&mut self) -> TouchFreq2DcapLpfW<'_, TouchFreq2ScanParaSpec> {
        TouchFreq2DcapLpfW::new(self, 0)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dres_lpf(&mut self) -> TouchFreq2DresLpfW<'_, TouchFreq2ScanParaSpec> {
        TouchFreq2DresLpfW::new(self, 7)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_drv_ls(&mut self) -> TouchFreq2DrvLsW<'_, TouchFreq2ScanParaSpec> {
        TouchFreq2DrvLsW::new(self, 9)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_drv_hs(&mut self) -> TouchFreq2DrvHsW<'_, TouchFreq2ScanParaSpec> {
        TouchFreq2DrvHsW::new(self, 13)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dbias(&mut self) -> TouchFreq2DbiasW<'_, TouchFreq2ScanParaSpec> {
        TouchFreq2DbiasW::new(self, 18)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_freq2_scan_para::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_freq2_scan_para::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchFreq2ScanParaSpec;
impl crate::RegisterSpec for TouchFreq2ScanParaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_freq2_scan_para::R`](R) reader structure"]
impl crate::Readable for TouchFreq2ScanParaSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_freq2_scan_para::W`](W) writer structure"]
impl crate::Writable for TouchFreq2ScanParaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_FREQ2_SCAN_PARA to value 0"]
impl crate::Resettable for TouchFreq2ScanParaSpec {}
