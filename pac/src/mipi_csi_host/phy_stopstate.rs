#[doc = "Register `PHY_STOPSTATE` reader"]
pub type R = crate::R<PhyStopstateSpec>;
#[doc = "Field `PHY_STOPSTATEDATA_0` reader - NA"]
pub type PhyStopstatedata0R = crate::BitReader;
#[doc = "Field `PHY_STOPSTATEDATA_1` reader - NA"]
pub type PhyStopstatedata1R = crate::BitReader;
#[doc = "Field `PHY_STOPSTATECLK` reader - NA"]
pub type PhyStopstateclkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_stopstatedata_0(&self) -> PhyStopstatedata0R {
        PhyStopstatedata0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_stopstatedata_1(&self) -> PhyStopstatedata1R {
        PhyStopstatedata1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn phy_stopstateclk(&self) -> PhyStopstateclkR {
        PhyStopstateclkR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_stopstate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyStopstateSpec;
impl crate::RegisterSpec for PhyStopstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_stopstate::R`](R) reader structure"]
impl crate::Readable for PhyStopstateSpec {}
#[doc = "`reset()` method sets PHY_STOPSTATE to value 0"]
impl crate::Resettable for PhyStopstateSpec {}
