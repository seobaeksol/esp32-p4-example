#[doc = "Register `ICM_INT_RAW` reader"]
pub type R = crate::R<IcmIntRawSpec>;
#[doc = "Register `ICM_INT_RAW` writer"]
pub type W = crate::W<IcmIntRawSpec>;
#[doc = "Field `DLOCK_INT_RAW` reader - "]
pub type DlockIntRawR = crate::BitReader;
#[doc = "Field `DLOCK_INT_RAW` writer - "]
pub type DlockIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_SYS_ADDRHOLE_INT_RAW` reader - "]
pub type IcmSysAddrholeIntRawR = crate::BitReader;
#[doc = "Field `ICM_SYS_ADDRHOLE_INT_RAW` writer - "]
pub type IcmSysAddrholeIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_CPU_ADDRHOLE_INT_RAW` reader - "]
pub type IcmCpuAddrholeIntRawR = crate::BitReader;
#[doc = "Field `ICM_CPU_ADDRHOLE_INT_RAW` writer - "]
pub type IcmCpuAddrholeIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dlock_int_raw(&self) -> DlockIntRawR {
        DlockIntRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_sys_addrhole_int_raw(&self) -> IcmSysAddrholeIntRawR {
        IcmSysAddrholeIntRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icm_cpu_addrhole_int_raw(&self) -> IcmCpuAddrholeIntRawR {
        IcmCpuAddrholeIntRawR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dlock_int_raw(&mut self) -> DlockIntRawW<'_, IcmIntRawSpec> {
        DlockIntRawW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_sys_addrhole_int_raw(&mut self) -> IcmSysAddrholeIntRawW<'_, IcmIntRawSpec> {
        IcmSysAddrholeIntRawW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icm_cpu_addrhole_int_raw(&mut self) -> IcmCpuAddrholeIntRawW<'_, IcmIntRawSpec> {
        IcmCpuAddrholeIntRawW::new(self, 2)
    }
}
#[doc = "ICM interrupt raw\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmIntRawSpec;
impl crate::RegisterSpec for IcmIntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_int_raw::R`](R) reader structure"]
impl crate::Readable for IcmIntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`icm_int_raw::W`](W) writer structure"]
impl crate::Writable for IcmIntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_INT_RAW to value 0"]
impl crate::Resettable for IcmIntRawSpec {}
