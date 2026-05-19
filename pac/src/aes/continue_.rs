#[doc = "Register `CONTINUE` writer"]
pub type W = crate::W<ContinueSpec>;
#[doc = "Field `CONTINUE` writer - Set this bit to continue GCM operation."]
pub type ContinueW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to continue GCM operation."]
    #[inline(always)]
    pub fn continue_(&mut self) -> ContinueW<'_, ContinueSpec> {
        ContinueW::new(self, 0)
    }
}
#[doc = "AES continue register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`continue_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ContinueSpec;
impl crate::RegisterSpec for ContinueSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`continue_::W`](W) writer structure"]
impl crate::Writable for ContinueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for ContinueSpec {}
