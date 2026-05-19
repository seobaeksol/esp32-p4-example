#[doc = "Register `CLOCK_GATE` reader"]
pub type R = crate::R<ClockGateSpec>;
#[doc = "Register `CLOCK_GATE` writer"]
pub type W = crate::W<ClockGateSpec>;
#[doc = "Field `CORE0_REG_CLK_EN` reader - NA"]
pub type Core0RegClkEnR = crate::BitReader;
#[doc = "Field `CORE0_REG_CLK_EN` writer - NA"]
pub type Core0RegClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn core0_reg_clk_en(&self) -> Core0RegClkEnR {
        Core0RegClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn core0_reg_clk_en(&mut self) -> Core0RegClkEnW<'_, ClockGateSpec> {
        Core0RegClkEnW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockGateSpec;
impl crate::RegisterSpec for ClockGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_gate::R`](R) reader structure"]
impl crate::Readable for ClockGateSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_gate::W`](W) writer structure"]
impl crate::Writable for ClockGateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK_GATE to value 0x01"]
impl crate::Resettable for ClockGateSpec {
    const RESET_VALUE: u32 = 0x01;
}
