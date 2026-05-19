#[doc = "Register `TCM_RAM_PWR_CTRL0` reader"]
pub type R = crate::R<TcmRamPwrCtrl0Spec>;
#[doc = "Register `TCM_RAM_PWR_CTRL0` writer"]
pub type W = crate::W<TcmRamPwrCtrl0Spec>;
#[doc = "Field `REG_HP_TCM_CLK_FORCE_ON` reader - hp_tcm clk gatig force on"]
pub type RegHpTcmClkForceOnR = crate::BitReader;
#[doc = "Field `REG_HP_TCM_CLK_FORCE_ON` writer - hp_tcm clk gatig force on"]
pub type RegHpTcmClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - hp_tcm clk gatig force on"]
    #[inline(always)]
    pub fn reg_hp_tcm_clk_force_on(&self) -> RegHpTcmClkForceOnR {
        RegHpTcmClkForceOnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hp_tcm clk gatig force on"]
    #[inline(always)]
    pub fn reg_hp_tcm_clk_force_on(&mut self) -> RegHpTcmClkForceOnW<'_, TcmRamPwrCtrl0Spec> {
        RegHpTcmClkForceOnW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_ram_pwr_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_ram_pwr_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmRamPwrCtrl0Spec;
impl crate::RegisterSpec for TcmRamPwrCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_ram_pwr_ctrl0::R`](R) reader structure"]
impl crate::Readable for TcmRamPwrCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tcm_ram_pwr_ctrl0::W`](W) writer structure"]
impl crate::Writable for TcmRamPwrCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_RAM_PWR_CTRL0 to value 0"]
impl crate::Resettable for TcmRamPwrCtrl0Spec {}
