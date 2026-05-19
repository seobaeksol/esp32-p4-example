#[doc = "Register `MEM_AUX_CTRL_1` reader"]
pub type R = crate::R<MemAuxCtrl1Spec>;
#[doc = "Register `MEM_AUX_CTRL_1` writer"]
pub type W = crate::W<MemAuxCtrl1Spec>;
#[doc = "Field `LSC_LUT_R_GR_MEM_AUX_CTRL` reader - this field configures the mem_aux of lsc r gr lut memory"]
pub type LscLutRGrMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `LSC_LUT_R_GR_MEM_AUX_CTRL` writer - this field configures the mem_aux of lsc r gr lut memory"]
pub type LscLutRGrMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSC_LUT_GB_B_MEM_AUX_CTRL` reader - this field configures the mem_aux of lsc gb b lut memory"]
pub type LscLutGbBMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `LSC_LUT_GB_B_MEM_AUX_CTRL` writer - this field configures the mem_aux of lsc gb b lut memory"]
pub type LscLutGbBMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of lsc r gr lut memory"]
    #[inline(always)]
    pub fn lsc_lut_r_gr_mem_aux_ctrl(&self) -> LscLutRGrMemAuxCtrlR {
        LscLutRGrMemAuxCtrlR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of lsc gb b lut memory"]
    #[inline(always)]
    pub fn lsc_lut_gb_b_mem_aux_ctrl(&self) -> LscLutGbBMemAuxCtrlR {
        LscLutGbBMemAuxCtrlR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of lsc r gr lut memory"]
    #[inline(always)]
    pub fn lsc_lut_r_gr_mem_aux_ctrl(&mut self) -> LscLutRGrMemAuxCtrlW<'_, MemAuxCtrl1Spec> {
        LscLutRGrMemAuxCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of lsc gb b lut memory"]
    #[inline(always)]
    pub fn lsc_lut_gb_b_mem_aux_ctrl(&mut self) -> LscLutGbBMemAuxCtrlW<'_, MemAuxCtrl1Spec> {
        LscLutGbBMemAuxCtrlW::new(self, 16)
    }
}
#[doc = "mem aux control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAuxCtrl1Spec;
impl crate::RegisterSpec for MemAuxCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_1::R`](R) reader structure"]
impl crate::Readable for MemAuxCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_1::W`](W) writer structure"]
impl crate::Writable for MemAuxCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_1 to value 0x1320_1320"]
impl crate::Resettable for MemAuxCtrl1Spec {
    const RESET_VALUE: u32 = 0x1320_1320;
}
