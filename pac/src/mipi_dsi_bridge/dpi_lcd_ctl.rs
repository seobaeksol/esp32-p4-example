#[doc = "Register `DPI_LCD_CTL` reader"]
pub type R = crate::R<DpiLcdCtlSpec>;
#[doc = "Register `DPI_LCD_CTL` writer"]
pub type W = crate::W<DpiLcdCtlSpec>;
#[doc = "Field `DPISHUTDN` reader - this bit configures dpishutdn signal in dpi interface"]
pub type DpishutdnR = crate::BitReader;
#[doc = "Field `DPISHUTDN` writer - this bit configures dpishutdn signal in dpi interface"]
pub type DpishutdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPICOLORM` reader - this bit configures dpicolorm signal in dpi interface"]
pub type DpicolormR = crate::BitReader;
#[doc = "Field `DPICOLORM` writer - this bit configures dpicolorm signal in dpi interface"]
pub type DpicolormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPIUPDATECFG` reader - this bit configures dpiupdatecfg signal in dpi interface"]
pub type DpiupdatecfgR = crate::BitReader;
#[doc = "Field `DPIUPDATECFG` writer - this bit configures dpiupdatecfg signal in dpi interface"]
pub type DpiupdatecfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures dpishutdn signal in dpi interface"]
    #[inline(always)]
    pub fn dpishutdn(&self) -> DpishutdnR {
        DpishutdnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures dpicolorm signal in dpi interface"]
    #[inline(always)]
    pub fn dpicolorm(&self) -> DpicolormR {
        DpicolormR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures dpiupdatecfg signal in dpi interface"]
    #[inline(always)]
    pub fn dpiupdatecfg(&self) -> DpiupdatecfgR {
        DpiupdatecfgR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures dpishutdn signal in dpi interface"]
    #[inline(always)]
    pub fn dpishutdn(&mut self) -> DpishutdnW<'_, DpiLcdCtlSpec> {
        DpishutdnW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures dpicolorm signal in dpi interface"]
    #[inline(always)]
    pub fn dpicolorm(&mut self) -> DpicolormW<'_, DpiLcdCtlSpec> {
        DpicolormW::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures dpiupdatecfg signal in dpi interface"]
    #[inline(always)]
    pub fn dpiupdatecfg(&mut self) -> DpiupdatecfgW<'_, DpiLcdCtlSpec> {
        DpiupdatecfgW::new(self, 2)
    }
}
#[doc = "dsi bridge dpi signal control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_lcd_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_lcd_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiLcdCtlSpec;
impl crate::RegisterSpec for DpiLcdCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_lcd_ctl::R`](R) reader structure"]
impl crate::Readable for DpiLcdCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_lcd_ctl::W`](W) writer structure"]
impl crate::Writable for DpiLcdCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_LCD_CTL to value 0"]
impl crate::Resettable for DpiLcdCtlSpec {}
