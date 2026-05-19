#[doc = "Register `MEM_CTRL` reader"]
pub type R = crate::R<MemCtrlSpec>;
#[doc = "Register `MEM_CTRL` writer"]
pub type W = crate::W<MemCtrlSpec>;
#[doc = "Field `CSI_BRIDGE_MEM_CLK_FORCE_ON` reader - csi bridge memory clock gating force on."]
pub type CsiBridgeMemClkForceOnR = crate::BitReader;
#[doc = "Field `CSI_BRIDGE_MEM_CLK_FORCE_ON` writer - csi bridge memory clock gating force on."]
pub type CsiBridgeMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_MEM_AUX_CTRL` reader - N/A"]
pub type CsiMemAuxCtrlR = crate::FieldReader<u16>;
#[doc = "Field `CSI_MEM_AUX_CTRL` writer - N/A"]
pub type CsiMemAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - csi bridge memory clock gating force on."]
    #[inline(always)]
    pub fn csi_bridge_mem_clk_force_on(&self) -> CsiBridgeMemClkForceOnR {
        CsiBridgeMemClkForceOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - N/A"]
    #[inline(always)]
    pub fn csi_mem_aux_ctrl(&self) -> CsiMemAuxCtrlR {
        CsiMemAuxCtrlR::new(((self.bits >> 1) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - csi bridge memory clock gating force on."]
    #[inline(always)]
    pub fn csi_bridge_mem_clk_force_on(&mut self) -> CsiBridgeMemClkForceOnW<'_, MemCtrlSpec> {
        CsiBridgeMemClkForceOnW::new(self, 0)
    }
    #[doc = "Bits 1:14 - N/A"]
    #[inline(always)]
    pub fn csi_mem_aux_ctrl(&mut self) -> CsiMemAuxCtrlW<'_, MemCtrlSpec> {
        CsiMemAuxCtrlW::new(self, 1)
    }
}
#[doc = "csi bridge buffer control.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemCtrlSpec;
impl crate::RegisterSpec for MemCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctrl::R`](R) reader structure"]
impl crate::Readable for MemCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ctrl::W`](W) writer structure"]
impl crate::Writable for MemCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CTRL to value 0x2640"]
impl crate::Resettable for MemCtrlSpec {
    const RESET_VALUE: u32 = 0x2640;
}
