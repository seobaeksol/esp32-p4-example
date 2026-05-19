#[doc = "Register `HIST_WEIGHT0` reader"]
pub type R = crate::R<HistWeight0Spec>;
#[doc = "Register `HIST_WEIGHT0` writer"]
pub type W = crate::W<HistWeight0Spec>;
#[doc = "Field `HIST_WEIGHT_03` reader - this field configures weight of subwindow 03"]
pub type HistWeight03R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_03` writer - this field configures weight of subwindow 03"]
pub type HistWeight03W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_02` reader - this field configures weight of subwindow 02"]
pub type HistWeight02R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_02` writer - this field configures weight of subwindow 02"]
pub type HistWeight02W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_01` reader - this field configures weight of subwindow 01"]
pub type HistWeight01R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_01` writer - this field configures weight of subwindow 01"]
pub type HistWeight01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_00` reader - this field configures weight of subwindow 00 and sum of all weight should be 256"]
pub type HistWeight00R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_00` writer - this field configures weight of subwindow 00 and sum of all weight should be 256"]
pub type HistWeight00W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 03"]
    #[inline(always)]
    pub fn hist_weight_03(&self) -> HistWeight03R {
        HistWeight03R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 02"]
    #[inline(always)]
    pub fn hist_weight_02(&self) -> HistWeight02R {
        HistWeight02R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 01"]
    #[inline(always)]
    pub fn hist_weight_01(&self) -> HistWeight01R {
        HistWeight01R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 00 and sum of all weight should be 256"]
    #[inline(always)]
    pub fn hist_weight_00(&self) -> HistWeight00R {
        HistWeight00R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 03"]
    #[inline(always)]
    pub fn hist_weight_03(&mut self) -> HistWeight03W<'_, HistWeight0Spec> {
        HistWeight03W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 02"]
    #[inline(always)]
    pub fn hist_weight_02(&mut self) -> HistWeight02W<'_, HistWeight0Spec> {
        HistWeight02W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 01"]
    #[inline(always)]
    pub fn hist_weight_01(&mut self) -> HistWeight01W<'_, HistWeight0Spec> {
        HistWeight01W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 00 and sum of all weight should be 256"]
    #[inline(always)]
    pub fn hist_weight_00(&mut self) -> HistWeight00W<'_, HistWeight0Spec> {
        HistWeight00W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight0Spec;
impl crate::RegisterSpec for HistWeight0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight0::R`](R) reader structure"]
impl crate::Readable for HistWeight0Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight0::W`](W) writer structure"]
impl crate::Writable for HistWeight0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_WEIGHT0 to value 0x0101_0101"]
impl crate::Resettable for HistWeight0Spec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
