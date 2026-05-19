#[doc = "Register `SET_START_MULT` writer"]
pub type W = crate::W<SetStartMultSpec>;
#[doc = "Field `SET_START_MULT` writer - Configures whether or not to start the multiplication.\\\\ 0: No effect\\\\ 1: Start\\\\"]
pub type SetStartMultW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to start the multiplication.\\\\ 0: No effect\\\\ 1: Start\\\\"]
    #[inline(always)]
    pub fn set_start_mult(&mut self) -> SetStartMultW<'_, SetStartMultSpec> {
        SetStartMultW::new(self, 0)
    }
}
#[doc = "Starts multiplication\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_start_mult::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetStartMultSpec;
impl crate::RegisterSpec for SetStartMultSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_mult::W`](W) writer structure"]
impl crate::Writable for SetStartMultSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_START_MULT to value 0"]
impl crate::Resettable for SetStartMultSpec {}
