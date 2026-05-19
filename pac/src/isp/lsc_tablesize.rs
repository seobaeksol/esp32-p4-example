#[doc = "Register `LSC_TABLESIZE` reader"]
pub type R = crate::R<LscTablesizeSpec>;
#[doc = "Register `LSC_TABLESIZE` writer"]
pub type W = crate::W<LscTablesizeSpec>;
#[doc = "Field `LSC_XTABLESIZE` reader - this field configures lsc table size in x-direction"]
pub type LscXtablesizeR = crate::FieldReader;
#[doc = "Field `LSC_XTABLESIZE` writer - this field configures lsc table size in x-direction"]
pub type LscXtablesizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this field configures lsc table size in x-direction"]
    #[inline(always)]
    pub fn lsc_xtablesize(&self) -> LscXtablesizeR {
        LscXtablesizeR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this field configures lsc table size in x-direction"]
    #[inline(always)]
    pub fn lsc_xtablesize(&mut self) -> LscXtablesizeW<'_, LscTablesizeSpec> {
        LscXtablesizeW::new(self, 0)
    }
}
#[doc = "LSC point in x-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`lsc_tablesize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsc_tablesize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscTablesizeSpec;
impl crate::RegisterSpec for LscTablesizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_tablesize::R`](R) reader structure"]
impl crate::Readable for LscTablesizeSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_tablesize::W`](W) writer structure"]
impl crate::Writable for LscTablesizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LSC_TABLESIZE to value 0x1f"]
impl crate::Resettable for LscTablesizeSpec {
    const RESET_VALUE: u32 = 0x1f;
}
