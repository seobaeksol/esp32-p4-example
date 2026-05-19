#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `COMP0_NEG` reader - analog comparator pos edge interrupt raw"]
pub type Comp0NegR = crate::BitReader;
#[doc = "Field `COMP0_NEG` writer - analog comparator pos edge interrupt raw"]
pub type Comp0NegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_POS` reader - analog comparator neg edge interrupt raw"]
pub type Comp0PosR = crate::BitReader;
#[doc = "Field `COMP0_POS` writer - analog comparator neg edge interrupt raw"]
pub type Comp0PosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_ALL` reader - analog comparator neg or pos edge interrupt raw"]
pub type Comp0AllR = crate::BitReader;
#[doc = "Field `COMP0_ALL` writer - analog comparator neg or pos edge interrupt raw"]
pub type Comp0AllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_NEG` reader - analog comparator pos edge interrupt raw"]
pub type Comp1NegR = crate::BitReader;
#[doc = "Field `COMP1_NEG` writer - analog comparator pos edge interrupt raw"]
pub type Comp1NegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_POS` reader - analog comparator neg edge interrupt raw"]
pub type Comp1PosR = crate::BitReader;
#[doc = "Field `COMP1_POS` writer - analog comparator neg edge interrupt raw"]
pub type Comp1PosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_ALL` reader - analog comparator neg or pos edge interrupt raw"]
pub type Comp1AllR = crate::BitReader;
#[doc = "Field `COMP1_ALL` writer - analog comparator neg or pos edge interrupt raw"]
pub type Comp1AllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTOK` reader - pad bistok interrupt raw"]
pub type BistokR = crate::BitReader;
#[doc = "Field `BISTOK` writer - pad bistok interrupt raw"]
pub type BistokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTFAIL` reader - pad bistfail interrupt raw"]
pub type BistfailR = crate::BitReader;
#[doc = "Field `BISTFAIL` writer - pad bistfail interrupt raw"]
pub type BistfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_neg(&self) -> Comp0NegR {
        Comp0NegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_pos(&self) -> Comp0PosR {
        Comp0PosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_all(&self) -> Comp0AllR {
        Comp0AllR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_neg(&self) -> Comp1NegR {
        Comp1NegR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_pos(&self) -> Comp1PosR {
        Comp1PosR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_all(&self) -> Comp1AllR {
        Comp1AllR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pad bistok interrupt raw"]
    #[inline(always)]
    pub fn bistok(&self) -> BistokR {
        BistokR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pad bistfail interrupt raw"]
    #[inline(always)]
    pub fn bistfail(&self) -> BistfailR {
        BistfailR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_neg(&mut self) -> Comp0NegW<'_, IntRawSpec> {
        Comp0NegW::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_pos(&mut self) -> Comp0PosW<'_, IntRawSpec> {
        Comp0PosW::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_all(&mut self) -> Comp0AllW<'_, IntRawSpec> {
        Comp0AllW::new(self, 2)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_neg(&mut self) -> Comp1NegW<'_, IntRawSpec> {
        Comp1NegW::new(self, 3)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_pos(&mut self) -> Comp1PosW<'_, IntRawSpec> {
        Comp1PosW::new(self, 4)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_all(&mut self) -> Comp1AllW<'_, IntRawSpec> {
        Comp1AllW::new(self, 5)
    }
    #[doc = "Bit 6 - pad bistok interrupt raw"]
    #[inline(always)]
    pub fn bistok(&mut self) -> BistokW<'_, IntRawSpec> {
        BistokW::new(self, 6)
    }
    #[doc = "Bit 7 - pad bistfail interrupt raw"]
    #[inline(always)]
    pub fn bistfail(&mut self) -> BistfailW<'_, IntRawSpec> {
        BistfailW::new(self, 7)
    }
}
#[doc = "analog comparator interrupt raw\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
