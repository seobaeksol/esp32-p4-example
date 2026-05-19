#[doc = "Register `LP_AONCLKRST_HPCPU_RESET_CTRL1` reader"]
pub type R = crate::R<LpAonclkrstHpcpuResetCtrl1Spec>;
#[doc = "Register `LP_AONCLKRST_HPCPU_RESET_CTRL1` writer"]
pub type W = crate::W<LpAonclkrstHpcpuResetCtrl1Spec>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_SW_STALL_CODE` reader - HP core0 software stall when set to 8'h86"]
pub type LpAonclkrstHpcore0SwStallCodeR = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_SW_STALL_CODE` writer - HP core0 software stall when set to 8'h86"]
pub type LpAonclkrstHpcore0SwStallCodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_SW_STALL_CODE` reader - HP core1 software stall when set to 8'h86"]
pub type LpAonclkrstHpcore1SwStallCodeR = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_SW_STALL_CODE` writer - HP core1 software stall when set to 8'h86"]
pub type LpAonclkrstHpcore1SwStallCodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - HP core0 software stall when set to 8'h86"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_stall_code(&self) -> LpAonclkrstHpcore0SwStallCodeR {
        LpAonclkrstHpcore0SwStallCodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - HP core1 software stall when set to 8'h86"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_sw_stall_code(&self) -> LpAonclkrstHpcore1SwStallCodeR {
        LpAonclkrstHpcore1SwStallCodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - HP core0 software stall when set to 8'h86"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_stall_code(
        &mut self,
    ) -> LpAonclkrstHpcore0SwStallCodeW<'_, LpAonclkrstHpcpuResetCtrl1Spec> {
        LpAonclkrstHpcore0SwStallCodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - HP core1 software stall when set to 8'h86"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_sw_stall_code(
        &mut self,
    ) -> LpAonclkrstHpcore1SwStallCodeW<'_, LpAonclkrstHpcpuResetCtrl1Spec> {
        LpAonclkrstHpcore1SwStallCodeW::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcpu_reset_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcpu_reset_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstHpcpuResetCtrl1Spec;
impl crate::RegisterSpec for LpAonclkrstHpcpuResetCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hpcpu_reset_ctrl1::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstHpcpuResetCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hpcpu_reset_ctrl1::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstHpcpuResetCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HPCPU_RESET_CTRL1 to value 0"]
impl crate::Resettable for LpAonclkrstHpcpuResetCtrl1Spec {}
