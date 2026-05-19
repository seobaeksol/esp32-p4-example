#[doc = "Register `LP_ADDRHOLE_ADDR` reader"]
pub type R = crate::R<LpAddrholeAddrSpec>;
#[doc = "Field `LP_ADDRHOLE_ADDR` reader - need_des"]
pub type LpAddrholeAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_addrhole_addr(&self) -> LpAddrholeAddrR {
        LpAddrholeAddrR::new(self.bits)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_addrhole_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAddrholeAddrSpec;
impl crate::RegisterSpec for LpAddrholeAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_addrhole_addr::R`](R) reader structure"]
impl crate::Readable for LpAddrholeAddrSpec {}
#[doc = "`reset()` method sets LP_ADDRHOLE_ADDR to value 0"]
impl crate::Resettable for LpAddrholeAddrSpec {}
