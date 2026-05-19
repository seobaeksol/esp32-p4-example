#[doc = "Register `TOUCH_FREQ1_SCAN_PARA` reader"]
pub type R = crate::R<TouchFreq1ScanParaSpec>;
#[doc = "Register `TOUCH_FREQ1_SCAN_PARA` writer"]
pub type W = crate::W<TouchFreq1ScanParaSpec>;
#[doc = "Field `TOUCH_FREQ1_DCAP_LPF` reader - need_des"]
pub type TouchFreq1DcapLpfR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ1_DCAP_LPF` writer - need_des"]
pub type TouchFreq1DcapLpfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TOUCH_FREQ1_DRES_LPF` reader - need_des"]
pub type TouchFreq1DresLpfR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ1_DRES_LPF` writer - need_des"]
pub type TouchFreq1DresLpfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_FREQ1_DRV_LS` reader - need_des"]
pub type TouchFreq1DrvLsR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ1_DRV_LS` writer - need_des"]
pub type TouchFreq1DrvLsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_FREQ1_DRV_HS` reader - need_des"]
pub type TouchFreq1DrvHsR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ1_DRV_HS` writer - need_des"]
pub type TouchFreq1DrvHsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOUCH_FREQ1_DBIAS` reader - need_des"]
pub type TouchFreq1DbiasR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ1_DBIAS` writer - need_des"]
pub type TouchFreq1DbiasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_dcap_lpf(&self) -> TouchFreq1DcapLpfR {
        TouchFreq1DcapLpfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_dres_lpf(&self) -> TouchFreq1DresLpfR {
        TouchFreq1DresLpfR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_drv_ls(&self) -> TouchFreq1DrvLsR {
        TouchFreq1DrvLsR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_drv_hs(&self) -> TouchFreq1DrvHsR {
        TouchFreq1DrvHsR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_dbias(&self) -> TouchFreq1DbiasR {
        TouchFreq1DbiasR::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_dcap_lpf(&mut self) -> TouchFreq1DcapLpfW<'_, TouchFreq1ScanParaSpec> {
        TouchFreq1DcapLpfW::new(self, 0)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_dres_lpf(&mut self) -> TouchFreq1DresLpfW<'_, TouchFreq1ScanParaSpec> {
        TouchFreq1DresLpfW::new(self, 7)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_drv_ls(&mut self) -> TouchFreq1DrvLsW<'_, TouchFreq1ScanParaSpec> {
        TouchFreq1DrvLsW::new(self, 9)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_drv_hs(&mut self) -> TouchFreq1DrvHsW<'_, TouchFreq1ScanParaSpec> {
        TouchFreq1DrvHsW::new(self, 13)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn touch_freq1_dbias(&mut self) -> TouchFreq1DbiasW<'_, TouchFreq1ScanParaSpec> {
        TouchFreq1DbiasW::new(self, 18)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_freq1_scan_para::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_freq1_scan_para::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchFreq1ScanParaSpec;
impl crate::RegisterSpec for TouchFreq1ScanParaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_freq1_scan_para::R`](R) reader structure"]
impl crate::Readable for TouchFreq1ScanParaSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_freq1_scan_para::W`](W) writer structure"]
impl crate::Writable for TouchFreq1ScanParaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_FREQ1_SCAN_PARA to value 0"]
impl crate::Resettable for TouchFreq1ScanParaSpec {}
