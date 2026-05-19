#[doc = "Register `INT_INFO` reader"]
pub type R = crate::R<IntInfoSpec>;
#[doc = "Field `NUM_INT` reader - The number of interrupt sources."]
pub type NumIntR = crate::FieldReader<u16>;
#[doc = "Field `VERSION` reader - The lower 4 bits are the modified version of the hardware implementation; the upper 4 bits are the CLIC architecture version information."]
pub type VersionR = crate::FieldReader;
#[doc = "Field `CTLBITS` reader - The effective bits of priority in the CLICINTCTL register."]
pub type CtlbitsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - The number of interrupt sources."]
    #[inline(always)]
    pub fn num_int(&self) -> NumIntR {
        NumIntR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - The lower 4 bits are the modified version of the hardware implementation; the upper 4 bits are the CLIC architecture version information."]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:28 - The effective bits of priority in the CLICINTCTL register."]
    #[inline(always)]
    pub fn ctlbits(&self) -> CtlbitsR {
        CtlbitsR::new(((self.bits >> 21) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntInfoSpec;
impl crate::RegisterSpec for IntInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_info::R`](R) reader structure"]
impl crate::Readable for IntInfoSpec {}
#[doc = "`reset()` method sets INT_INFO to value 0"]
impl crate::Resettable for IntInfoSpec {}
