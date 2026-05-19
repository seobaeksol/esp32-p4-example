#[doc = "Register `WDATAB` writer"]
pub type W = crate::W<WdatabSpec>;
#[doc = "Field `WDATAB` writer - NA"]
pub type WdatabW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDATA_END` writer - NA"]
pub type WdataEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn wdatab(&mut self) -> WdatabW<'_, WdatabSpec> {
        WdatabW::new(self, 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn wdata_end(&mut self) -> WdataEndW<'_, WdatabSpec> {
        WdataEndW::new(self, 8)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdatab::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdatabSpec;
impl crate::RegisterSpec for WdatabSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdatab::W`](W) writer structure"]
impl crate::Writable for WdatabSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDATAB to value 0"]
impl crate::Resettable for WdatabSpec {}
