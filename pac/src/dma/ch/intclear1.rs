#[doc = "Register `INTCLEAR1` writer"]
pub type W = crate::W<Intclear1Spec>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT` writer - NA"]
pub type Ch1ClearEccProtChmemCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type Ch1ClearEccProtChmemUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT` writer - NA"]
pub type Ch1ClearEccProtUidmemCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type Ch1ClearEccProtUidmemUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_chmem_correrr_intstat(
        &mut self,
    ) -> Ch1ClearEccProtChmemCorrerrIntstatW<'_, Intclear1Spec> {
        Ch1ClearEccProtChmemCorrerrIntstatW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_chmem_uncorrerr_intstat(
        &mut self,
    ) -> Ch1ClearEccProtChmemUncorrerrIntstatW<'_, Intclear1Spec> {
        Ch1ClearEccProtChmemUncorrerrIntstatW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_uidmem_correrr_intstat(
        &mut self,
    ) -> Ch1ClearEccProtUidmemCorrerrIntstatW<'_, Intclear1Spec> {
        Ch1ClearEccProtUidmemCorrerrIntstatW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_uidmem_uncorrerr_intstat(
        &mut self,
    ) -> Ch1ClearEccProtUidmemUncorrerrIntstatW<'_, Intclear1Spec> {
        Ch1ClearEccProtUidmemUncorrerrIntstatW::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intclear1Spec;
impl crate::RegisterSpec for Intclear1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclear1::W`](W) writer structure"]
impl crate::Writable for Intclear1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLEAR1 to value 0"]
impl crate::Resettable for Intclear1Spec {}
