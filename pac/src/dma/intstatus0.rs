#[doc = "Register `INTSTATUS0` reader"]
pub type R = crate::R<Intstatus0Spec>;
#[doc = "Field `CH1_INTSTAT` reader - NA"]
pub type Ch1IntstatR = crate::BitReader;
#[doc = "Field `CH2_INTSTAT` reader - NA"]
pub type Ch2IntstatR = crate::BitReader;
#[doc = "Field `CH3_INTSTAT` reader - NA"]
pub type Ch3IntstatR = crate::BitReader;
#[doc = "Field `CH4_INTSTAT` reader - NA"]
pub type Ch4IntstatR = crate::BitReader;
#[doc = "Field `COMMONREG_INTSTAT` reader - NA"]
pub type CommonregIntstatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_intstat(&self) -> Ch1IntstatR {
        Ch1IntstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch2_intstat(&self) -> Ch2IntstatR {
        Ch2IntstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch3_intstat(&self) -> Ch3IntstatR {
        Ch3IntstatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch4_intstat(&self) -> Ch4IntstatR {
        Ch4IntstatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn commonreg_intstat(&self) -> CommonregIntstatR {
        CommonregIntstatR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intstatus0Spec;
impl crate::RegisterSpec for Intstatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus0::R`](R) reader structure"]
impl crate::Readable for Intstatus0Spec {}
#[doc = "`reset()` method sets INTSTATUS0 to value 0"]
impl crate::Resettable for Intstatus0Spec {}
