#[doc = "Register `SAR2_PATT_TAB2` reader"]
pub type R = crate::R<Sar2PattTab2Spec>;
#[doc = "Register `SAR2_PATT_TAB2` writer"]
pub type W = crate::W<Sar2PattTab2Spec>;
#[doc = "Field `SAR2_PATT_TAB2` reader - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
pub type Sar2PattTab2R = crate::FieldReader<u32>;
#[doc = "Field `SAR2_PATT_TAB2` writer - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
pub type Sar2PattTab2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab2(&self) -> Sar2PattTab2R {
        Sar2PattTab2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab2(&mut self) -> Sar2PattTab2W<'_, Sar2PattTab2Spec> {
        Sar2PattTab2W::new(self, 0)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar2PattTab2Spec;
impl crate::RegisterSpec for Sar2PattTab2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_patt_tab2::R`](R) reader structure"]
impl crate::Readable for Sar2PattTab2Spec {}
#[doc = "`write(|w| ..)` method takes [`sar2_patt_tab2::W`](W) writer structure"]
impl crate::Writable for Sar2PattTab2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB2 to value 0"]
impl crate::Resettable for Sar2PattTab2Spec {}
