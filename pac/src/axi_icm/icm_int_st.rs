#[doc = "Register `ICM_INT_ST` reader"]
pub type R = crate::R<IcmIntStSpec>;
#[doc = "Field `DLOCK_INT_ST` reader - "]
pub type DlockIntStR = crate::BitReader;
#[doc = "Field `ICM_SYS_ADDRHOLE_INT_ST` reader - "]
pub type IcmSysAddrholeIntStR = crate::BitReader;
#[doc = "Field `ICM_CPU_ADDRHOLE_INT_ST` reader - "]
pub type IcmCpuAddrholeIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dlock_int_st(&self) -> DlockIntStR {
        DlockIntStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_sys_addrhole_int_st(&self) -> IcmSysAddrholeIntStR {
        IcmSysAddrholeIntStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icm_cpu_addrhole_int_st(&self) -> IcmCpuAddrholeIntStR {
        IcmCpuAddrholeIntStR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ICM interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmIntStSpec;
impl crate::RegisterSpec for IcmIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_int_st::R`](R) reader structure"]
impl crate::Readable for IcmIntStSpec {}
#[doc = "`reset()` method sets ICM_INT_ST to value 0"]
impl crate::Resettable for IcmIntStSpec {}
