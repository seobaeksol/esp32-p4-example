#[doc = "Register `MEM_END_ADDR` reader"]
pub type R = crate::R<MemEndAddrSpec>;
#[doc = "Register `MEM_END_ADDR` writer"]
pub type W = crate::W<MemEndAddrSpec>;
#[doc = "Field `MEM_END_ADDR` reader - The end address of trace memory"]
pub type MemEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `MEM_END_ADDR` writer - The end address of trace memory"]
pub type MemEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of trace memory"]
    #[inline(always)]
    pub fn mem_end_addr(&self) -> MemEndAddrR {
        MemEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of trace memory"]
    #[inline(always)]
    pub fn mem_end_addr(&mut self) -> MemEndAddrW<'_, MemEndAddrSpec> {
        MemEndAddrW::new(self, 0)
    }
}
#[doc = "mem end addr\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_end_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_end_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemEndAddrSpec;
impl crate::RegisterSpec for MemEndAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_end_addr::R`](R) reader structure"]
impl crate::Readable for MemEndAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_end_addr::W`](W) writer structure"]
impl crate::Writable for MemEndAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_END_ADDR to value 0xffff_ffff"]
impl crate::Resettable for MemEndAddrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
