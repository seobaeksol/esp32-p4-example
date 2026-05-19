#[doc = "Register `HIST_SEG3` reader"]
pub type R = crate::R<HistSeg3Spec>;
#[doc = "Register `HIST_SEG3` writer"]
pub type W = crate::W<HistSeg3Spec>;
#[doc = "Field `HIST_SEG_14_15` reader - this field configures threshold of histogram bin 14 and bin 15"]
pub type HistSeg14_15R = crate::FieldReader;
#[doc = "Field `HIST_SEG_14_15` writer - this field configures threshold of histogram bin 14 and bin 15"]
pub type HistSeg14_15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_13_14` reader - this field configures threshold of histogram bin 13 and bin 14"]
pub type HistSeg13_14R = crate::FieldReader;
#[doc = "Field `HIST_SEG_13_14` writer - this field configures threshold of histogram bin 13 and bin 14"]
pub type HistSeg13_14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_12_13` reader - this field configures threshold of histogram bin 12 and bin 13"]
pub type HistSeg12_13R = crate::FieldReader;
#[doc = "Field `HIST_SEG_12_13` writer - this field configures threshold of histogram bin 12 and bin 13"]
pub type HistSeg12_13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 14 and bin 15"]
    #[inline(always)]
    pub fn hist_seg_14_15(&self) -> HistSeg14_15R {
        HistSeg14_15R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 13 and bin 14"]
    #[inline(always)]
    pub fn hist_seg_13_14(&self) -> HistSeg13_14R {
        HistSeg13_14R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 12 and bin 13"]
    #[inline(always)]
    pub fn hist_seg_12_13(&self) -> HistSeg12_13R {
        HistSeg12_13R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 14 and bin 15"]
    #[inline(always)]
    pub fn hist_seg_14_15(&mut self) -> HistSeg14_15W<'_, HistSeg3Spec> {
        HistSeg14_15W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 13 and bin 14"]
    #[inline(always)]
    pub fn hist_seg_13_14(&mut self) -> HistSeg13_14W<'_, HistSeg3Spec> {
        HistSeg13_14W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 12 and bin 13"]
    #[inline(always)]
    pub fn hist_seg_12_13(&mut self) -> HistSeg12_13W<'_, HistSeg3Spec> {
        HistSeg12_13W::new(self, 16)
    }
}
#[doc = "histogram bin control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_seg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_seg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistSeg3Spec;
impl crate::RegisterSpec for HistSeg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_seg3::R`](R) reader structure"]
impl crate::Readable for HistSeg3Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_seg3::W`](W) writer structure"]
impl crate::Writable for HistSeg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_SEG3 to value 0x00d0_e0f0"]
impl crate::Resettable for HistSeg3Spec {
    const RESET_VALUE: u32 = 0x00d0_e0f0;
}
