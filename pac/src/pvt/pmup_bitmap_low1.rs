#[doc = "Register `PMUP_BITMAP_LOW1` reader"]
pub type R = crate::R<PmupBitmapLow1Spec>;
#[doc = "Register `PMUP_BITMAP_LOW1` writer"]
pub type W = crate::W<PmupBitmapLow1Spec>;
#[doc = "Field `PUMP_BITMAP_LOW1` reader - select valid low channel1"]
pub type PumpBitmapLow1R = crate::FieldReader<u32>;
#[doc = "Field `PUMP_BITMAP_LOW1` writer - select valid low channel1"]
pub type PumpBitmapLow1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - select valid low channel1"]
    #[inline(always)]
    pub fn pump_bitmap_low1(&self) -> PumpBitmapLow1R {
        PumpBitmapLow1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - select valid low channel1"]
    #[inline(always)]
    pub fn pump_bitmap_low1(&mut self) -> PumpBitmapLow1W<'_, PmupBitmapLow1Spec> {
        PumpBitmapLow1W::new(self, 0)
    }
}
#[doc = "select valid pvt channel\n\nYou can [`read`](crate::Reg::read) this register and get [`pmup_bitmap_low1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmup_bitmap_low1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmupBitmapLow1Spec;
impl crate::RegisterSpec for PmupBitmapLow1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_bitmap_low1::R`](R) reader structure"]
impl crate::Readable for PmupBitmapLow1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmup_bitmap_low1::W`](W) writer structure"]
impl crate::Writable for PmupBitmapLow1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUP_BITMAP_LOW1 to value 0"]
impl crate::Resettable for PmupBitmapLow1Spec {}
