#[doc = "Register `CH_ENA_AD1_SET` writer"]
pub type W = crate::W<ChEnaAd1SetSpec>;
#[doc = "Field `CH_SET(32-49)` writer - Configures whether or not to enable ch%s.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type ChSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Configures whether or not to enable ch(32-49).\\\\0: Invalid, No effect\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_SET32` field.</div>"]
    #[inline(always)]
    pub fn ch_set(&mut self, n: u8) -> ChSetW<'_, ChEnaAd1SetSpec> {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        ChSetW::new(self, n)
    }
    #[doc = "Bit 0 - Configures whether or not to enable ch32.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set32(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable ch33.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set33(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable ch34.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set34(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable ch35.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set35(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable ch36.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set36(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ch37.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set37(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable ch38.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set38(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ch39.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set39(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable ch40.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set40(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable ch41.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set41(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable ch42.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set42(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable ch43.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set43(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable ch44.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set44(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable ch45.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set45(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable ch46.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set46(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable ch47.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set47(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable ch48.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set48(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable ch49.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set49(&mut self) -> ChSetW<'_, ChEnaAd1SetSpec> {
        ChSetW::new(self, 17)
    }
}
#[doc = "Channel enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChEnaAd1SetSpec;
impl crate::RegisterSpec for ChEnaAd1SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad1_set::W`](W) writer structure"]
impl crate::Writable for ChEnaAd1SetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_ENA_AD1_SET to value 0"]
impl crate::Resettable for ChEnaAd1SetSpec {}
