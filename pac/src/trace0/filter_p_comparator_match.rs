#[doc = "Register `FILTER_P_COMPARATOR_MATCH` reader"]
pub type R = crate::R<FilterPComparatorMatchSpec>;
#[doc = "Register `FILTER_P_COMPARATOR_MATCH` writer"]
pub type W = crate::W<FilterPComparatorMatchSpec>;
#[doc = "Field `P_MATCH` reader - primary comparator match value"]
pub type PMatchR = crate::FieldReader<u32>;
#[doc = "Field `P_MATCH` writer - primary comparator match value"]
pub type PMatchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - primary comparator match value"]
    #[inline(always)]
    pub fn p_match(&self) -> PMatchR {
        PMatchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - primary comparator match value"]
    #[inline(always)]
    pub fn p_match(&mut self) -> PMatchW<'_, FilterPComparatorMatchSpec> {
        PMatchW::new(self, 0)
    }
}
#[doc = "primary comparator match value\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_p_comparator_match::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_p_comparator_match::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterPComparatorMatchSpec;
impl crate::RegisterSpec for FilterPComparatorMatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_p_comparator_match::R`](R) reader structure"]
impl crate::Readable for FilterPComparatorMatchSpec {}
#[doc = "`write(|w| ..)` method takes [`filter_p_comparator_match::W`](W) writer structure"]
impl crate::Writable for FilterPComparatorMatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_P_COMPARATOR_MATCH to value 0"]
impl crate::Resettable for FilterPComparatorMatchSpec {}
