#[doc = "Register `EXTR_MEM_END_ADDR` reader"]
pub type R = crate::R<ExtrMemEndAddrSpec>;
#[doc = "Register `EXTR_MEM_END_ADDR` writer"]
pub type W = crate::W<ExtrMemEndAddrSpec>;
#[doc = "Field `ACCESS_EXTR_MEM_END_ADDR` reader - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessExtrMemEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_EXTR_MEM_END_ADDR` writer - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessExtrMemEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_extr_mem_end_addr(&self) -> AccessExtrMemEndAddrR {
        AccessExtrMemEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_extr_mem_end_addr(&mut self) -> AccessExtrMemEndAddrW<'_, ExtrMemEndAddrSpec> {
        AccessExtrMemEndAddrW::new(self, 0)
    }
}
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::Reg::read) this register and get [`extr_mem_end_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extr_mem_end_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtrMemEndAddrSpec;
impl crate::RegisterSpec for ExtrMemEndAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extr_mem_end_addr::R`](R) reader structure"]
impl crate::Readable for ExtrMemEndAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`extr_mem_end_addr::W`](W) writer structure"]
impl crate::Writable for ExtrMemEndAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTR_MEM_END_ADDR to value 0x8fff_ffff"]
impl crate::Resettable for ExtrMemEndAddrSpec {
    const RESET_VALUE: u32 = 0x8fff_ffff;
}
