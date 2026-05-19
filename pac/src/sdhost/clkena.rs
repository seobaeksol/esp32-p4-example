#[doc = "Register `CLKENA` reader"]
pub type R = crate::R<ClkenaSpec>;
#[doc = "Register `CLKENA` writer"]
pub type W = crate::W<ClkenaSpec>;
#[doc = "Field `CCLK_ENABLE` reader - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
pub type CclkEnableR = crate::FieldReader;
#[doc = "Field `CCLK_ENABLE` writer - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
pub type CclkEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_ENABLE` reader - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
pub type LpEnableR = crate::FieldReader;
#[doc = "Field `LP_ENABLE` writer - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
pub type LpEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
    #[inline(always)]
    pub fn cclk_enable(&self) -> CclkEnableR {
        CclkEnableR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
    #[inline(always)]
    pub fn lp_enable(&self) -> LpEnableR {
        LpEnableR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
    #[inline(always)]
    pub fn cclk_enable(&mut self) -> CclkEnableW<'_, ClkenaSpec> {
        CclkEnableW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
    #[inline(always)]
    pub fn lp_enable(&mut self) -> LpEnableW<'_, ClkenaSpec> {
        LpEnableW::new(self, 16)
    }
}
#[doc = "Clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkenaSpec;
impl crate::RegisterSpec for ClkenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkena::R`](R) reader structure"]
impl crate::Readable for ClkenaSpec {}
#[doc = "`write(|w| ..)` method takes [`clkena::W`](W) writer structure"]
impl crate::Writable for ClkenaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKENA to value 0"]
impl crate::Resettable for ClkenaSpec {}
