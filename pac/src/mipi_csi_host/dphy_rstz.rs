#[doc = "Register `DPHY_RSTZ` reader"]
pub type R = crate::R<DphyRstzSpec>;
#[doc = "Register `DPHY_RSTZ` writer"]
pub type W = crate::W<DphyRstzSpec>;
#[doc = "Field `DPHY_RSTZ` reader - NA"]
pub type DphyRstzR = crate::BitReader;
#[doc = "Field `DPHY_RSTZ` writer - NA"]
pub type DphyRstzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dphy_rstz(&self) -> DphyRstzR {
        DphyRstzR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dphy_rstz(&mut self) -> DphyRstzW<'_, DphyRstzSpec> {
        DphyRstzW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_rstz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_rstz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyRstzSpec;
impl crate::RegisterSpec for DphyRstzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_rstz::R`](R) reader structure"]
impl crate::Readable for DphyRstzSpec {}
#[doc = "`write(|w| ..)` method takes [`dphy_rstz::W`](W) writer structure"]
impl crate::Writable for DphyRstzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_RSTZ to value 0"]
impl crate::Resettable for DphyRstzSpec {}
