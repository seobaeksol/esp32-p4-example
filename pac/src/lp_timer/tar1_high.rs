#[doc = "Register `TAR1_HIGH` reader"]
pub type R = crate::R<Tar1HighSpec>;
#[doc = "Register `TAR1_HIGH` writer"]
pub type W = crate::W<Tar1HighSpec>;
#[doc = "Field `MAIN_TIMER_TAR_HIGH1` reader - need_des"]
pub type MainTimerTarHigh1R = crate::FieldReader<u16>;
#[doc = "Field `MAIN_TIMER_TAR_HIGH1` writer - need_des"]
pub type MainTimerTarHigh1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MAIN_TIMER_TAR_EN1` writer - need_des"]
pub type MainTimerTarEn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_high1(&self) -> MainTimerTarHigh1R {
        MainTimerTarHigh1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_high1(&mut self) -> MainTimerTarHigh1W<'_, Tar1HighSpec> {
        MainTimerTarHigh1W::new(self, 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_en1(&mut self) -> MainTimerTarEn1W<'_, Tar1HighSpec> {
        MainTimerTarEn1W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tar1_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar1_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tar1HighSpec;
impl crate::RegisterSpec for Tar1HighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar1_high::R`](R) reader structure"]
impl crate::Readable for Tar1HighSpec {}
#[doc = "`write(|w| ..)` method takes [`tar1_high::W`](W) writer structure"]
impl crate::Writable for Tar1HighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAR1_HIGH to value 0"]
impl crate::Resettable for Tar1HighSpec {}
