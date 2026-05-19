#[doc = "Register `IN_LINK2` reader"]
pub type R = crate::R<InLink2Spec>;
#[doc = "Register `IN_LINK2` writer"]
pub type W = crate::W<InLink2Spec>;
#[doc = "Field `INLINK_ADDR` reader - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type InlinkAddrR = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type InlinkAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> InlinkAddrR {
        InlinkAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&mut self) -> InlinkAddrW<'_, InLink2Spec> {
        InlinkAddrW::new(self, 0)
    }
}
#[doc = "Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InLink2Spec;
impl crate::RegisterSpec for InLink2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link2::R`](R) reader structure"]
impl crate::Readable for InLink2Spec {}
#[doc = "`write(|w| ..)` method takes [`in_link2::W`](W) writer structure"]
impl crate::Writable for InLink2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_LINK2 to value 0"]
impl crate::Resettable for InLink2Spec {}
