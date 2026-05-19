#[doc = "Register `INTER_MEM_END_ADDR0` reader"]
pub type R = crate::R<InterMemEndAddr0Spec>;
#[doc = "Register `INTER_MEM_END_ADDR0` writer"]
pub type W = crate::W<InterMemEndAddr0Spec>;
#[doc = "Field `ACCESS_INTER_MEM_END_ADDR0` reader - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessInterMemEndAddr0R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_INTER_MEM_END_ADDR0` writer - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type AccessInterMemEndAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_inter_mem_end_addr0(&self) -> AccessInterMemEndAddr0R {
        AccessInterMemEndAddr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_inter_mem_end_addr0(
        &mut self,
    ) -> AccessInterMemEndAddr0W<'_, InterMemEndAddr0Spec> {
        AccessInterMemEndAddr0W::new(self, 0)
    }
}
#[doc = "end address of inter memory range0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_mem_end_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_mem_end_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterMemEndAddr0Spec;
impl crate::RegisterSpec for InterMemEndAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inter_mem_end_addr0::R`](R) reader structure"]
impl crate::Readable for InterMemEndAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`inter_mem_end_addr0::W`](W) writer structure"]
impl crate::Writable for InterMemEndAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTER_MEM_END_ADDR0 to value 0x8fff_ffff"]
impl crate::Resettable for InterMemEndAddr0Spec {
    const RESET_VALUE: u32 = 0x8fff_ffff;
}
