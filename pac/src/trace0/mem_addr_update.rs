#[doc = "Register `MEM_ADDR_UPDATE` writer"]
pub type W = crate::W<MemAddrUpdateSpec>;
#[doc = "Field `MEM_CURRENT_ADDR_UPDATE` writer - when set, the will \\hyperref\\[fielddesc:TRACEMEMCURRENTADDR\\]{TRACE_MEM_CURRENT_ADDR} update to \\hyperref\\[fielddesc:TRACEMEMSTARTADDR\\]{TRACE_MEM_START_ADDR}."]
pub type MemCurrentAddrUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - when set, the will \\hyperref\\[fielddesc:TRACEMEMCURRENTADDR\\]{TRACE_MEM_CURRENT_ADDR} update to \\hyperref\\[fielddesc:TRACEMEMSTARTADDR\\]{TRACE_MEM_START_ADDR}."]
    #[inline(always)]
    pub fn mem_current_addr_update(&mut self) -> MemCurrentAddrUpdateW<'_, MemAddrUpdateSpec> {
        MemCurrentAddrUpdateW::new(self, 0)
    }
}
#[doc = "mem addr update\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_addr_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAddrUpdateSpec;
impl crate::RegisterSpec for MemAddrUpdateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mem_addr_update::W`](W) writer structure"]
impl crate::Writable for MemAddrUpdateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_ADDR_UPDATE to value 0"]
impl crate::Resettable for MemAddrUpdateSpec {}
