#[doc = "Register `DUTY` reader"]
pub type R = crate::R<DutySpec>;
#[doc = "Register `DUTY` writer"]
pub type W = crate::W<DutySpec>;
#[doc = "Field `DUTY` reader - Configures the duty of signal output on channel %s."]
pub type DutyR = crate::FieldReader<u32>;
#[doc = "Field `DUTY` writer - Configures the duty of signal output on channel %s."]
pub type DutyW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Configures the duty of signal output on channel %s."]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Configures the duty of signal output on channel %s."]
    #[inline(always)]
    pub fn duty(&mut self) -> DutyW<'_, DutySpec> {
        DutyW::new(self, 0)
    }
}
#[doc = "Initial duty cycle register for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`duty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DutySpec;
impl crate::RegisterSpec for DutySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duty::R`](R) reader structure"]
impl crate::Readable for DutySpec {}
#[doc = "`write(|w| ..)` method takes [`duty::W`](W) writer structure"]
impl crate::Writable for DutySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUTY to value 0"]
impl crate::Resettable for DutySpec {}
