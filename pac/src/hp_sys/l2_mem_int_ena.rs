#[doc = "Register `L2_MEM_INT_ENA` reader"]
pub type R = crate::R<L2MemIntEnaSpec>;
#[doc = "Register `L2_MEM_INT_ENA` writer"]
pub type W = crate::W<L2MemIntEnaSpec>;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_ENA` reader - NA"]
pub type RegL2MemEccErrIntEnaR = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_ENA` writer - NA"]
pub type RegL2MemEccErrIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_ENA` reader - NA"]
pub type RegL2MemExceedAddrIntEnaR = crate::BitReader;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_ENA` writer - NA"]
pub type RegL2MemExceedAddrIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_ENA` reader - NA"]
pub type RegL2MemErrRespIntEnaR = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_ENA` writer - NA"]
pub type RegL2MemErrRespIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_ena(&self) -> RegL2MemEccErrIntEnaR {
        RegL2MemEccErrIntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_ena(&self) -> RegL2MemExceedAddrIntEnaR {
        RegL2MemExceedAddrIntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_ena(&self) -> RegL2MemErrRespIntEnaR {
        RegL2MemErrRespIntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_ena(&mut self) -> RegL2MemEccErrIntEnaW<'_, L2MemIntEnaSpec> {
        RegL2MemEccErrIntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_ena(
        &mut self,
    ) -> RegL2MemExceedAddrIntEnaW<'_, L2MemIntEnaSpec> {
        RegL2MemExceedAddrIntEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_ena(&mut self) -> RegL2MemErrRespIntEnaW<'_, L2MemIntEnaSpec> {
        RegL2MemErrRespIntEnaW::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2MemIntEnaSpec;
impl crate::RegisterSpec for L2MemIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_int_ena::R`](R) reader structure"]
impl crate::Readable for L2MemIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_int_ena::W`](W) writer structure"]
impl crate::Writable for L2MemIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_INT_ENA to value 0"]
impl crate::Resettable for L2MemIntEnaSpec {}
