#[doc = "Register `ALARMHI` reader"]
pub type R = crate::R<AlarmhiSpec>;
#[doc = "Register `ALARMHI` writer"]
pub type W = crate::W<AlarmhiSpec>;
#[doc = "Field `ALARM_HI` reader - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub type AlarmHiR = crate::FieldReader<u32>;
#[doc = "Field `ALARM_HI` writer - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub type AlarmHiW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    pub fn alarm_hi(&self) -> AlarmHiR {
        AlarmHiR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    pub fn alarm_hi(&mut self) -> AlarmHiW<'_, AlarmhiSpec> {
        AlarmHiW::new(self, 0)
    }
}
#[doc = "Timer 0 alarm value, high bits\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmhiSpec;
impl crate::RegisterSpec for AlarmhiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarmhi::R`](R) reader structure"]
impl crate::Readable for AlarmhiSpec {}
#[doc = "`write(|w| ..)` method takes [`alarmhi::W`](W) writer structure"]
impl crate::Writable for AlarmhiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALARMHI to value 0"]
impl crate::Resettable for AlarmhiSpec {}
