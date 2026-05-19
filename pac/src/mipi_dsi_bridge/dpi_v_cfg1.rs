#[doc = "Register `DPI_V_CFG1` reader"]
pub type R = crate::R<DpiVCfg1Spec>;
#[doc = "Register `DPI_V_CFG1` writer"]
pub type W = crate::W<DpiVCfg1Spec>;
#[doc = "Field `VBANK` reader - this field configures the length between vsync and valid line (by line) for dpi output"]
pub type VbankR = crate::FieldReader<u16>;
#[doc = "Field `VBANK` writer - this field configures the length between vsync and valid line (by line) for dpi output"]
pub type VbankW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VSYNC` reader - this field configures the length of vsync (by line) for dpi output"]
pub type VsyncR = crate::FieldReader<u16>;
#[doc = "Field `VSYNC` writer - this field configures the length of vsync (by line) for dpi output"]
pub type VsyncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures the length between vsync and valid line (by line) for dpi output"]
    #[inline(always)]
    pub fn vbank(&self) -> VbankR {
        VbankR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures the length of vsync (by line) for dpi output"]
    #[inline(always)]
    pub fn vsync(&self) -> VsyncR {
        VsyncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures the length between vsync and valid line (by line) for dpi output"]
    #[inline(always)]
    pub fn vbank(&mut self) -> VbankW<'_, DpiVCfg1Spec> {
        VbankW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures the length of vsync (by line) for dpi output"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VsyncW<'_, DpiVCfg1Spec> {
        VsyncW::new(self, 16)
    }
}
#[doc = "dsi bridge dpi v config register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_v_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_v_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiVCfg1Spec;
impl crate::RegisterSpec for DpiVCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_v_cfg1::R`](R) reader structure"]
impl crate::Readable for DpiVCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpi_v_cfg1::W`](W) writer structure"]
impl crate::Writable for DpiVCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_V_CFG1 to value 0x0002_0021"]
impl crate::Resettable for DpiVCfg1Spec {
    const RESET_VALUE: u32 = 0x0002_0021;
}
