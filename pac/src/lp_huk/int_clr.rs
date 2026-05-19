#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `PREP_DONE` writer - Set this bit to clear the huk_prep_done_int interrupt"]
pub type PrepDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PROC_DONE` writer - Set this bit to clear the huk_proc_done_int interrupt"]
pub type ProcDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `POST_DONE` writer - Set this bit to clear the huk_post_done_int interrupt"]
pub type PostDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the huk_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done(&mut self) -> PrepDoneW<'_, IntClrSpec> {
        PrepDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the huk_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done(&mut self) -> ProcDoneW<'_, IntClrSpec> {
        ProcDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the huk_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done(&mut self) -> PostDoneW<'_, IntClrSpec> {
        PostDoneW::new(self, 2)
    }
}
#[doc = "HUK Generator interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
