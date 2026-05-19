#[doc = "Register `LINK_ADDR` reader"]
pub type R = crate::R<LinkAddrSpec>;
#[doc = "Register `LINK_ADDR` writer"]
pub type W = crate::W<LinkAddrSpec>;
#[doc = "Field `OUTLINK_ADDR` reader - This register stores the first outlink descriptor's address."]
pub type OutlinkAddrR = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - This register stores the first outlink descriptor's address."]
pub type OutlinkAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OutlinkAddrR {
        OutlinkAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&mut self) -> OutlinkAddrW<'_, LinkAddrSpec> {
        OutlinkAddrW::new(self, 0)
    }
}
#[doc = "TX CHx out_link dscr addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`link_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkAddrSpec;
impl crate::RegisterSpec for LinkAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_addr::R`](R) reader structure"]
impl crate::Readable for LinkAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`link_addr::W`](W) writer structure"]
impl crate::Writable for LinkAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK_ADDR to value 0"]
impl crate::Resettable for LinkAddrSpec {}
