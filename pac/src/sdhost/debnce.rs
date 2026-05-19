#[doc = "Register `DEBNCE` reader"]
pub type R = crate::R<DebnceSpec>;
#[doc = "Register `DEBNCE` writer"]
pub type W = crate::W<DebnceSpec>;
#[doc = "Field `DEBOUNCE_COUNT` reader - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
pub type DebounceCountR = crate::FieldReader<u32>;
#[doc = "Field `DEBOUNCE_COUNT` writer - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
pub type DebounceCountW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
    #[inline(always)]
    pub fn debounce_count(&self) -> DebounceCountR {
        DebounceCountR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
    #[inline(always)]
    pub fn debounce_count(&mut self) -> DebounceCountW<'_, DebnceSpec> {
        DebounceCountW::new(self, 0)
    }
}
#[doc = "Debounce filter time configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`debnce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debnce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebnceSpec;
impl crate::RegisterSpec for DebnceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debnce::R`](R) reader structure"]
impl crate::Readable for DebnceSpec {}
#[doc = "`write(|w| ..)` method takes [`debnce::W`](W) writer structure"]
impl crate::Writable for DebnceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBNCE to value 0"]
impl crate::Resettable for DebnceSpec {}
