#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `T(0-1)` writer - Set this bit to clear the TIMG_T%s_INT interrupt."]
pub type TW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WDT` writer - Set this bit to clear the TIMG_WDT_INT interrupt."]
pub type WdtW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Set this bit to clear the TIMG_T(0-1)_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `T0` field.</div>"]
    #[inline(always)]
    pub fn t(&mut self, n: u8) -> TW<'_, IntClrSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TW::new(self, n)
    }
    #[doc = "Bit 0 - Set this bit to clear the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0(&mut self) -> TW<'_, IntClrSpec> {
        TW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1(&mut self) -> TW<'_, IntClrSpec> {
        TW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WdtW<'_, IntClrSpec> {
        WdtW::new(self, 2)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
