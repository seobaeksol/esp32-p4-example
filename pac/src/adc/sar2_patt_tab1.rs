#[doc = "Register `SAR2_PATT_TAB1` reader"]
pub type R = crate::R<Sar2PattTab1Spec>;
#[doc = "Register `SAR2_PATT_TAB1` writer"]
pub type W = crate::W<Sar2PattTab1Spec>;
#[doc = "Field `SAR2_PATT_TAB1` reader - item 0 ~ 3 for pattern table 2 (each item one byte)"]
pub type Sar2PattTab1R = crate::FieldReader<u32>;
#[doc = "Field `SAR2_PATT_TAB1` writer - item 0 ~ 3 for pattern table 2 (each item one byte)"]
pub type Sar2PattTab1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - item 0 ~ 3 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab1(&self) -> Sar2PattTab1R {
        Sar2PattTab1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - item 0 ~ 3 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab1(&mut self) -> Sar2PattTab1W<'_, Sar2PattTab1Spec> {
        Sar2PattTab1W::new(self, 0)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar2PattTab1Spec;
impl crate::RegisterSpec for Sar2PattTab1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_patt_tab1::R`](R) reader structure"]
impl crate::Readable for Sar2PattTab1Spec {}
#[doc = "`write(|w| ..)` method takes [`sar2_patt_tab1::W`](W) writer structure"]
impl crate::Writable for Sar2PattTab1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB1 to value 0"]
impl crate::Resettable for Sar2PattTab1Spec {}
