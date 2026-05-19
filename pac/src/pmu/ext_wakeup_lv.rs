#[doc = "Register `EXT_WAKEUP_LV` reader"]
pub type R = crate::R<ExtWakeupLvSpec>;
#[doc = "Register `EXT_WAKEUP_LV` writer"]
pub type W = crate::W<ExtWakeupLvSpec>;
#[doc = "Field `EXT_WAKEUP_LV` reader - need_des"]
pub type ExtWakeupLvR = crate::FieldReader<u32>;
#[doc = "Field `EXT_WAKEUP_LV` writer - need_des"]
pub type ExtWakeupLvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_lv(&self) -> ExtWakeupLvR {
        ExtWakeupLvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_lv(&mut self) -> ExtWakeupLvW<'_, ExtWakeupLvSpec> {
        ExtWakeupLvW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_lv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_lv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtWakeupLvSpec;
impl crate::RegisterSpec for ExtWakeupLvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_lv::R`](R) reader structure"]
impl crate::Readable for ExtWakeupLvSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_lv::W`](W) writer structure"]
impl crate::Writable for ExtWakeupLvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_WAKEUP_LV to value 0"]
impl crate::Resettable for ExtWakeupLvSpec {}
