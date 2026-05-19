#[doc = "Register `EXTER_MEM_END_ADDR1` reader"]
pub type R = crate::R<ExterMemEndAddr1Spec>;
#[doc = "Register `EXTER_MEM_END_ADDR1` writer"]
pub type W = crate::W<ExterMemEndAddr1Spec>;
#[doc = "Field `ACCESS_EXTER_MEM_END_ADDR1` reader - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessExterMemEndAddr1R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_EXTER_MEM_END_ADDR1` writer - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessExterMemEndAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_exter_mem_end_addr1(&self) -> AccessExterMemEndAddr1R {
        AccessExterMemEndAddr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_exter_mem_end_addr1(
        &mut self,
    ) -> AccessExterMemEndAddr1W<'_, ExterMemEndAddr1Spec> {
        AccessExterMemEndAddr1W::new(self, 0)
    }
}
#[doc = "end address of exter memory range1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_mem_end_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exter_mem_end_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExterMemEndAddr1Spec;
impl crate::RegisterSpec for ExterMemEndAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exter_mem_end_addr1::R`](R) reader structure"]
impl crate::Readable for ExterMemEndAddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`exter_mem_end_addr1::W`](W) writer structure"]
impl crate::Writable for ExterMemEndAddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTER_MEM_END_ADDR1 to value 0x8fff_ffff"]
impl crate::Resettable for ExterMemEndAddr1Spec {
    const RESET_VALUE: u32 = 0x8fff_ffff;
}
