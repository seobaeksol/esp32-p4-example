#[doc = "Register `ICM_SYS_ADDRHOLE_INFO` reader"]
pub type R = crate::R<IcmSysAddrholeInfoSpec>;
#[doc = "Field `ICM_SYS_ADDRHOLE_ID` reader - "]
pub type IcmSysAddrholeIdR = crate::FieldReader;
#[doc = "Field `ICM_SYS_ADDRHOLE_WR` reader - "]
pub type IcmSysAddrholeWrR = crate::BitReader;
#[doc = "Field `ICM_SYS_ADDRHOLE_SECURE` reader - "]
pub type IcmSysAddrholeSecureR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn icm_sys_addrhole_id(&self) -> IcmSysAddrholeIdR {
        IcmSysAddrholeIdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn icm_sys_addrhole_wr(&self) -> IcmSysAddrholeWrR {
        IcmSysAddrholeWrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn icm_sys_addrhole_secure(&self) -> IcmSysAddrholeSecureR {
        IcmSysAddrholeSecureR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "SYS address hole info\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_sys_addrhole_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmSysAddrholeInfoSpec;
impl crate::RegisterSpec for IcmSysAddrholeInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_sys_addrhole_info::R`](R) reader structure"]
impl crate::Readable for IcmSysAddrholeInfoSpec {}
#[doc = "`reset()` method sets ICM_SYS_ADDRHOLE_INFO to value 0"]
impl crate::Resettable for IcmSysAddrholeInfoSpec {}
