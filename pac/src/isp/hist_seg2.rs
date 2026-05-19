#[doc = "Register `HIST_SEG2` reader"]
pub type R = crate::R<HistSeg2Spec>;
#[doc = "Register `HIST_SEG2` writer"]
pub type W = crate::W<HistSeg2Spec>;
#[doc = "Field `HIST_SEG_11_12` reader - this field configures threshold of histogram bin 11 and bin 12"]
pub type HistSeg11_12R = crate::FieldReader;
#[doc = "Field `HIST_SEG_11_12` writer - this field configures threshold of histogram bin 11 and bin 12"]
pub type HistSeg11_12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_10_11` reader - this field configures threshold of histogram bin 10 and bin 11"]
pub type HistSeg10_11R = crate::FieldReader;
#[doc = "Field `HIST_SEG_10_11` writer - this field configures threshold of histogram bin 10 and bin 11"]
pub type HistSeg10_11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_9_10` reader - this field configures threshold of histogram bin 9 and bin 10"]
pub type HistSeg9_10R = crate::FieldReader;
#[doc = "Field `HIST_SEG_9_10` writer - this field configures threshold of histogram bin 9 and bin 10"]
pub type HistSeg9_10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_8_9` reader - this field configures threshold of histogram bin 8 and bin 9"]
pub type HistSeg8_9R = crate::FieldReader;
#[doc = "Field `HIST_SEG_8_9` writer - this field configures threshold of histogram bin 8 and bin 9"]
pub type HistSeg8_9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 11 and bin 12"]
    #[inline(always)]
    pub fn hist_seg_11_12(&self) -> HistSeg11_12R {
        HistSeg11_12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 10 and bin 11"]
    #[inline(always)]
    pub fn hist_seg_10_11(&self) -> HistSeg10_11R {
        HistSeg10_11R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 9 and bin 10"]
    #[inline(always)]
    pub fn hist_seg_9_10(&self) -> HistSeg9_10R {
        HistSeg9_10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 8 and bin 9"]
    #[inline(always)]
    pub fn hist_seg_8_9(&self) -> HistSeg8_9R {
        HistSeg8_9R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 11 and bin 12"]
    #[inline(always)]
    pub fn hist_seg_11_12(&mut self) -> HistSeg11_12W<'_, HistSeg2Spec> {
        HistSeg11_12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 10 and bin 11"]
    #[inline(always)]
    pub fn hist_seg_10_11(&mut self) -> HistSeg10_11W<'_, HistSeg2Spec> {
        HistSeg10_11W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 9 and bin 10"]
    #[inline(always)]
    pub fn hist_seg_9_10(&mut self) -> HistSeg9_10W<'_, HistSeg2Spec> {
        HistSeg9_10W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 8 and bin 9"]
    #[inline(always)]
    pub fn hist_seg_8_9(&mut self) -> HistSeg8_9W<'_, HistSeg2Spec> {
        HistSeg8_9W::new(self, 24)
    }
}
#[doc = "histogram bin control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_seg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_seg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistSeg2Spec;
impl crate::RegisterSpec for HistSeg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_seg2::R`](R) reader structure"]
impl crate::Readable for HistSeg2Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_seg2::W`](W) writer structure"]
impl crate::Writable for HistSeg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_SEG2 to value 0x90a0_b0c0"]
impl crate::Resettable for HistSeg2Spec {
    const RESET_VALUE: u32 = 0x90a0_b0c0;
}
