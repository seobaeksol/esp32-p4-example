#[doc = "Register `MEM_AUX_CTRL_2` reader"]
pub type R = crate::R<MemAuxCtrl2Spec>;
#[doc = "Register `MEM_AUX_CTRL_2` writer"]
pub type W = crate::W<MemAuxCtrl2Spec>;
#[doc = "Field `BF_MATRIX_MEM_AUX_CTRL` reader - this field configures the mem_aux of bf line buffer memory"]
pub type BfMatrixMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `BF_MATRIX_MEM_AUX_CTRL` writer - this field configures the mem_aux of bf line buffer memory"]
pub type BfMatrixMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DPC_MATRIX_MEM_AUX_CTRL` reader - this field configures the mem_aux of dpc line buffer memory"]
pub type DpcMatrixMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `DPC_MATRIX_MEM_AUX_CTRL` writer - this field configures the mem_aux of dpc line buffer memory"]
pub type DpcMatrixMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of bf line buffer memory"]
    #[inline(always)]
    pub fn bf_matrix_mem_aux_ctrl(&self) -> BfMatrixMemAuxCtrlR {
        BfMatrixMemAuxCtrlR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of dpc line buffer memory"]
    #[inline(always)]
    pub fn dpc_matrix_mem_aux_ctrl(&self) -> DpcMatrixMemAuxCtrlR {
        DpcMatrixMemAuxCtrlR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of bf line buffer memory"]
    #[inline(always)]
    pub fn bf_matrix_mem_aux_ctrl(&mut self) -> BfMatrixMemAuxCtrlW<'_, MemAuxCtrl2Spec> {
        BfMatrixMemAuxCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of dpc line buffer memory"]
    #[inline(always)]
    pub fn dpc_matrix_mem_aux_ctrl(&mut self) -> DpcMatrixMemAuxCtrlW<'_, MemAuxCtrl2Spec> {
        DpcMatrixMemAuxCtrlW::new(self, 16)
    }
}
#[doc = "mem aux control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAuxCtrl2Spec;
impl crate::RegisterSpec for MemAuxCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_2::R`](R) reader structure"]
impl crate::Readable for MemAuxCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_2::W`](W) writer structure"]
impl crate::Writable for MemAuxCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_2 to value 0x1320_1320"]
impl crate::Resettable for MemAuxCtrl2Spec {
    const RESET_VALUE: u32 = 0x1320_1320;
}
