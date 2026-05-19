#[doc = "Register `PHY_CAL` reader"]
pub type R = crate::R<PhyCalSpec>;
#[doc = "Register `PHY_CAL` writer"]
pub type W = crate::W<PhyCalSpec>;
#[doc = "Field `TXSKEWCALHS` reader - NA"]
pub type TxskewcalhsR = crate::BitReader;
#[doc = "Field `TXSKEWCALHS` writer - NA"]
pub type TxskewcalhsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn txskewcalhs(&self) -> TxskewcalhsR {
        TxskewcalhsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn txskewcalhs(&mut self) -> TxskewcalhsW<'_, PhyCalSpec> {
        TxskewcalhsW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCalSpec;
impl crate::RegisterSpec for PhyCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cal::R`](R) reader structure"]
impl crate::Readable for PhyCalSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_cal::W`](W) writer structure"]
impl crate::Writable for PhyCalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CAL to value 0"]
impl crate::Resettable for PhyCalSpec {}
