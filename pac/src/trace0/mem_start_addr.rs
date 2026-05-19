#[doc = "Register `MEM_START_ADDR` reader"]
pub type R = crate::R<MemStartAddrSpec>;
#[doc = "Register `MEM_START_ADDR` writer"]
pub type W = crate::W<MemStartAddrSpec>;
#[doc = "Field `MEM_START_ADDR` reader - The start address of trace memory"]
pub type MemStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `MEM_START_ADDR` writer - The start address of trace memory"]
pub type MemStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of trace memory"]
    #[inline(always)]
    pub fn mem_start_addr(&self) -> MemStartAddrR {
        MemStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of trace memory"]
    #[inline(always)]
    pub fn mem_start_addr(&mut self) -> MemStartAddrW<'_, MemStartAddrSpec> {
        MemStartAddrW::new(self, 0)
    }
}
#[doc = "mem start addr\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemStartAddrSpec;
impl crate::RegisterSpec for MemStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_start_addr::R`](R) reader structure"]
impl crate::Readable for MemStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_start_addr::W`](W) writer structure"]
impl crate::Writable for MemStartAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_START_ADDR to value 0"]
impl crate::Resettable for MemStartAddrSpec {}
