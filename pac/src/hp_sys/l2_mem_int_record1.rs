#[doc = "Register `L2_MEM_INT_RECORD1` reader"]
pub type R = crate::R<L2MemIntRecord1Spec>;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_ADDR` reader - NA"]
pub type RegL2MemEccErrIntAddrR = crate::FieldReader<u16>;
#[doc = "Field `REG_L2_MEM_ECC_ONE_BIT_ERR` reader - NA"]
pub type RegL2MemEccOneBitErrR = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ECC_TWO_BIT_ERR` reader - NA"]
pub type RegL2MemEccTwoBitErrR = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ECC_ERR_BIT` reader - NA"]
pub type RegL2MemEccErrBitR = crate::FieldReader<u16>;
#[doc = "Field `REG_L2_CACHE_ERR_BANK` reader - NA"]
pub type RegL2CacheErrBankR = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_addr(&self) -> RegL2MemEccErrIntAddrR {
        RegL2MemEccErrIntAddrR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_one_bit_err(&self) -> RegL2MemEccOneBitErrR {
        RegL2MemEccOneBitErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_two_bit_err(&self) -> RegL2MemEccTwoBitErrR {
        RegL2MemEccTwoBitErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_bit(&self) -> RegL2MemEccErrBitR {
        RegL2MemEccErrBitR::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn reg_l2_cache_err_bank(&self) -> RegL2CacheErrBankR {
        RegL2CacheErrBankR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_record1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2MemIntRecord1Spec;
impl crate::RegisterSpec for L2MemIntRecord1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_int_record1::R`](R) reader structure"]
impl crate::Readable for L2MemIntRecord1Spec {}
#[doc = "`reset()` method sets L2_MEM_INT_RECORD1 to value 0"]
impl crate::Resettable for L2MemIntRecord1Spec {}
