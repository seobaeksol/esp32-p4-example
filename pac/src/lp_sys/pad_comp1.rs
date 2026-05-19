#[doc = "Register `PAD_COMP1` reader"]
pub type R = crate::R<PadComp1Spec>;
#[doc = "Register `PAD_COMP1` writer"]
pub type W = crate::W<PadComp1Spec>;
#[doc = "Field `DREF_COMP1` reader - pad comp dref"]
pub type DrefComp1R = crate::FieldReader;
#[doc = "Field `DREF_COMP1` writer - pad comp dref"]
pub type DrefComp1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE_COMP1` reader - pad comp mode"]
pub type ModeComp1R = crate::BitReader;
#[doc = "Field `MODE_COMP1` writer - pad comp mode"]
pub type ModeComp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_COMP1` reader - pad comp xpd"]
pub type XpdComp1R = crate::BitReader;
#[doc = "Field `XPD_COMP1` writer - pad comp xpd"]
pub type XpdComp1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - pad comp dref"]
    #[inline(always)]
    pub fn dref_comp1(&self) -> DrefComp1R {
        DrefComp1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - pad comp mode"]
    #[inline(always)]
    pub fn mode_comp1(&self) -> ModeComp1R {
        ModeComp1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pad comp xpd"]
    #[inline(always)]
    pub fn xpd_comp1(&self) -> XpdComp1R {
        XpdComp1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - pad comp dref"]
    #[inline(always)]
    pub fn dref_comp1(&mut self) -> DrefComp1W<'_, PadComp1Spec> {
        DrefComp1W::new(self, 0)
    }
    #[doc = "Bit 3 - pad comp mode"]
    #[inline(always)]
    pub fn mode_comp1(&mut self) -> ModeComp1W<'_, PadComp1Spec> {
        ModeComp1W::new(self, 3)
    }
    #[doc = "Bit 4 - pad comp xpd"]
    #[inline(always)]
    pub fn xpd_comp1(&mut self) -> XpdComp1W<'_, PadComp1Spec> {
        XpdComp1W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadComp1Spec;
impl crate::RegisterSpec for PadComp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp1::R`](R) reader structure"]
impl crate::Readable for PadComp1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_comp1::W`](W) writer structure"]
impl crate::Writable for PadComp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_COMP1 to value 0"]
impl crate::Resettable for PadComp1Spec {}
