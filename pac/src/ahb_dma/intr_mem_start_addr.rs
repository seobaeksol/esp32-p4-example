#[doc = "Register `INTR_MEM_START_ADDR` reader"]
pub type R = crate::R<IntrMemStartAddrSpec>;
#[doc = "Register `INTR_MEM_START_ADDR` writer"]
pub type W = crate::W<IntrMemStartAddrSpec>;
#[doc = "Field `ACCESS_INTR_MEM_START_ADDR` reader - Accessible address space start address configuration register"]
pub type AccessIntrMemStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_INTR_MEM_START_ADDR` writer - Accessible address space start address configuration register"]
pub type AccessIntrMemStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub fn access_intr_mem_start_addr(&self) -> AccessIntrMemStartAddrR {
        AccessIntrMemStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub fn access_intr_mem_start_addr(
        &mut self,
    ) -> AccessIntrMemStartAddrW<'_, IntrMemStartAddrSpec> {
        AccessIntrMemStartAddrW::new(self, 0)
    }
}
#[doc = "Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMemStartAddrSpec;
impl crate::RegisterSpec for IntrMemStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mem_start_addr::R`](R) reader structure"]
impl crate::Readable for IntrMemStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mem_start_addr::W`](W) writer structure"]
impl crate::Writable for IntrMemStartAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_MEM_START_ADDR to value 0"]
impl crate::Resettable for IntrMemStartAddrSpec {}
