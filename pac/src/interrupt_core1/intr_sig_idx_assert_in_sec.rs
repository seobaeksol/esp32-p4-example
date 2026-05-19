#[doc = "Register `INTR_SIG_IDX_ASSERT_IN_SEC` reader"]
pub type R = crate::R<IntrSigIdxAssertInSecSpec>;
#[doc = "Register `INTR_SIG_IDX_ASSERT_IN_SEC` writer"]
pub type W = crate::W<IntrSigIdxAssertInSecSpec>;
#[doc = "Field `CORE1_INTR_SIG_IDX_ASSERT_IN_SEC` reader - NA"]
pub type Core1IntrSigIdxAssertInSecR = crate::FieldReader;
#[doc = "Field `CORE1_INTR_SIG_IDX_ASSERT_IN_SEC` writer - NA"]
pub type Core1IntrSigIdxAssertInSecW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core1_intr_sig_idx_assert_in_sec(&self) -> Core1IntrSigIdxAssertInSecR {
        Core1IntrSigIdxAssertInSecR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core1_intr_sig_idx_assert_in_sec(
        &mut self,
    ) -> Core1IntrSigIdxAssertInSecW<'_, IntrSigIdxAssertInSecSpec> {
        Core1IntrSigIdxAssertInSecW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sig_idx_assert_in_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sig_idx_assert_in_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSigIdxAssertInSecSpec;
impl crate::RegisterSpec for IntrSigIdxAssertInSecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sig_idx_assert_in_sec::R`](R) reader structure"]
impl crate::Readable for IntrSigIdxAssertInSecSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_sig_idx_assert_in_sec::W`](W) writer structure"]
impl crate::Writable for IntrSigIdxAssertInSecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_SIG_IDX_ASSERT_IN_SEC to value 0"]
impl crate::Resettable for IntrSigIdxAssertInSecSpec {}
