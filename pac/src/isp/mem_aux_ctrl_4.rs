#[doc = "Register `MEM_AUX_CTRL_4` reader"]
pub type R = crate::R<MemAuxCtrl4Spec>;
#[doc = "Register `MEM_AUX_CTRL_4` writer"]
pub type W = crate::W<MemAuxCtrl4Spec>;
#[doc = "Field `SHARP_MATRIX_UV_MEM_AUX_CTRL` reader - this field configures the mem_aux of sharp uv line buffer memory"]
pub type SharpMatrixUvMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `SHARP_MATRIX_UV_MEM_AUX_CTRL` writer - this field configures the mem_aux of sharp uv line buffer memory"]
pub type SharpMatrixUvMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of sharp uv line buffer memory"]
    #[inline(always)]
    pub fn sharp_matrix_uv_mem_aux_ctrl(&self) -> SharpMatrixUvMemAuxCtrlR {
        SharpMatrixUvMemAuxCtrlR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of sharp uv line buffer memory"]
    #[inline(always)]
    pub fn sharp_matrix_uv_mem_aux_ctrl(
        &mut self,
    ) -> SharpMatrixUvMemAuxCtrlW<'_, MemAuxCtrl4Spec> {
        SharpMatrixUvMemAuxCtrlW::new(self, 0)
    }
}
#[doc = "mem aux control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAuxCtrl4Spec;
impl crate::RegisterSpec for MemAuxCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_4::R`](R) reader structure"]
impl crate::Readable for MemAuxCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_4::W`](W) writer structure"]
impl crate::Writable for MemAuxCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_4 to value 0x1320"]
impl crate::Resettable for MemAuxCtrl4Spec {
    const RESET_VALUE: u32 = 0x1320;
}
