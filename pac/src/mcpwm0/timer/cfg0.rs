#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `PRESCALE` reader - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PERIOD` reader - Configures the period shadow of PWM timer%s"]
pub type PeriodR = crate::FieldReader<u16>;
#[doc = "Field `PERIOD` writer - Configures the period shadow of PWM timer%s"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PERIOD_UPMETHOD` reader - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
pub type PeriodUpmethodR = crate::FieldReader;
#[doc = "Field `PERIOD_UPMETHOD` writer - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
pub type PeriodUpmethodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Configures the period shadow of PWM timer%s"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn period_upmethod(&self) -> PeriodUpmethodR {
        PeriodUpmethodR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the prescaler value of timer%s, so that the period of PT0_clk = Period of PWM_clk * (PWM_TIMER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<'_, Cfg0Spec> {
        PrescaleW::new(self, 0)
    }
    #[doc = "Bits 8:23 - Configures the period shadow of PWM timer%s"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, Cfg0Spec> {
        PeriodW::new(self, 8)
    }
    #[doc = "Bits 24:25 - Configures the update method for active register of PWM timer%s period.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn period_upmethod(&mut self) -> PeriodUpmethodW<'_, Cfg0Spec> {
        PeriodUpmethodW::new(self, 24)
    }
}
#[doc = "PWM timer0 period and update method configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0xff00"]
impl crate::Resettable for Cfg0Spec {
    const RESET_VALUE: u32 = 0xff00;
}
