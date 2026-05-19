#[doc = "Register `EXTER_MEM_END_ADDR0` reader"]
pub type R = crate::R<ExterMemEndAddr0Spec>;
#[doc = "Register `EXTER_MEM_END_ADDR0` writer"]
pub type W = crate::W<ExterMemEndAddr0Spec>;
#[doc = "Field `ACCESS_EXTER_MEM_END_ADDR0` reader - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessExterMemEndAddr0R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_EXTER_MEM_END_ADDR0` writer - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessExterMemEndAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_exter_mem_end_addr0(&self) -> AccessExterMemEndAddr0R {
        AccessExterMemEndAddr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_exter_mem_end_addr0(
        &mut self,
    ) -> AccessExterMemEndAddr0W<'_, ExterMemEndAddr0Spec> {
        AccessExterMemEndAddr0W::new(self, 0)
    }
}
#[doc = "end address of exter memory range0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_mem_end_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exter_mem_end_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExterMemEndAddr0Spec;
impl crate::RegisterSpec for ExterMemEndAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exter_mem_end_addr0::R`](R) reader structure"]
impl crate::Readable for ExterMemEndAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`exter_mem_end_addr0::W`](W) writer structure"]
impl crate::Writable for ExterMemEndAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTER_MEM_END_ADDR0 to value 0x8fff_ffff"]
impl crate::Resettable for ExterMemEndAddr0Spec {
    const RESET_VALUE: u32 = 0x8fff_ffff;
}
