#[doc = "Register `TARGET%s_CONF` reader"]
pub type R = crate::R<TargetConfSpec>;
#[doc = "Register `TARGET%s_CONF` writer"]
pub type W = crate::W<TargetConfSpec>;
#[doc = "Field `PERIOD` reader - target0 period"]
pub type PeriodR = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - target0 period"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `PERIOD_MODE` reader - Set target0 to period mode"]
pub type PeriodModeR = crate::BitReader;
#[doc = "Field `PERIOD_MODE` writer - Set target0 to period mode"]
pub type PeriodModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT_SEL` reader - select which unit to compare"]
pub type TimerUnitSelR = crate::BitReader;
#[doc = "Field `TIMER_UNIT_SEL` writer - select which unit to compare"]
pub type TimerUnitSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - target0 period"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Set target0 to period mode"]
    #[inline(always)]
    pub fn period_mode(&self) -> PeriodModeR {
        PeriodModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    pub fn timer_unit_sel(&self) -> TimerUnitSelR {
        TimerUnitSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - target0 period"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, TargetConfSpec> {
        PeriodW::new(self, 0)
    }
    #[doc = "Bit 30 - Set target0 to period mode"]
    #[inline(always)]
    pub fn period_mode(&mut self) -> PeriodModeW<'_, TargetConfSpec> {
        PeriodModeW::new(self, 30)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    pub fn timer_unit_sel(&mut self) -> TimerUnitSelW<'_, TargetConfSpec> {
        TimerUnitSelW::new(self, 31)
    }
}
#[doc = "system timer comp%s target mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`target_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetConfSpec;
impl crate::RegisterSpec for TargetConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_conf::R`](R) reader structure"]
impl crate::Readable for TargetConfSpec {}
#[doc = "`write(|w| ..)` method takes [`target_conf::W`](W) writer structure"]
impl crate::Writable for TargetConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET%s_CONF to value 0"]
impl crate::Resettable for TargetConfSpec {}
