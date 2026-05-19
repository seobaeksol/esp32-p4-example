#[doc = "Register `CLK_CFG` reader"]
pub type R = crate::R<ClkCfgSpec>;
#[doc = "Register `CLK_CFG` writer"]
pub type W = crate::W<ClkCfgSpec>;
#[doc = "Field `CLK_PRESCALE` reader - Configures the prescaler value of clock, so that the period of PWM_clk = 6.25ns * (PWM_CLK_PRESCALE + 1)."]
pub type ClkPrescaleR = crate::FieldReader;
#[doc = "Field `CLK_PRESCALE` writer - Configures the prescaler value of clock, so that the period of PWM_clk = 6.25ns * (PWM_CLK_PRESCALE + 1)."]
pub type ClkPrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the prescaler value of clock, so that the period of PWM_clk = 6.25ns * (PWM_CLK_PRESCALE + 1)."]
    #[inline(always)]
    pub fn clk_prescale(&self) -> ClkPrescaleR {
        ClkPrescaleR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the prescaler value of clock, so that the period of PWM_clk = 6.25ns * (PWM_CLK_PRESCALE + 1)."]
    #[inline(always)]
    pub fn clk_prescale(&mut self) -> ClkPrescaleW<'_, ClkCfgSpec> {
        ClkPrescaleW::new(self, 0)
    }
}
#[doc = "PWM clock prescaler register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCfgSpec;
impl crate::RegisterSpec for ClkCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cfg::R`](R) reader structure"]
impl crate::Readable for ClkCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_cfg::W`](W) writer structure"]
impl crate::Writable for ClkCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for ClkCfgSpec {}
