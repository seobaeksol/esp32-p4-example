#[doc = "Register `MEM_AUX_CTRL_0` reader"]
pub type R = crate::R<MemAuxCtrl0Spec>;
#[doc = "Register `MEM_AUX_CTRL_0` writer"]
pub type W = crate::W<MemAuxCtrl0Spec>;
#[doc = "Field `HEADER_MEM_AUX_CTRL` reader - this field configures the mem_aux of isp input buffer memory"]
pub type HeaderMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `HEADER_MEM_AUX_CTRL` writer - this field configures the mem_aux of isp input buffer memory"]
pub type HeaderMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DPC_LUT_MEM_AUX_CTRL` reader - this field represents this field configures the mem_aux of dpc lut memory"]
pub type DpcLutMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `DPC_LUT_MEM_AUX_CTRL` writer - this field represents this field configures the mem_aux of dpc lut memory"]
pub type DpcLutMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of isp input buffer memory"]
    #[inline(always)]
    pub fn header_mem_aux_ctrl(&self) -> HeaderMemAuxCtrlR {
        HeaderMemAuxCtrlR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - this field represents this field configures the mem_aux of dpc lut memory"]
    #[inline(always)]
    pub fn dpc_lut_mem_aux_ctrl(&self) -> DpcLutMemAuxCtrlR {
        DpcLutMemAuxCtrlR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of isp input buffer memory"]
    #[inline(always)]
    pub fn header_mem_aux_ctrl(&mut self) -> HeaderMemAuxCtrlW<'_, MemAuxCtrl0Spec> {
        HeaderMemAuxCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:29 - this field represents this field configures the mem_aux of dpc lut memory"]
    #[inline(always)]
    pub fn dpc_lut_mem_aux_ctrl(&mut self) -> DpcLutMemAuxCtrlW<'_, MemAuxCtrl0Spec> {
        DpcLutMemAuxCtrlW::new(self, 16)
    }
}
#[doc = "mem aux control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAuxCtrl0Spec;
impl crate::RegisterSpec for MemAuxCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_0::R`](R) reader structure"]
impl crate::Readable for MemAuxCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_0::W`](W) writer structure"]
impl crate::Writable for MemAuxCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_0 to value 0x1320_1320"]
impl crate::Resettable for MemAuxCtrl0Spec {
    const RESET_VALUE: u32 = 0x1320_1320;
}
