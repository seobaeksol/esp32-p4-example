#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `COMP0_NEG` reader - analog comparator pos edge interrupt enable"]
pub type Comp0NegR = crate::BitReader;
#[doc = "Field `COMP0_NEG` writer - analog comparator pos edge interrupt enable"]
pub type Comp0NegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_POS` reader - analog comparator neg edge interrupt enable"]
pub type Comp0PosR = crate::BitReader;
#[doc = "Field `COMP0_POS` writer - analog comparator neg edge interrupt enable"]
pub type Comp0PosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_ALL` reader - analog comparator neg or pos edge interrupt enable"]
pub type Comp0AllR = crate::BitReader;
#[doc = "Field `COMP0_ALL` writer - analog comparator neg or pos edge interrupt enable"]
pub type Comp0AllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_NEG` reader - analog comparator pos edge interrupt enable"]
pub type Comp1NegR = crate::BitReader;
#[doc = "Field `COMP1_NEG` writer - analog comparator pos edge interrupt enable"]
pub type Comp1NegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_POS` reader - analog comparator neg edge interrupt enable"]
pub type Comp1PosR = crate::BitReader;
#[doc = "Field `COMP1_POS` writer - analog comparator neg edge interrupt enable"]
pub type Comp1PosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_ALL` reader - analog comparator neg or pos edge interrupt enable"]
pub type Comp1AllR = crate::BitReader;
#[doc = "Field `COMP1_ALL` writer - analog comparator neg or pos edge interrupt enable"]
pub type Comp1AllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTOK` reader - pad bistok interrupt enable"]
pub type BistokR = crate::BitReader;
#[doc = "Field `BISTOK` writer - pad bistok interrupt enable"]
pub type BistokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTFAIL` reader - pad bistfail interrupt enable"]
pub type BistfailR = crate::BitReader;
#[doc = "Field `BISTFAIL` writer - pad bistfail interrupt enable"]
pub type BistfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_neg(&self) -> Comp0NegR {
        Comp0NegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_pos(&self) -> Comp0PosR {
        Comp0PosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_all(&self) -> Comp0AllR {
        Comp0AllR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_neg(&self) -> Comp1NegR {
        Comp1NegR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_pos(&self) -> Comp1PosR {
        Comp1PosR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_all(&self) -> Comp1AllR {
        Comp1AllR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pad bistok interrupt enable"]
    #[inline(always)]
    pub fn bistok(&self) -> BistokR {
        BistokR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pad bistfail interrupt enable"]
    #[inline(always)]
    pub fn bistfail(&self) -> BistfailR {
        BistfailR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_neg(&mut self) -> Comp0NegW<'_, IntEnaSpec> {
        Comp0NegW::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_pos(&mut self) -> Comp0PosW<'_, IntEnaSpec> {
        Comp0PosW::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_all(&mut self) -> Comp0AllW<'_, IntEnaSpec> {
        Comp0AllW::new(self, 2)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_neg(&mut self) -> Comp1NegW<'_, IntEnaSpec> {
        Comp1NegW::new(self, 3)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_pos(&mut self) -> Comp1PosW<'_, IntEnaSpec> {
        Comp1PosW::new(self, 4)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_all(&mut self) -> Comp1AllW<'_, IntEnaSpec> {
        Comp1AllW::new(self, 5)
    }
    #[doc = "Bit 6 - pad bistok interrupt enable"]
    #[inline(always)]
    pub fn bistok(&mut self) -> BistokW<'_, IntEnaSpec> {
        BistokW::new(self, 6)
    }
    #[doc = "Bit 7 - pad bistfail interrupt enable"]
    #[inline(always)]
    pub fn bistfail(&mut self) -> BistfailW<'_, IntEnaSpec> {
        BistfailW::new(self, 7)
    }
}
#[doc = "analog comparator interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0xff"]
impl crate::Resettable for IntEnaSpec {
    const RESET_VALUE: u32 = 0xff;
}
