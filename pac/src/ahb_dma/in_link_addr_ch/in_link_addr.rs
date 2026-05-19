#[doc = "Register `IN_LINK_ADDR` reader"]
pub type R = crate::R<InLinkAddrSpec>;
#[doc = "Register `IN_LINK_ADDR` writer"]
pub type W = crate::W<InLinkAddrSpec>;
#[doc = "Field `INLINK_ADDR_CH0` reader - Configures the 32 bits of the first receive descriptor's address"]
pub type InlinkAddrCh0R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR_CH0` writer - Configures the 32 bits of the first receive descriptor's address"]
pub type InlinkAddrCh0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address"]
    #[inline(always)]
    pub fn inlink_addr_ch0(&self) -> InlinkAddrCh0R {
        InlinkAddrCh0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address"]
    #[inline(always)]
    pub fn inlink_addr_ch0(&mut self) -> InlinkAddrCh0W<'_, InLinkAddrSpec> {
        InlinkAddrCh0W::new(self, 0)
    }
}
#[doc = "Link list descriptor address configuration of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InLinkAddrSpec;
impl crate::RegisterSpec for InLinkAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link_addr::R`](R) reader structure"]
impl crate::Readable for InLinkAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`in_link_addr::W`](W) writer structure"]
impl crate::Writable for InLinkAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_LINK_ADDR to value 0"]
impl crate::Resettable for InLinkAddrSpec {}
