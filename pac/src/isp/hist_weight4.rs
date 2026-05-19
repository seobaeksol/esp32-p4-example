#[doc = "Register `HIST_WEIGHT4` reader"]
pub type R = crate::R<HistWeight4Spec>;
#[doc = "Register `HIST_WEIGHT4` writer"]
pub type W = crate::W<HistWeight4Spec>;
#[doc = "Field `HIST_WEIGHT_34` reader - this field configures weight of subwindow 34"]
pub type HistWeight34R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_34` writer - this field configures weight of subwindow 34"]
pub type HistWeight34W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_33` reader - this field configures weight of subwindow 33"]
pub type HistWeight33R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_33` writer - this field configures weight of subwindow 33"]
pub type HistWeight33W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_32` reader - this field configures weight of subwindow 32"]
pub type HistWeight32R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_32` writer - this field configures weight of subwindow 32"]
pub type HistWeight32W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_31` reader - this field configures weight of subwindow 31"]
pub type HistWeight31R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_31` writer - this field configures weight of subwindow 31"]
pub type HistWeight31W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 34"]
    #[inline(always)]
    pub fn hist_weight_34(&self) -> HistWeight34R {
        HistWeight34R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 33"]
    #[inline(always)]
    pub fn hist_weight_33(&self) -> HistWeight33R {
        HistWeight33R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 32"]
    #[inline(always)]
    pub fn hist_weight_32(&self) -> HistWeight32R {
        HistWeight32R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 31"]
    #[inline(always)]
    pub fn hist_weight_31(&self) -> HistWeight31R {
        HistWeight31R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 34"]
    #[inline(always)]
    pub fn hist_weight_34(&mut self) -> HistWeight34W<'_, HistWeight4Spec> {
        HistWeight34W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 33"]
    #[inline(always)]
    pub fn hist_weight_33(&mut self) -> HistWeight33W<'_, HistWeight4Spec> {
        HistWeight33W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 32"]
    #[inline(always)]
    pub fn hist_weight_32(&mut self) -> HistWeight32W<'_, HistWeight4Spec> {
        HistWeight32W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 31"]
    #[inline(always)]
    pub fn hist_weight_31(&mut self) -> HistWeight31W<'_, HistWeight4Spec> {
        HistWeight31W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight4Spec;
impl crate::RegisterSpec for HistWeight4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight4::R`](R) reader structure"]
impl crate::Readable for HistWeight4Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight4::W`](W) writer structure"]
impl crate::Writable for HistWeight4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_WEIGHT4 to value 0x0101_0101"]
impl crate::Resettable for HistWeight4Spec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
