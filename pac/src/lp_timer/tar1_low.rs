#[doc = "Register `TAR1_LOW` reader"]
pub type R = crate::R<Tar1LowSpec>;
#[doc = "Register `TAR1_LOW` writer"]
pub type W = crate::W<Tar1LowSpec>;
#[doc = "Field `MAIN_TIMER_TAR_LOW1` reader - need_des"]
pub type MainTimerTarLow1R = crate::FieldReader<u32>;
#[doc = "Field `MAIN_TIMER_TAR_LOW1` writer - need_des"]
pub type MainTimerTarLow1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_low1(&self) -> MainTimerTarLow1R {
        MainTimerTarLow1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_low1(&mut self) -> MainTimerTarLow1W<'_, Tar1LowSpec> {
        MainTimerTarLow1W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tar1_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar1_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tar1LowSpec;
impl crate::RegisterSpec for Tar1LowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar1_low::R`](R) reader structure"]
impl crate::Readable for Tar1LowSpec {}
#[doc = "`write(|w| ..)` method takes [`tar1_low::W`](W) writer structure"]
impl crate::Writable for Tar1LowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAR1_LOW to value 0"]
impl crate::Resettable for Tar1LowSpec {}
