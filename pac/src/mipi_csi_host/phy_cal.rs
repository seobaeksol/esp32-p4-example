#[doc = "Register `PHY_CAL` reader"]
pub type R = crate::R<PhyCalSpec>;
#[doc = "Field `RXSKEWCALHS` reader - NA"]
pub type RxskewcalhsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn rxskewcalhs(&self) -> RxskewcalhsR {
        RxskewcalhsR::new((self.bits & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCalSpec;
impl crate::RegisterSpec for PhyCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cal::R`](R) reader structure"]
impl crate::Readable for PhyCalSpec {}
#[doc = "`reset()` method sets PHY_CAL to value 0"]
impl crate::Resettable for PhyCalSpec {}
