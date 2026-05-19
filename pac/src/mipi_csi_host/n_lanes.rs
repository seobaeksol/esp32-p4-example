#[doc = "Register `N_LANES` reader"]
pub type R = crate::R<NLanesSpec>;
#[doc = "Register `N_LANES` writer"]
pub type W = crate::W<NLanesSpec>;
#[doc = "Field `N_LANES` reader - NA"]
pub type NLanesR = crate::FieldReader;
#[doc = "Field `N_LANES` writer - NA"]
pub type NLanesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn n_lanes(&self) -> NLanesR {
        NLanesR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn n_lanes(&mut self) -> NLanesW<'_, NLanesSpec> {
        NLanesW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`n_lanes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_lanes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NLanesSpec;
impl crate::RegisterSpec for NLanesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_lanes::R`](R) reader structure"]
impl crate::Readable for NLanesSpec {}
#[doc = "`write(|w| ..)` method takes [`n_lanes::W`](W) writer structure"]
impl crate::Writable for NLanesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets N_LANES to value 0x01"]
impl crate::Resettable for NLanesSpec {
    const RESET_VALUE: u32 = 0x01;
}
