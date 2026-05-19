#[doc = "Register `PMT_CSR` reader"]
pub type R = crate::R<PmtCsrSpec>;
#[doc = "Field `PWRDWN` reader - When set the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame.This bit must only be set when MGKPKTEN GLBLUCAST or RWKPKTEN bit is set high."]
pub type PwrdwnR = crate::BitReader;
#[doc = "Field `MGKPKTEN` reader - When set enables generation of a power management event because of magic packet reception."]
pub type MgkpktenR = crate::BitReader;
#[doc = "Field `RWKPKTEN` reader - When set enables generation of a power management event because of remote wake-up frame reception"]
pub type RwkpktenR = crate::BitReader;
#[doc = "Field `MGKPRCVD` reader - When set this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
pub type MgkprcvdR = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - When set this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
pub type RwkprcvdR = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - When set enables any unicast packet filtered by the MAC (DAFilter) address recognition to be a remote wake-up frame."]
pub type GlblucastR = crate::BitReader;
#[doc = "Field `RWKPTR` reader - The maximum value of the pointer is 7 the detail information please refer to PMT_RWUFFR."]
pub type RwkptrR = crate::FieldReader;
#[doc = "Field `RWKFILTRST` reader - When this bit is set it resets the RWKPTR register to 3’b000."]
pub type RwkfiltrstR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When set the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame.This bit must only be set when MGKPKTEN GLBLUCAST or RWKPKTEN bit is set high."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PwrdwnR {
        PwrdwnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set enables generation of a power management event because of magic packet reception."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MgkpktenR {
        MgkpktenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set enables generation of a power management event because of remote wake-up frame reception"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RwkpktenR {
        RwkpktenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - When set this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MgkprcvdR {
        MgkprcvdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RwkprcvdR {
        RwkprcvdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - When set enables any unicast packet filtered by the MAC (DAFilter) address recognition to be a remote wake-up frame."]
    #[inline(always)]
    pub fn glblucast(&self) -> GlblucastR {
        GlblucastR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 24:28 - The maximum value of the pointer is 7 the detail information please refer to PMT_RWUFFR."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RwkptrR {
        RwkptrR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - When this bit is set it resets the RWKPTR register to 3’b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RwkfiltrstR {
        RwkfiltrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PMT Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmtCsrSpec;
impl crate::RegisterSpec for PmtCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmt_csr::R`](R) reader structure"]
impl crate::Readable for PmtCsrSpec {}
#[doc = "`reset()` method sets PMT_CSR to value 0"]
impl crate::Resettable for PmtCsrSpec {}
