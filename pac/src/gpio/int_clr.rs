#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `COMP0_NEG` writer - analog comparator pos edge interrupt clear"]
pub type Comp0NegW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP0_POS` writer - analog comparator neg edge interrupt clear"]
pub type Comp0PosW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP0_ALL` writer - analog comparator neg or pos edge interrupt clear"]
pub type Comp0AllW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP1_NEG` writer - analog comparator pos edge interrupt clear"]
pub type Comp1NegW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP1_POS` writer - analog comparator neg edge interrupt clear"]
pub type Comp1PosW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP1_ALL` writer - analog comparator neg or pos edge interrupt clear"]
pub type Comp1AllW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BISTOK` writer - pad bistok interrupt enable"]
pub type BistokW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BISTFAIL` writer - pad bistfail interrupt enable"]
pub type BistfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp0_neg(&mut self) -> Comp0NegW<'_, IntClrSpec> {
        Comp0NegW::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    pub fn comp0_pos(&mut self) -> Comp0PosW<'_, IntClrSpec> {
        Comp0PosW::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp0_all(&mut self) -> Comp0AllW<'_, IntClrSpec> {
        Comp0AllW::new(self, 2)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp1_neg(&mut self) -> Comp1NegW<'_, IntClrSpec> {
        Comp1NegW::new(self, 3)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    pub fn comp1_pos(&mut self) -> Comp1PosW<'_, IntClrSpec> {
        Comp1PosW::new(self, 4)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp1_all(&mut self) -> Comp1AllW<'_, IntClrSpec> {
        Comp1AllW::new(self, 5)
    }
    #[doc = "Bit 6 - pad bistok interrupt enable"]
    #[inline(always)]
    pub fn bistok(&mut self) -> BistokW<'_, IntClrSpec> {
        BistokW::new(self, 6)
    }
    #[doc = "Bit 7 - pad bistfail interrupt enable"]
    #[inline(always)]
    pub fn bistfail(&mut self) -> BistfailW<'_, IntClrSpec> {
        BistfailW::new(self, 7)
    }
}
#[doc = "analog comparator interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
