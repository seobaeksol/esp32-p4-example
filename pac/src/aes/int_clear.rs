#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<IntClearSpec>;
#[doc = "Field `INT_CLEAR` writer - Configures whether or not to clear AES interrupt. \\\\ 0: No effect \\\\ 1: Clear \\\\"]
pub type IntClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear AES interrupt. \\\\ 0: No effect \\\\ 1: Clear \\\\"]
    #[inline(always)]
    pub fn int_clear(&mut self) -> IntClearW<'_, IntClearSpec> {
        IntClearW::new(self, 0)
    }
}
#[doc = "DMA-AES interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for IntClearSpec {}
