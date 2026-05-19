#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `COCPU_TSENS_WAKE` reader - Tsens wakeup interrupt raw."]
pub type CocpuTsensWakeR = crate::BitReader;
#[doc = "Field `COCPU_TSENS_WAKE` writer - Tsens wakeup interrupt raw."]
pub type CocpuTsensWakeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tsens wakeup interrupt raw."]
    #[inline(always)]
    pub fn cocpu_tsens_wake(&self) -> CocpuTsensWakeR {
        CocpuTsensWakeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tsens wakeup interrupt raw."]
    #[inline(always)]
    pub fn cocpu_tsens_wake(&mut self) -> CocpuTsensWakeW<'_, IntRawSpec> {
        CocpuTsensWakeW::new(self, 0)
    }
}
#[doc = "Tsens interrupt raw registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
