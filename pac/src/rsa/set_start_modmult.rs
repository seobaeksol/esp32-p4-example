#[doc = "Register `SET_START_MODMULT` writer"]
pub type W = crate::W<SetStartModmultSpec>;
#[doc = "Field `SET_START_MODMULT` writer - Configures whether or not to start the modular multiplication.\\\\ 0: No effect\\\\ 1: Start\\\\"]
pub type SetStartModmultW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to start the modular multiplication.\\\\ 0: No effect\\\\ 1: Start\\\\"]
    #[inline(always)]
    pub fn set_start_modmult(&mut self) -> SetStartModmultW<'_, SetStartModmultSpec> {
        SetStartModmultW::new(self, 0)
    }
}
#[doc = "Starts modular multiplication\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_start_modmult::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetStartModmultSpec;
impl crate::RegisterSpec for SetStartModmultSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_modmult::W`](W) writer structure"]
impl crate::Writable for SetStartModmultSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_START_MODMULT to value 0"]
impl crate::Resettable for SetStartModmultSpec {}
