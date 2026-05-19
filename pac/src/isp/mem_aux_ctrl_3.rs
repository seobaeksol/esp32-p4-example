#[doc = "Register `MEM_AUX_CTRL_3` reader"]
pub type R = crate::R<MemAuxCtrl3Spec>;
#[doc = "Register `MEM_AUX_CTRL_3` writer"]
pub type W = crate::W<MemAuxCtrl3Spec>;
#[doc = "Field `SHARP_MATRIX_Y_MEM_AUX_CTRL` reader - this field configures the mem_aux of sharp y line buffer memory"]
pub type SharpMatrixYMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `SHARP_MATRIX_Y_MEM_AUX_CTRL` writer - this field configures the mem_aux of sharp y line buffer memory"]
pub type SharpMatrixYMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DEMOSAIC_MATRIX_MEM_AUX_CTRL` reader - this field configures the mem_aux of demosaic line buffer memory"]
pub type DemosaicMatrixMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `DEMOSAIC_MATRIX_MEM_AUX_CTRL` writer - this field configures the mem_aux of demosaic line buffer memory"]
pub type DemosaicMatrixMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of sharp y line buffer memory"]
    #[inline(always)]
    pub fn sharp_matrix_y_mem_aux_ctrl(&self) -> SharpMatrixYMemAuxCtrlR {
        SharpMatrixYMemAuxCtrlR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of demosaic line buffer memory"]
    #[inline(always)]
    pub fn demosaic_matrix_mem_aux_ctrl(&self) -> DemosaicMatrixMemAuxCtrlR {
        DemosaicMatrixMemAuxCtrlR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of sharp y line buffer memory"]
    #[inline(always)]
    pub fn sharp_matrix_y_mem_aux_ctrl(&mut self) -> SharpMatrixYMemAuxCtrlW<'_, MemAuxCtrl3Spec> {
        SharpMatrixYMemAuxCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of demosaic line buffer memory"]
    #[inline(always)]
    pub fn demosaic_matrix_mem_aux_ctrl(
        &mut self,
    ) -> DemosaicMatrixMemAuxCtrlW<'_, MemAuxCtrl3Spec> {
        DemosaicMatrixMemAuxCtrlW::new(self, 16)
    }
}
#[doc = "mem aux control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAuxCtrl3Spec;
impl crate::RegisterSpec for MemAuxCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_3::R`](R) reader structure"]
impl crate::Readable for MemAuxCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_3::W`](W) writer structure"]
impl crate::Writable for MemAuxCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_3 to value 0x1320_1320"]
impl crate::Resettable for MemAuxCtrl3Spec {
    const RESET_VALUE: u32 = 0x1320_1320;
}
