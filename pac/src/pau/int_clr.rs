#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `DONE` writer - backup done flag"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` writer - error flag"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - backup done flag"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntClrSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - error flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntClrSpec> {
        ErrorW::new(self, 1)
    }
}
#[doc = "Read only register for error and done\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
