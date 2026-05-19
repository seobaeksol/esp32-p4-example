#[doc = "Register `L2_MEM_RAM_PWR_CTRL0` reader"]
pub type R = crate::R<L2MemRamPwrCtrl0Spec>;
#[doc = "Register `L2_MEM_RAM_PWR_CTRL0` writer"]
pub type W = crate::W<L2MemRamPwrCtrl0Spec>;
#[doc = "Field `REG_L2_MEM_CLK_FORCE_ON` reader - l2ram clk_gating force on"]
pub type RegL2MemClkForceOnR = crate::BitReader;
#[doc = "Field `REG_L2_MEM_CLK_FORCE_ON` writer - l2ram clk_gating force on"]
pub type RegL2MemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - l2ram clk_gating force on"]
    #[inline(always)]
    pub fn reg_l2_mem_clk_force_on(&self) -> RegL2MemClkForceOnR {
        RegL2MemClkForceOnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - l2ram clk_gating force on"]
    #[inline(always)]
    pub fn reg_l2_mem_clk_force_on(&mut self) -> RegL2MemClkForceOnW<'_, L2MemRamPwrCtrl0Spec> {
        RegL2MemClkForceOnW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_ram_pwr_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_ram_pwr_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2MemRamPwrCtrl0Spec;
impl crate::RegisterSpec for L2MemRamPwrCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_ram_pwr_ctrl0::R`](R) reader structure"]
impl crate::Readable for L2MemRamPwrCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_ram_pwr_ctrl0::W`](W) writer structure"]
impl crate::Writable for L2MemRamPwrCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_RAM_PWR_CTRL0 to value 0"]
impl crate::Resettable for L2MemRamPwrCtrl0Spec {}
