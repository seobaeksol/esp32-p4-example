#[doc = "Register `PROBE_OUT` reader"]
pub type R = crate::R<ProbeOutSpec>;
#[doc = "Field `REG_PROBE_TOP_OUT` reader - NA"]
pub type RegProbeTopOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_probe_top_out(&self) -> RegProbeTopOutR {
        RegProbeTopOutR::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`probe_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeOutSpec;
impl crate::RegisterSpec for ProbeOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_out::R`](R) reader structure"]
impl crate::Readable for ProbeOutSpec {}
#[doc = "`reset()` method sets PROBE_OUT to value 0"]
impl crate::Resettable for ProbeOutSpec {}
