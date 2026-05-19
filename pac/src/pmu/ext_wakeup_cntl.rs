#[doc = "Register `EXT_WAKEUP_CNTL` reader"]
pub type R = crate::R<ExtWakeupCntlSpec>;
#[doc = "Register `EXT_WAKEUP_CNTL` writer"]
pub type W = crate::W<ExtWakeupCntlSpec>;
#[doc = "Field `EXT_WAKEUP_STATUS_CLR` reader - need_des"]
pub type ExtWakeupStatusClrR = crate::BitReader;
#[doc = "Field `EXT_WAKEUP_STATUS_CLR` writer - need_des"]
pub type ExtWakeupStatusClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_WAKEUP_FILTER` reader - need_des"]
pub type ExtWakeupFilterR = crate::BitReader;
#[doc = "Field `EXT_WAKEUP_FILTER` writer - need_des"]
pub type ExtWakeupFilterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_status_clr(&self) -> ExtWakeupStatusClrR {
        ExtWakeupStatusClrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_filter(&self) -> ExtWakeupFilterR {
        ExtWakeupFilterR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_status_clr(&mut self) -> ExtWakeupStatusClrW<'_, ExtWakeupCntlSpec> {
        ExtWakeupStatusClrW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_filter(&mut self) -> ExtWakeupFilterW<'_, ExtWakeupCntlSpec> {
        ExtWakeupFilterW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtWakeupCntlSpec;
impl crate::RegisterSpec for ExtWakeupCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_cntl::R`](R) reader structure"]
impl crate::Readable for ExtWakeupCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_cntl::W`](W) writer structure"]
impl crate::Writable for ExtWakeupCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CNTL to value 0"]
impl crate::Resettable for ExtWakeupCntlSpec {}
