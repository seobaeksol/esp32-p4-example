#[doc = "Register `REGDMA_LINK_2_ADDR` reader"]
pub type R = crate::R<RegdmaLink2AddrSpec>;
#[doc = "Register `REGDMA_LINK_2_ADDR` writer"]
pub type W = crate::W<RegdmaLink2AddrSpec>;
#[doc = "Field `LINK_ADDR_2` reader - Link_2_addr reg"]
pub type LinkAddr2R = crate::FieldReader<u32>;
#[doc = "Field `LINK_ADDR_2` writer - Link_2_addr reg"]
pub type LinkAddr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link_2_addr reg"]
    #[inline(always)]
    pub fn link_addr_2(&self) -> LinkAddr2R {
        LinkAddr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link_2_addr reg"]
    #[inline(always)]
    pub fn link_addr_2(&mut self) -> LinkAddr2W<'_, RegdmaLink2AddrSpec> {
        LinkAddr2W::new(self, 0)
    }
}
#[doc = "Link_2_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_link_2_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_link_2_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegdmaLink2AddrSpec;
impl crate::RegisterSpec for RegdmaLink2AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_link_2_addr::R`](R) reader structure"]
impl crate::Readable for RegdmaLink2AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`regdma_link_2_addr::W`](W) writer structure"]
impl crate::Writable for RegdmaLink2AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_LINK_2_ADDR to value 0"]
impl crate::Resettable for RegdmaLink2AddrSpec {}
