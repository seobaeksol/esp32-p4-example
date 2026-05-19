#[doc = "Register `STATUS3` reader"]
pub type R = crate::R<Status3Spec>;
#[doc = "Field `YO` reader - component y transferred from rgb input"]
pub type YoR = crate::FieldReader<u16>;
#[doc = "Field `Y_READY` reader - component y valid signal, high active"]
pub type YReadyR = crate::BitReader;
#[doc = "Field `CBO` reader - component cb transferred from rgb input"]
pub type CboR = crate::FieldReader<u16>;
#[doc = "Field `CB_READY` reader - component cb valid signal, high active"]
pub type CbReadyR = crate::BitReader;
#[doc = "Field `CRO` reader - component cr transferred from rgb input"]
pub type CroR = crate::FieldReader<u16>;
#[doc = "Field `CR_READY` reader - component cr valid signal, high active"]
pub type CrReadyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - component y transferred from rgb input"]
    #[inline(always)]
    pub fn yo(&self) -> YoR {
        YoR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - component y valid signal, high active"]
    #[inline(always)]
    pub fn y_ready(&self) -> YReadyR {
        YReadyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:18 - component cb transferred from rgb input"]
    #[inline(always)]
    pub fn cbo(&self) -> CboR {
        CboR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bit 19 - component cb valid signal, high active"]
    #[inline(always)]
    pub fn cb_ready(&self) -> CbReadyR {
        CbReadyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28 - component cr transferred from rgb input"]
    #[inline(always)]
    pub fn cro(&self) -> CroR {
        CroR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - component cr valid signal, high active"]
    #[inline(always)]
    pub fn cr_ready(&self) -> CrReadyR {
        CrReadyR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`status3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status3Spec;
impl crate::RegisterSpec for Status3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status3::R`](R) reader structure"]
impl crate::Readable for Status3Spec {}
#[doc = "`reset()` method sets STATUS3 to value 0"]
impl crate::Resettable for Status3Spec {}
