#[doc = "Register `CLOCK_GATE` reader"]
pub type R = crate::R<ClockGateSpec>;
#[doc = "Register `CLOCK_GATE` writer"]
pub type W = crate::W<ClockGateSpec>;
#[doc = "Field `CLK_EN` reader - Clock enable bit of configuration registers for sigma delta modulation."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Clock enable bit of configuration registers for sigma delta modulation."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock enable bit of configuration registers for sigma delta modulation."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable bit of configuration registers for sigma delta modulation."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, ClockGateSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "Clock Gating Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CLOCK_GATE to value 0"]
impl crate::Resettable for ClockGateSpec {}
