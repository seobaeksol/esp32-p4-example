#[doc = "Register `MULT_INT_CLR` writer"]
pub type W = crate::W<MultIntClrSpec>;
#[doc = "Field `CALC_DONE` writer - Write 1 to clear the ECC_CALC_DONE_INT interrupt."]
pub type CalcDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn calc_done(&mut self) -> CalcDoneW<'_, MultIntClrSpec> {
        CalcDoneW::new(self, 0)
    }
}
#[doc = "ECC interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MultIntClrSpec;
impl crate::RegisterSpec for MultIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mult_int_clr::W`](W) writer structure"]
impl crate::Writable for MultIntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets MULT_INT_CLR to value 0"]
impl crate::Resettable for MultIntClrSpec {}
