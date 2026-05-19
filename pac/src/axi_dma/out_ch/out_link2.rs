#[doc = "Register `OUT_LINK2` reader"]
pub type R = crate::R<OutLink2Spec>;
#[doc = "Register `OUT_LINK2` writer"]
pub type W = crate::W<OutLink2Spec>;
#[doc = "Field `OUTLINK_ADDR` reader - This register stores the 32 least significant bits of the first outlink descriptor's address."]
pub type OutlinkAddrR = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - This register stores the 32 least significant bits of the first outlink descriptor's address."]
pub type OutlinkAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the 32 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OutlinkAddrR {
        OutlinkAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the 32 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&mut self) -> OutlinkAddrW<'_, OutLink2Spec> {
        OutlinkAddrW::new(self, 0)
    }
}
#[doc = "Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutLink2Spec;
impl crate::RegisterSpec for OutLink2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link2::R`](R) reader structure"]
impl crate::Readable for OutLink2Spec {}
#[doc = "`write(|w| ..)` method takes [`out_link2::W`](W) writer structure"]
impl crate::Writable for OutLink2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_LINK2 to value 0"]
impl crate::Resettable for OutLink2Spec {}
