#[doc = "Register `INTER_MEM_START_ADDR0` reader"]
pub type R = crate::R<InterMemStartAddr0Spec>;
#[doc = "Register `INTER_MEM_START_ADDR0` writer"]
pub type W = crate::W<InterMemStartAddr0Spec>;
#[doc = "Field `ACCESS_INTER_MEM_START_ADDR0` reader - The start address of accessible address space."]
pub type AccessInterMemStartAddr0R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_INTER_MEM_START_ADDR0` writer - The start address of accessible address space."]
pub type AccessInterMemStartAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    pub fn access_inter_mem_start_addr0(&self) -> AccessInterMemStartAddr0R {
        AccessInterMemStartAddr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    pub fn access_inter_mem_start_addr0(
        &mut self,
    ) -> AccessInterMemStartAddr0W<'_, InterMemStartAddr0Spec> {
        AccessInterMemStartAddr0W::new(self, 0)
    }
}
#[doc = "Start address of inter memory range0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_mem_start_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_mem_start_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterMemStartAddr0Spec;
impl crate::RegisterSpec for InterMemStartAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inter_mem_start_addr0::R`](R) reader structure"]
impl crate::Readable for InterMemStartAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`inter_mem_start_addr0::W`](W) writer structure"]
impl crate::Writable for InterMemStartAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTER_MEM_START_ADDR0 to value 0x3010_0000"]
impl crate::Resettable for InterMemStartAddr0Spec {
    const RESET_VALUE: u32 = 0x3010_0000;
}
