#[doc = "Register `HP_SLEEP_LP_DCDC_RESERVE` writer"]
pub type W = crate::W<HpSleepLpDcdcReserveSpec>;
#[doc = "Field `PMU_HP_SLEEP_LP_DCDC_RESERVE` writer - need_des"]
pub type PmuHpSleepLpDcdcReserveW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn pmu_hp_sleep_lp_dcdc_reserve(
        &mut self,
    ) -> PmuHpSleepLpDcdcReserveW<'_, HpSleepLpDcdcReserveSpec> {
        PmuHpSleepLpDcdcReserveW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_dcdc_reserve::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpSleepLpDcdcReserveSpec;
impl crate::RegisterSpec for HpSleepLpDcdcReserveSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_lp_dcdc_reserve::W`](W) writer structure"]
impl crate::Writable for HpSleepLpDcdcReserveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_DCDC_RESERVE to value 0"]
impl crate::Resettable for HpSleepLpDcdcReserveSpec {}
