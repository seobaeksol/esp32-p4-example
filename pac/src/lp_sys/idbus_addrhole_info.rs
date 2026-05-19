#[doc = "Register `IDBUS_ADDRHOLE_INFO` reader"]
pub type R = crate::R<IdbusAddrholeInfoSpec>;
#[doc = "Field `IDBUS_ADDRHOLE_ID` reader - need_des"]
pub type IdbusAddrholeIdR = crate::FieldReader;
#[doc = "Field `IDBUS_ADDRHOLE_WR` reader - need_des"]
pub type IdbusAddrholeWrR = crate::BitReader;
#[doc = "Field `IDBUS_ADDRHOLE_SECURE` reader - need_des"]
pub type IdbusAddrholeSecureR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn idbus_addrhole_id(&self) -> IdbusAddrholeIdR {
        IdbusAddrholeIdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn idbus_addrhole_wr(&self) -> IdbusAddrholeWrR {
        IdbusAddrholeWrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn idbus_addrhole_secure(&self) -> IdbusAddrholeSecureR {
        IdbusAddrholeSecureR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`idbus_addrhole_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdbusAddrholeInfoSpec;
impl crate::RegisterSpec for IdbusAddrholeInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idbus_addrhole_info::R`](R) reader structure"]
impl crate::Readable for IdbusAddrholeInfoSpec {}
#[doc = "`reset()` method sets IDBUS_ADDRHOLE_INFO to value 0"]
impl crate::Resettable for IdbusAddrholeInfoSpec {}
