#[doc = "Register `MEM_CURRENT_ADDR` reader"]
pub type R = crate::R<MemCurrentAddrSpec>;
#[doc = "Field `MEM_CURRENT_ADDR` reader - current_mem_addr,indicate that next writing addr"]
pub type MemCurrentAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - current_mem_addr,indicate that next writing addr"]
    #[inline(always)]
    pub fn mem_current_addr(&self) -> MemCurrentAddrR {
        MemCurrentAddrR::new(self.bits)
    }
}
#[doc = "mem current addr\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_current_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemCurrentAddrSpec;
impl crate::RegisterSpec for MemCurrentAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_current_addr::R`](R) reader structure"]
impl crate::Readable for MemCurrentAddrSpec {}
#[doc = "`reset()` method sets MEM_CURRENT_ADDR to value 0"]
impl crate::Resettable for MemCurrentAddrSpec {}
