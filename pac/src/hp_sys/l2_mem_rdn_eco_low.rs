#[doc = "Register `L2_MEM_RDN_ECO_LOW` reader"]
pub type R = crate::R<L2MemRdnEcoLowSpec>;
#[doc = "Register `L2_MEM_RDN_ECO_LOW` writer"]
pub type W = crate::W<L2MemRdnEcoLowSpec>;
#[doc = "Field `REG_L2_MEM_RDN_ECO_LOW` reader - NA"]
pub type RegL2MemRdnEcoLowR = crate::FieldReader<u32>;
#[doc = "Field `REG_L2_MEM_RDN_ECO_LOW` writer - NA"]
pub type RegL2MemRdnEcoLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_low(&self) -> RegL2MemRdnEcoLowR {
        RegL2MemRdnEcoLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_low(&mut self) -> RegL2MemRdnEcoLowW<'_, L2MemRdnEcoLowSpec> {
        RegL2MemRdnEcoLowW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_rdn_eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2MemRdnEcoLowSpec;
impl crate::RegisterSpec for L2MemRdnEcoLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for L2MemRdnEcoLowSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for L2MemRdnEcoLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_RDN_ECO_LOW to value 0"]
impl crate::Resettable for L2MemRdnEcoLowSpec {}
