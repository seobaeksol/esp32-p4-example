#[doc = "Register `XTS_RELEASE` writer"]
pub type W = crate::W<XtsReleaseSpec>;
#[doc = "Field `SPI_XTS_RELEASE` writer - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
pub type SpiXtsReleaseW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
    #[inline(always)]
    pub fn spi_xts_release(&mut self) -> SpiXtsReleaseW<'_, XtsReleaseSpec> {
        SpiXtsReleaseW::new(self, 0)
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtsReleaseSpec;
impl crate::RegisterSpec for XtsReleaseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`xts_release::W`](W) writer structure"]
impl crate::Writable for XtsReleaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTS_RELEASE to value 0"]
impl crate::Resettable for XtsReleaseSpec {}
