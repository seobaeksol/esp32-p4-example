#[doc = "Register `TOUCH_FREQ0_SCAN_PARA` reader"]
pub type R = crate::R<TouchFreq0ScanParaSpec>;
#[doc = "Register `TOUCH_FREQ0_SCAN_PARA` writer"]
pub type W = crate::W<TouchFreq0ScanParaSpec>;
#[doc = "Field `TOUCH_FREQ0_DCAP_LPF` reader - need_des"]
pub type TouchFreq0DcapLpfR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ0_DCAP_LPF` writer - need_des"]
pub type TouchFreq0DcapLpfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TOUCH_FREQ0_DRES_LPF` reader - need_des"]
pub type TouchFreq0DresLpfR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ0_DRES_LPF` writer - need_des"]
pub type TouchFreq0DresLpfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_FREQ0_DRV_LS` reader - need_des"]
pub type TouchFreq0DrvLsR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ0_DRV_LS` writer - need_des"]
pub type TouchFreq0DrvLsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_FREQ0_DRV_HS` reader - need_des"]
pub type TouchFreq0DrvHsR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ0_DRV_HS` writer - need_des"]
pub type TouchFreq0DrvHsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOUCH_FREQ0_DBIAS` reader - need_des"]
pub type TouchFreq0DbiasR = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ0_DBIAS` writer - need_des"]
pub type TouchFreq0DbiasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_dcap_lpf(&self) -> TouchFreq0DcapLpfR {
        TouchFreq0DcapLpfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_dres_lpf(&self) -> TouchFreq0DresLpfR {
        TouchFreq0DresLpfR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_drv_ls(&self) -> TouchFreq0DrvLsR {
        TouchFreq0DrvLsR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_drv_hs(&self) -> TouchFreq0DrvHsR {
        TouchFreq0DrvHsR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_dbias(&self) -> TouchFreq0DbiasR {
        TouchFreq0DbiasR::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_dcap_lpf(&mut self) -> TouchFreq0DcapLpfW<'_, TouchFreq0ScanParaSpec> {
        TouchFreq0DcapLpfW::new(self, 0)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_dres_lpf(&mut self) -> TouchFreq0DresLpfW<'_, TouchFreq0ScanParaSpec> {
        TouchFreq0DresLpfW::new(self, 7)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_drv_ls(&mut self) -> TouchFreq0DrvLsW<'_, TouchFreq0ScanParaSpec> {
        TouchFreq0DrvLsW::new(self, 9)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_drv_hs(&mut self) -> TouchFreq0DrvHsW<'_, TouchFreq0ScanParaSpec> {
        TouchFreq0DrvHsW::new(self, 13)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn touch_freq0_dbias(&mut self) -> TouchFreq0DbiasW<'_, TouchFreq0ScanParaSpec> {
        TouchFreq0DbiasW::new(self, 18)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_freq0_scan_para::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_freq0_scan_para::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchFreq0ScanParaSpec;
impl crate::RegisterSpec for TouchFreq0ScanParaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_freq0_scan_para::R`](R) reader structure"]
impl crate::Readable for TouchFreq0ScanParaSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_freq0_scan_para::W`](W) writer structure"]
impl crate::Writable for TouchFreq0ScanParaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_FREQ0_SCAN_PARA to value 0"]
impl crate::Resettable for TouchFreq0ScanParaSpec {}
