#[doc = "Register `SET_START` writer"]
pub type W = crate::W<SetStartSpec>;
#[doc = "Field `SET_START` writer - Configures whether or not to activate the DS peripheral.\\\\ 0: Invalid\\\\ 1: Activate the DS peripheral\\\\"]
pub type SetStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to activate the DS peripheral.\\\\ 0: Invalid\\\\ 1: Activate the DS peripheral\\\\"]
    #[inline(always)]
    pub fn set_start(&mut self) -> SetStartW<'_, SetStartSpec> {
        SetStartW::new(self, 0)
    }
}
#[doc = "Activates the DS module\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetStartSpec;
impl crate::RegisterSpec for SetStartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start::W`](W) writer structure"]
impl crate::Writable for SetStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_START to value 0"]
impl crate::Resettable for SetStartSpec {}
