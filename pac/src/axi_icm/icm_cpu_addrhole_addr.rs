#[doc = "Register `ICM_CPU_ADDRHOLE_ADDR` reader"]
pub type R = crate::R<IcmCpuAddrholeAddrSpec>;
#[doc = "Field `ICM_CPU_ADDRHOLE_ADDR` reader - "]
pub type IcmCpuAddrholeAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn icm_cpu_addrhole_addr(&self) -> IcmCpuAddrholeAddrR {
        IcmCpuAddrholeAddrR::new(self.bits)
    }
}
#[doc = "CPU address hole address\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_cpu_addrhole_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmCpuAddrholeAddrSpec;
impl crate::RegisterSpec for IcmCpuAddrholeAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_cpu_addrhole_addr::R`](R) reader structure"]
impl crate::Readable for IcmCpuAddrholeAddrSpec {}
#[doc = "`reset()` method sets ICM_CPU_ADDRHOLE_ADDR to value 0"]
impl crate::Resettable for IcmCpuAddrholeAddrSpec {}
