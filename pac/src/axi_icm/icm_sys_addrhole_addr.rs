#[doc = "Register `ICM_SYS_ADDRHOLE_ADDR` reader"]
pub type R = crate::R<IcmSysAddrholeAddrSpec>;
#[doc = "Field `ICM_SYS_ADDRHOLE_ADDR` reader - "]
pub type IcmSysAddrholeAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn icm_sys_addrhole_addr(&self) -> IcmSysAddrholeAddrR {
        IcmSysAddrholeAddrR::new(self.bits)
    }
}
#[doc = "SYS address hole address\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_sys_addrhole_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmSysAddrholeAddrSpec;
impl crate::RegisterSpec for IcmSysAddrholeAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_sys_addrhole_addr::R`](R) reader structure"]
impl crate::Readable for IcmSysAddrholeAddrSpec {}
#[doc = "`reset()` method sets ICM_SYS_ADDRHOLE_ADDR to value 0"]
impl crate::Resettable for IcmSysAddrholeAddrSpec {}
