#[doc = "Register `CAP_STATUS` reader"]
pub type R = crate::R<CapStatusSpec>;
#[doc = "Field `CAP0_EDGE` reader - Represents edge of last capture trigger on channel0.\\\\0: Posedge\\\\1: Negedge"]
pub type Cap0EdgeR = crate::BitReader;
#[doc = "Field `CAP1_EDGE` reader - Represents edge of last capture trigger on channel1.\\\\0: Posedge\\\\1: Negedge"]
pub type Cap1EdgeR = crate::BitReader;
#[doc = "Field `CAP2_EDGE` reader - Represents edge of last capture trigger on channel2.\\\\0: Posedge\\\\1: Negedge"]
pub type Cap2EdgeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents edge of last capture trigger on channel0.\\\\0: Posedge\\\\1: Negedge"]
    #[inline(always)]
    pub fn cap0_edge(&self) -> Cap0EdgeR {
        Cap0EdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents edge of last capture trigger on channel1.\\\\0: Posedge\\\\1: Negedge"]
    #[inline(always)]
    pub fn cap1_edge(&self) -> Cap1EdgeR {
        Cap1EdgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents edge of last capture trigger on channel2.\\\\0: Posedge\\\\1: Negedge"]
    #[inline(always)]
    pub fn cap2_edge(&self) -> Cap2EdgeR {
        Cap2EdgeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Last capture trigger edge information register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapStatusSpec;
impl crate::RegisterSpec for CapStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_status::R`](R) reader structure"]
impl crate::Readable for CapStatusSpec {}
#[doc = "`reset()` method sets CAP_STATUS to value 0"]
impl crate::Resettable for CapStatusSpec {}
