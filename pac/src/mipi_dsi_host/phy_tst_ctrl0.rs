#[doc = "Register `PHY_TST_CTRL0` reader"]
pub type R = crate::R<PhyTstCtrl0Spec>;
#[doc = "Register `PHY_TST_CTRL0` writer"]
pub type W = crate::W<PhyTstCtrl0Spec>;
#[doc = "Field `PHY_TESTCLR` reader - NA"]
pub type PhyTestclrR = crate::BitReader;
#[doc = "Field `PHY_TESTCLR` writer - NA"]
pub type PhyTestclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TESTCLK` reader - NA"]
pub type PhyTestclkR = crate::BitReader;
#[doc = "Field `PHY_TESTCLK` writer - NA"]
pub type PhyTestclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_testclr(&self) -> PhyTestclrR {
        PhyTestclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_testclk(&self) -> PhyTestclkR {
        PhyTestclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_testclr(&mut self) -> PhyTestclrW<'_, PhyTstCtrl0Spec> {
        PhyTestclrW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_testclk(&mut self) -> PhyTestclkW<'_, PhyTstCtrl0Spec> {
        PhyTestclkW::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_tst_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_tst_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTstCtrl0Spec;
impl crate::RegisterSpec for PhyTstCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tst_ctrl0::R`](R) reader structure"]
impl crate::Readable for PhyTstCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_tst_ctrl0::W`](W) writer structure"]
impl crate::Writable for PhyTstCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_TST_CTRL0 to value 0x01"]
impl crate::Resettable for PhyTstCtrl0Spec {
    const RESET_VALUE: u32 = 0x01;
}
