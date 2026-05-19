#[doc = "Register `SET_START_MODEXP` writer"]
pub type W = crate::W<SetStartModexpSpec>;
#[doc = "Field `SET_START_MODEXP` writer - Configures whether or not to starts the modular exponentiation. \\\\ 0: No effect\\\\ 1: Start\\\\"]
pub type SetStartModexpW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to starts the modular exponentiation. \\\\ 0: No effect\\\\ 1: Start\\\\"]
    #[inline(always)]
    pub fn set_start_modexp(&mut self) -> SetStartModexpW<'_, SetStartModexpSpec> {
        SetStartModexpW::new(self, 0)
    }
}
#[doc = "Starts modular exponentiation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_start_modexp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetStartModexpSpec;
impl crate::RegisterSpec for SetStartModexpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_modexp::W`](W) writer structure"]
impl crate::Writable for SetStartModexpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_START_MODEXP to value 0"]
impl crate::Resettable for SetStartModexpSpec {}
