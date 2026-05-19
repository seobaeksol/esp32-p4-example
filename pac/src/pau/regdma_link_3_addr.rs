#[doc = "Register `REGDMA_LINK_3_ADDR` reader"]
pub type R = crate::R<RegdmaLink3AddrSpec>;
#[doc = "Register `REGDMA_LINK_3_ADDR` writer"]
pub type W = crate::W<RegdmaLink3AddrSpec>;
#[doc = "Field `LINK_ADDR_3` reader - Link_3_addr reg"]
pub type LinkAddr3R = crate::FieldReader<u32>;
#[doc = "Field `LINK_ADDR_3` writer - Link_3_addr reg"]
pub type LinkAddr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link_3_addr reg"]
    #[inline(always)]
    pub fn link_addr_3(&self) -> LinkAddr3R {
        LinkAddr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link_3_addr reg"]
    #[inline(always)]
    pub fn link_addr_3(&mut self) -> LinkAddr3W<'_, RegdmaLink3AddrSpec> {
        LinkAddr3W::new(self, 0)
    }
}
#[doc = "Link_3_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_link_3_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_link_3_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegdmaLink3AddrSpec;
impl crate::RegisterSpec for RegdmaLink3AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_link_3_addr::R`](R) reader structure"]
impl crate::Readable for RegdmaLink3AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`regdma_link_3_addr::W`](W) writer structure"]
impl crate::Writable for RegdmaLink3AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_LINK_3_ADDR to value 0"]
impl crate::Resettable for RegdmaLink3AddrSpec {}
