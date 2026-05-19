#[doc = "Register `MULT_INT_ENA` reader"]
pub type R = crate::R<MultIntEnaSpec>;
#[doc = "Register `MULT_INT_ENA` writer"]
pub type W = crate::W<MultIntEnaSpec>;
#[doc = "Field `CALC_DONE` reader - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
pub type CalcDoneR = crate::BitReader;
#[doc = "Field `CALC_DONE` writer - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
pub type CalcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn calc_done(&self) -> CalcDoneR {
        CalcDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn calc_done(&mut self) -> CalcDoneW<'_, MultIntEnaSpec> {
        CalcDoneW::new(self, 0)
    }
}
#[doc = "ECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MultIntEnaSpec;
impl crate::RegisterSpec for MultIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_int_ena::R`](R) reader structure"]
impl crate::Readable for MultIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`mult_int_ena::W`](W) writer structure"]
impl crate::Writable for MultIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_INT_ENA to value 0"]
impl crate::Resettable for MultIntEnaSpec {}
