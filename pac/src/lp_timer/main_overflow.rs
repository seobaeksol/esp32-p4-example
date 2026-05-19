#[doc = "Register `MAIN_OVERFLOW` writer"]
pub type W = crate::W<MainOverflowSpec>;
#[doc = "Field `MAIN_TIMER_ALARM_LOAD` writer - need_des"]
pub type MainTimerAlarmLoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_alarm_load(&mut self) -> MainTimerAlarmLoadW<'_, MainOverflowSpec> {
        MainTimerAlarmLoadW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`main_overflow::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainOverflowSpec;
impl crate::RegisterSpec for MainOverflowSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`main_overflow::W`](W) writer structure"]
impl crate::Writable for MainOverflowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAIN_OVERFLOW to value 0"]
impl crate::Resettable for MainOverflowSpec {}
